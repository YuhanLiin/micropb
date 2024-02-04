use core::{any::TypeId, num::NonZeroU32};

#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::Tag;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExtensionId(NonZeroU32);

impl ExtensionId {
    pub fn new(u: NonZeroU32) -> Self {
        Self(u)
    }
}

pub trait ExtensionField: Default + 'static {
    const FIELD_NUM: u32;
    type MESSAGE: 'static;
}

pub trait ExtensionRegistry {
    fn alloc_ext(&mut self, msg_type: &TypeId) -> Option<ExtensionId>;

    fn get_field<F: ExtensionField>(&self, id: ExtensionId) -> Option<&F>
    where
        Self: Sized;

    fn get_field_mut<F: ExtensionField>(&mut self, id: ExtensionId) -> Option<&mut F>
    where
        Self: Sized;

    #[must_use]
    fn remove(&mut self, id: ExtensionId) -> bool;

    fn reset(&mut self);
}

#[cfg(feature = "decode")]
pub trait ExtensionRegistryDecode<R: PbRead>: ExtensionRegistry {
    fn decode_ext_field(
        &mut self,
        id: ExtensionId,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
    ) -> Result<bool, DecodeError<R::Error>>;
}

#[cfg(feature = "encode")]
pub trait ExtensionRegistrySizeof: ExtensionRegistry {
    fn compute_ext_size(&self, id: ExtensionId) -> Option<usize>;
}

#[cfg(feature = "encode")]
/// Allows `&dyn ExtensionRegistryEncode` to be downcasted into `&dyn ExtensionRegistrySizeof`. Do
/// not implement.
pub trait DynExtensionRegistrySizeof: ExtensionRegistrySizeof {
    fn as_dyn(&self) -> &dyn ExtensionRegistrySizeof;
}

impl<T: ExtensionRegistrySizeof> DynExtensionRegistrySizeof for T {
    fn as_dyn(&self) -> &dyn ExtensionRegistrySizeof {
        self
    }
}

#[cfg(feature = "encode")]
pub trait ExtensionRegistryEncode<W: PbWrite>: DynExtensionRegistrySizeof {
    fn encode_ext(&self, id: ExtensionId, encoder: &mut PbEncoder<W>) -> Result<bool, W::Error>;
}

#[macro_export]
macro_rules! map_extension_registry {
    (@base $Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ })+) => {
        paste::paste! {
            $(
                #[allow(non_snake_case)]
                mod [<mod_ $Msg>] {
                    use super::*;

                    #[derive(Debug, Default)]
                    pub(crate) struct Extension {
                        $(pub(crate) $extname: $Ext),+
                    }
                }
            )+

            #[derive(Debug, Default)]
            #[allow(non_snake_case)]
            struct $Name {
                id_counter: u32,
                $([<map_ $Msg>]: $Map<ExtensionId, [<mod_ $Msg>]::Extension $(, $N)?>,)+
            }
        }

        impl ExtensionRegistry for $Name {
            fn alloc_ext(&mut self, msg_type: &core::any::TypeId) -> Option<$crate::extension::ExtensionId> {
                use $crate::PbMap;
                paste::paste! {
                    $(if msg_type == &core::any::TypeId::of::<$Msg>() {
                        // Return None on overflow
                        self.id_counter = self.id_counter.checked_add(1)?;
                        let id = $crate::extension::ExtensionId::new(self.id_counter.try_into().unwrap());
                        return self.[<map_ $Msg>].pb_insert(id, Default::default()).ok().map(|_| id);
                    })+
                }
                None
            }

            fn get_field<F: $crate::extension::ExtensionField>(&self, id: $crate::extension::ExtensionId) -> Option<&F>
            where
                Self: Sized
            {
                use $crate::PbMap;
                use core::any::{TypeId, Any};
                paste::paste! {
                    match TypeId::of::<F::MESSAGE>() {
                        $(t if t == TypeId::of::<$Msg>() => {
                            let ext = self.[<map_ $Msg>].pb_get(&id)?;
                            let field: &dyn Any = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => &ext.$extname,)+
                                _ => return None,
                            };
                            field.downcast_ref::<F>()
                        })+
                        _ => None
                    }
                }
            }

            fn get_field_mut<F: $crate::extension::ExtensionField>(&mut self, id: $crate::extension::ExtensionId) -> Option<&mut F>
            where
                Self: Sized
            {
                use $crate::PbMap;
                use core::any::{TypeId, Any};
                paste::paste! {
                    match TypeId::of::<F::MESSAGE>() {
                        $(t if t == TypeId::of::<$Msg>() => {
                            let ext = self.[<map_ $Msg>].pb_get_mut(&id)?;
                            let field: &mut dyn Any = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => &mut ext.$extname,)+
                                _ => return None,
                            };
                            return field.downcast_mut::<F>();
                        })+
                        _ => None
                    }
                }
            }

            #[must_use]
            fn remove(&mut self, id: $crate::extension::ExtensionId) -> bool {
                use $crate::PbMap;
                paste::paste! {
                    $(if self.[<map_ $Msg>].pb_remove(&id).is_some() {
                        return true;
                    })+
                }
                false
            }

            fn reset(&mut self) {
                use $crate::PbMap;
                paste::paste! {
                    $(self.[<map_ $Msg>].pb_clear();)+
                }
                self.id_counter = 0;
            }
        }
    };

    (@decode $Name:ident, $($Msg:ident => { $($extname:ident : $Ext:ty),+ })+) => {
        impl<R: $crate::PbRead> ExtensionRegistryDecode<R> for $Name {
            fn decode_ext_field(
                &mut self,
                id: $crate::extension::ExtensionId,
                tag: $crate::Tag,
                decoder: &mut $crate::PbDecoder<R>,
            ) -> Result<bool, $crate::DecodeError<R::Error>>
            {
                use $crate::{callback::DecodeCallback, PbMap};
                paste::paste! {
                    $(if let Some(mut ext) = self.[<map_ $Msg>].pb_remove(&id) {
                        let mut written = true;
                        match tag.field_num() {
                            $($Ext::FIELD_NUM => ext.$extname.decode_field(tag, decoder, Some(self))?,)+
                            _ => written = false,
                        }
                        self.[<map_ $Msg>].pb_insert(id, ext).unwrap();
                        return Ok(written);
                    })+
                }
                Ok(false)
            }
        }
    };

    (@encode $Name:ident, $($Msg:ident => { $($extname:ident : $Ext:ty),+ })+) => {
        impl ExtensionRegistrySizeof for $Name {
            fn compute_ext_size(&self, id: $crate::extension::ExtensionId) -> Option<usize> {
                use $crate::{callback::EncodeCallback, PbMap};
                paste::paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        let mut size = 0;
                        $(size += ext.$extname.compute_field_size(Some(self));)+
                        return Some(size);
                    })+
                }
                None
            }
        }

        impl<W: $crate::PbWrite> ExtensionRegistryEncode<W> for $Name {
            fn encode_ext(&self, id: $crate::extension::ExtensionId, encoder: &mut $crate::PbEncoder<W>) -> Result<bool, W::Error> {
                use $crate::{callback::EncodeCallback, PbMap};
                paste::paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        $(ext.$extname.encode_field(encoder, Some(self))?;)+
                        return Ok(true);
                    })+
                }
                Ok(false)
            }
        }
    };

    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
        map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_decode_only {
    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_encode_only {
    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[cfg(test)]
mod tests {
    use heapless::FnvIndexMap;

    use super::*;

    use core::any::Any;
    use std::collections::HashMap;

    use crate::{
        callback::{DecodeCallback, EncodeCallback},
        size::{sizeof_tag, sizeof_varint32},
    };

    #[derive(Debug, Default)]
    struct NumMsg {}

    #[derive(Debug, Default)]
    struct RecursiveMsg {
        ext: Option<ExtensionId>,
    }

    #[derive(Debug, Default)]
    struct NumExtension1(u32);

    impl ExtensionField for NumExtension1 {
        const FIELD_NUM: u32 = 1;
        type MESSAGE = NumMsg;
    }

    impl DecodeCallback for NumExtension1 {
        fn decode_field<R: PbRead>(
            &mut self,
            _tag: Tag,
            decoder: &mut PbDecoder<R>,
            _registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
        ) -> Result<(), DecodeError<R::Error>> {
            self.0 = decoder.decode_varint32()?;
            Ok(())
        }
    }

    impl EncodeCallback for NumExtension1 {
        fn encode_field<W: PbWrite>(
            &self,
            encoder: &mut PbEncoder<W>,
            _registry: Option<&dyn ExtensionRegistryEncode<W>>,
        ) -> Result<(), W::Error> {
            encoder.encode_tag(Tag::from_parts(1, 0))?;
            encoder.encode_varint32(self.0)?;
            Ok(())
        }

        fn compute_field_size(&self, _registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
            sizeof_tag(Tag::from_parts(1, 0)) + sizeof_varint32(self.0)
        }
    }

    // Extension for RecursiveMsg instead of NumMsg
    #[derive(Debug, Default)]
    struct NumExtension2(u32);

    impl ExtensionField for NumExtension2 {
        const FIELD_NUM: u32 = 1;
        type MESSAGE = RecursiveMsg;
    }

    impl DecodeCallback for NumExtension2 {
        fn decode_field<R: PbRead>(
            &mut self,
            _tag: Tag,
            decoder: &mut PbDecoder<R>,
            _registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
        ) -> Result<(), DecodeError<R::Error>> {
            self.0 = decoder.decode_varint32()?;
            Ok(())
        }
    }

    impl EncodeCallback for NumExtension2 {
        fn encode_field<W: PbWrite>(
            &self,
            encoder: &mut PbEncoder<W>,
            _registry: Option<&dyn ExtensionRegistryEncode<W>>,
        ) -> Result<(), W::Error> {
            encoder.encode_tag(Tag::from_parts(1, 0))?;
            encoder.encode_varint32(self.0)?;
            Ok(())
        }

        fn compute_field_size(&self, _registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
            sizeof_tag(Tag::from_parts(1, 0)) + sizeof_varint32(self.0)
        }
    }

    #[derive(Debug, Default)]
    struct RecursiveExtension(RecursiveMsg);

    impl ExtensionField for RecursiveExtension {
        const FIELD_NUM: u32 = 2;
        type MESSAGE = RecursiveMsg;
    }

    impl DecodeCallback for RecursiveExtension {
        fn decode_field<R: PbRead>(
            &mut self,
            _tag: Tag,
            decoder: &mut PbDecoder<R>,
            registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
        ) -> Result<(), DecodeError<R::Error>> {
            let registry = registry.unwrap();
            let id = registry.alloc_ext(&self.0.type_id()).unwrap();
            self.0.ext = Some(id);
            loop {
                let idx = decoder.bytes_read();
                let tag = match decoder.decode_tag() {
                    Ok(tag) => tag,
                    Err(DecodeError::UnexpectedEof) if decoder.bytes_read() == idx => return Ok(()),
                    Err(e) => return Err(e),
                };
                registry.decode_ext_field(id, tag, decoder)?;
            }
        }
    }

    impl EncodeCallback for RecursiveExtension {
        fn encode_field<W: PbWrite>(
            &self,
            encoder: &mut PbEncoder<W>,
            registry: Option<&dyn ExtensionRegistryEncode<W>>,
        ) -> Result<(), W::Error> {
            let registry = registry.unwrap();
            if let Some(id) = self.0.ext {
                encoder.encode_tag(Tag::from_parts(2, 0))?;
                registry.encode_ext(id, encoder)?;
            }
            Ok(())
        }

        fn compute_field_size(&self, registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
            let registry = registry.unwrap();
            self.0
                .ext
                .map(|id| {
                    sizeof_tag(Tag::from_parts(2, 0)) + registry.compute_ext_size(id).unwrap()
                })
                .unwrap_or(0)
        }
    }

    map_extension_registry!(
        TestRegistry,
        NumMsg[HashMap] => {
            num: NumExtension1,
        }
        RecursiveMsg[FnvIndexMap<8>] => {
            num: NumExtension2,
            msg: RecursiveExtension
        }
    );

    #[test]
    fn map_macro() {
        let mut registry = TestRegistry::default();
        let mut decoder = PbDecoder::new([0x57].as_slice());
        assert_eq!(registry.alloc_ext(&TypeId::of::<u32>()), None);
        let id = registry.alloc_ext(&TypeId::of::<NumMsg>()).unwrap();

        // unknown field number, ignored
        assert!(!registry
            .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
            .unwrap());
        assert!(registry
            .decode_ext_field(id, Tag::from_parts(1, 0), &mut decoder)
            .unwrap());
        assert_eq!(registry.get_field::<NumExtension1>(id).unwrap().0, 0x57);

        let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
        registry.get_field_mut::<NumExtension1>(id).unwrap().0 = 0x69;
        assert!(registry.encode_ext(id, &mut encoder).unwrap());
        // encoding also outputs the tag
        assert_eq!(encoder.into_inner(), &[0x08, 0x69]);
        assert_eq!(registry.compute_ext_size(id).unwrap(), 2);

        assert!(registry.remove(id));
        assert!(!registry.remove(id));
        assert!(registry.get_field_mut::<NumExtension1>(id).is_none());
    }

    #[test]
    fn map_macro_recursive() {
        let mut registry = TestRegistry::default();
        // 0x08 is tag for field 1, 0x10 is tag for field 2
        let mut decoder =
            PbDecoder::new([0x08, 0x34, 0x10, 0x08, 0x12, 0x10, 0x08, 0x55].as_slice());
        let id = registry.alloc_ext(&TypeId::of::<RecursiveMsg>()).unwrap();

        // Populate the RecursiveMsg field of the extension
        assert!(registry
            .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
            .unwrap());
        let id1 = registry
            .get_field::<RecursiveExtension>(id)
            .unwrap()
            .0
            .ext
            .unwrap();
        assert_eq!(registry.get_field::<NumExtension2>(id1).unwrap().0, 0x34);
        let id2 = registry
            .get_field::<RecursiveExtension>(id1)
            .unwrap()
            .0
            .ext
            .unwrap();
        assert_eq!(registry.get_field::<NumExtension2>(id2).unwrap().0, 0x12);
        let id3 = registry
            .get_field::<RecursiveExtension>(id2)
            .unwrap()
            .0
            .ext
            .unwrap();
        assert_eq!(registry.get_field::<NumExtension2>(id3).unwrap().0, 0x55);
        assert_eq!(
            registry.get_field::<RecursiveExtension>(id3).unwrap().0.ext,
            None
        );

        let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
        registry.get_field_mut::<NumExtension2>(id).unwrap().0 = 0x02;
        registry
            .get_field_mut::<RecursiveExtension>(id1)
            .unwrap()
            .0
            .ext = None;
        assert!(registry.encode_ext(id, &mut encoder).unwrap());
        let out = encoder.into_inner();
        assert_eq!(out, &[0x08, 0x02, 0x10, 0x08, 0x34]);
        assert_eq!(registry.compute_ext_size(id).unwrap(), out.len());
    }
}
