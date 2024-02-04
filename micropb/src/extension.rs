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

pub trait ExtensionField: 'static {
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
/// Allows `&dyn ExtensionRegistryEncode` to be downcasted into `&dyn ExtensionRegistrySizeof`.
///
/// This trait is automatically implemented for all applicable types.
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
                $([<map_ $Msg>]: $Map<$crate::extension::ExtensionId, [<mod_ $Msg>]::Extension $(, $N)?>,)+
            }
        }

        impl $crate::extension::ExtensionRegistry for $Name {
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
                use $crate::extension::ExtensionField;
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
                use $crate::extension::ExtensionField;
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
        impl<R: $crate::PbRead> $crate::extension::ExtensionRegistryDecode<R> for $Name {
            fn decode_ext_field(
                &mut self,
                id: $crate::extension::ExtensionId,
                tag: $crate::Tag,
                decoder: &mut $crate::PbDecoder<R>,
            ) -> Result<bool, $crate::DecodeError<R::Error>>
            {
                use $crate::{field::FieldDecode, PbMap};
                use $crate::extension::ExtensionField;
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
        impl $crate::extension::ExtensionRegistrySizeof for $Name {
            fn compute_ext_size(&self, id: $crate::extension::ExtensionId) -> Option<usize> {
                use $crate::{field::FieldEncode, PbMap};
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

        impl<W: $crate::PbWrite> $crate::extension::ExtensionRegistryEncode<W> for $Name {
            fn encode_ext(&self, id: $crate::extension::ExtensionId, encoder: &mut $crate::PbEncoder<W>) -> Result<bool, W::Error> {
                use $crate::{field::FieldEncode, PbMap};
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
        $crate::map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_decode_only {
    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        $crate::map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_encode_only {
    ($Name:ident, $($Msg:ident [ $Map:ident $(<$N:literal>)? ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        $crate::map_extension_registry!(@base $Name, $($Msg[$Map $(<$N>)?] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}
