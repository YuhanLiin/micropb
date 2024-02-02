use core::{
    any::{Any, TypeId},
    num::NonZeroU32,
};

#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::Tag;

use paste::paste;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExtensionId(NonZeroU32);

impl ExtensionId {
    pub fn new(u: NonZeroU32) -> Self {
        Self(u)
    }
}

pub trait ExtensionField: Default + 'static {
    const FIELD_NUM: u32;
}

pub trait ExtensionRegistry {
    fn alloc_ext(&mut self, msg_type: &TypeId) -> Option<ExtensionId>;

    fn get_any<F: ExtensionField>(&self, id: ExtensionId) -> &dyn Any
    where
        Self: Sized;

    fn get_any_mut<F: ExtensionField>(&mut self, id: ExtensionId) -> &mut dyn Any
    where
        Self: Sized;

    fn get_field<F: ExtensionField>(&self, id: ExtensionId) -> &F
    where
        Self: Sized,
    {
        self.get_any::<F>(id).downcast_ref().unwrap()
    }

    fn get_field_mut<F: ExtensionField>(&mut self, id: ExtensionId) -> &mut F
    where
        Self: Sized,
    {
        self.get_any_mut::<F>(id).downcast_mut().unwrap()
    }
}

#[cfg(feature = "decode")]
pub trait ExtensionRegistryDecode<R: PbRead>: ExtensionRegistry {
    fn decode_ext_field(
        &mut self,
        id: ExtensionId,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>>;
}

#[cfg(feature = "encode")]
pub trait ExtensionRegistrySizeof: ExtensionRegistry {
    fn compute_ext_size(&self, id: ExtensionId) -> usize;
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
    fn encode_ext(&self, id: ExtensionId, encoder: &mut PbEncoder<W>) -> Result<(), W::Error>;
}

macro_rules! extension_registry {
    (@base $Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ })+) => {
        paste! {
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
                paste! {
                    $(if msg_type == &TypeId::of::<$Msg>() {
                        self.id_counter += 1;
                        let id = ExtensionId::new(self.id_counter.try_into().unwrap());
                        return self.[<map_ $Msg>].pb_insert(id, Default::default()).ok().map(|_| id);
                    })+
                }
                None
            }

            fn get_any<F: $crate::extension::ExtensionField>(&self, id: $crate::extension::ExtensionId) -> &dyn core::any::Any
            where
                Self: Sized
            {
                paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        match F::FIELD_NUM {
                            $($Ext::FIELD_NUM => return &ext.$extname,)+
                            _ => unreachable!("unknown extension field")
                        }
                    })+
                }
                unreachable!("extension ID not found")
            }

            fn get_any_mut<F: $crate::extension::ExtensionField>(&mut self, id: $crate::extension::ExtensionId) -> &mut dyn core::any::Any
            where
                Self: Sized
            {
                paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get_mut(&id) {
                        match F::FIELD_NUM {
                            $($Ext::FIELD_NUM => return &mut ext.$extname,)+
                            _ => unreachable!("unknown extension field")
                        }
                    })+
                }
                unreachable!("extension ID not found")
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
            ) -> Result<(), $crate::DecodeError<R::Error>>
            {
                paste! {
                    $(if let Some(mut ext) = self.[<map_ $Msg>].pb_remove(&id) {
                        match tag.field_num() {
                            $($Ext::FIELD_NUM => ext.$extname.decode_field(tag, decoder, Some(self))?,)+
                            _ => {}
                        }
                        self.[<map_ $Msg>].pb_insert(id, ext).unwrap();
                        return Ok(());
                    })+
                }
                unreachable!("extension ID not found")
            }
        }
    };

    (@encode $Name:ident, $($Msg:ident => { $($extname:ident : $Ext:ty),+ })+) => {
        impl ExtensionRegistrySizeof for $Name {
            fn compute_ext_size(&self, id: $crate::extension::ExtensionId) -> usize {
                paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        let mut size = 0;
                        $(size += ext.$extname.compute_field_size(Some(self));)+
                        return size;
                    })+
                }
                unreachable!("extension ID not found")
            }
        }

        impl<W: $crate::PbWrite> ExtensionRegistryEncode<W> for $Name {
            fn encode_ext(&self, id: $crate::extension::ExtensionId, encoder: &mut $crate::PbEncoder<W>) -> Result<(), W::Error> {
                paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        $(ext.$extname.encode_field(encoder, Some(self))?;)+
                        return Ok(());
                    })+
                }
                unreachable!("extension ID not found")
            }
        }
    };
}

macro_rules! map_registry {
    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
        extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[cfg(test)]
mod tests {
    use heapless::FnvIndexMap;

    use super::*;

    use std::collections::HashMap;

    use crate::{
        callback::{DecodeCallback, EncodeCallback},
        size::{sizeof_tag, sizeof_varint32},
        PbMap,
    };

    #[derive(Debug, Default)]
    struct NumMsg {}

    #[derive(Debug, Default)]
    struct RecursiveMsg {
        ext: Option<ExtensionId>,
    }

    #[derive(Debug, Default)]
    struct NumExtension(u32);

    impl ExtensionField for NumExtension {
        const FIELD_NUM: u32 = 1;
    }

    impl DecodeCallback for NumExtension {
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

    impl EncodeCallback for NumExtension {
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
                .map(|id| sizeof_tag(Tag::from_parts(2, 0)) + registry.compute_ext_size(id))
                .unwrap_or(0)
        }
    }

    map_registry!(
        TestRegistry,
        NumMsg[HashMap] => {
            num: NumExtension,
        }
        RecursiveMsg[FnvIndexMap<8>] => {
            num: NumExtension,
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
        registry
            .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
            .unwrap();
        registry
            .decode_ext_field(id, Tag::from_parts(1, 0), &mut decoder)
            .unwrap();
        assert_eq!(registry.get_field::<NumExtension>(id).0, 0x57);

        let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
        registry.get_field_mut::<NumExtension>(id).0 = 0x69;
        registry.encode_ext(id, &mut encoder).unwrap();
        // encoding also outputs the tag
        assert_eq!(encoder.into_inner(), &[0x08, 0x69]);
        assert_eq!(registry.compute_ext_size(id), 2);
    }

    #[test]
    fn map_macro_recursive() {
        let mut registry = TestRegistry::default();
        // 0x08 is tag for field 1, 0x10 is tag for field 2
        let mut decoder =
            PbDecoder::new([0x08, 0x34, 0x10, 0x08, 0x12, 0x10, 0x08, 0x55].as_slice());
        let id = registry.alloc_ext(&TypeId::of::<RecursiveMsg>()).unwrap();

        // Populate the RecursiveMsg field of the extension
        registry
            .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
            .unwrap();
        let id1 = registry.get_field::<RecursiveExtension>(id).0.ext.unwrap();
        assert_eq!(registry.get_field::<NumExtension>(id1).0, 0x34);
        let id2 = registry.get_field::<RecursiveExtension>(id1).0.ext.unwrap();
        assert_eq!(registry.get_field::<NumExtension>(id2).0, 0x12);
        let id3 = registry.get_field::<RecursiveExtension>(id2).0.ext.unwrap();
        assert_eq!(registry.get_field::<NumExtension>(id3).0, 0x55);
        assert_eq!(registry.get_field::<RecursiveExtension>(id3).0.ext, None);

        let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
        registry.get_field_mut::<NumExtension>(id).0 = 0x02;
        registry.get_field_mut::<RecursiveExtension>(id1).0.ext = None;
        registry.encode_ext(id, &mut encoder).unwrap();
        let out = encoder.into_inner();
        assert_eq!(out, &[0x08, 0x02, 0x10, 0x08, 0x34]);
        assert_eq!(registry.compute_ext_size(id), out.len());
    }
}
