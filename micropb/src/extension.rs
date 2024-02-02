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
pub trait ExtensionRegistryEncode<W: PbWrite>: ExtensionRegistry {
    fn compute_ext_size(&self, id: ExtensionId) -> usize;

    fn encode_ext(&self, id: ExtensionId, encoder: &mut PbEncoder<W>) -> Result<(), W::Error>;
}

macro_rules! extension_registry {
    (@base $Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ })+) => {
        paste! {
            $(
                #[allow(non_snake_case)]
                mod [<mod_ $Msg>] {
                    use super::*;

                    #[derive(Default)]
                    pub(crate) struct Extension {
                        $(pub(crate) $extname: $Ext),+
                    }
                }
            )+

            #[derive(Default)]
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
        impl<W: $crate::PbWrite> ExtensionRegistryEncode<W> for $Name {
            fn compute_ext_size(&self, id: $crate::extension::ExtensionId) -> usize {
                paste! {
                    $(if let Some(ext) = self.[<map_ $Msg>].pb_get(&id) {
                        let mut size = 0;
                        $(size += ext.$extname.compute_field_size();)+
                        return size;
                    })+
                }
                unreachable!("extension ID not found")
            }

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
    use super::*;

    use std::collections::HashMap;

    use crate::{
        callback::{DecodeCallback, EncodeCallback},
        PbMap,
    };

    struct NumMsg {
        num: Option<ExtensionId>,
    }

    #[derive(Default)]
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
            self.0 = decoder.decode_fixed32()?;
            Ok(())
        }
    }
    impl EncodeCallback for NumExtension {
        fn encode_field<W: PbWrite>(
            &self,
            encoder: &mut PbEncoder<W>,
            _registry: Option<&dyn ExtensionRegistryEncode<W>>,
        ) -> Result<(), W::Error> {
            encoder.encode_fixed32(self.0)
        }

        fn compute_field_size(&self) -> usize {
            4
        }
    }

    map_registry!(
        TestRegistry,
        NumMsg[HashMap] => {
            num: NumExtension,
        }
    );
}
