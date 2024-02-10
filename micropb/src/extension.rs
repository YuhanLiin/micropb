use core::any::TypeId;

#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::Tag;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExtensionId(pub usize);

pub trait ExtensionField: 'static {
    const FIELD_NUM: u32;
    type MESSAGE: 'static;
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum RegistryError {
    IdNotFound,
    BadField,
}

pub trait ExtensionRegistry {
    fn alloc_ext(&mut self, msg_type: &TypeId) -> Option<ExtensionId>;

    fn get_field<F: ExtensionField>(&self, id: ExtensionId) -> Result<Option<&F>, RegistryError>
    where
        Self: Sized;

    fn get_field_mut<F: ExtensionField>(
        &mut self,
        id: ExtensionId,
    ) -> Result<Option<&mut F>, RegistryError>
    where
        Self: Sized;

    fn add_field<F: ExtensionField>(&mut self, id: ExtensionId) -> Result<&mut F, RegistryError>
    where
        Self: Sized;

    fn clear_field<F: ExtensionField>(&mut self, id: ExtensionId) -> Result<(), RegistryError>
    where
        Self: Sized;

    fn dealloc_ext(&mut self, id: ExtensionId) -> Result<(), RegistryError>;

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
    (@base $Name:ident, $($Msg:ident [ $len:expr ] => { $($extname:ident : $Ext:path),+ })+) => {
        paste::paste! {
            $(
                #[allow(non_snake_case)]
                mod [<mod_ $Msg>] {
                    use super::*;

                    #[derive(Debug, Default)]
                    pub(crate) struct Extension {
                        $(pub(crate) $extname: $Ext, pub(crate) [<has_ $extname>]: bool),+
                    }
                }
            )+

            #[derive(Debug, Default)]
            #[allow(non_snake_case)]
            struct $Name {
                $([<alloc_ $Msg>]: [Option<[<mod_ $Msg>]::Extension>; $len],)+
            }
        }

        impl $crate::extension::ExtensionRegistry for $Name {
            fn alloc_ext(&mut self, msg_type: &core::any::TypeId) -> Option<$crate::extension::ExtensionId> {
                let mut id = 0;
                paste::paste! {
                    $(if msg_type == &core::any::TypeId::of::<$Msg>() {
                        for (i, slot) in self.[<alloc_ $Msg>].iter_mut().enumerate() {
                            if slot.is_none() {
                                *slot = Some(Default::default());
                                return Some($crate::extension::ExtensionId(id + i));
                            }
                        }
                        return None;
                    }
                    id += self.[<alloc_ $Msg>].len();)+
                }
                None
            }

            fn get_field<F: $crate::extension::ExtensionField>(&self, id: $crate::extension::ExtensionId) -> Result<Option<&F>, $crate::extension::RegistryError>
            where
                Self: Sized
            {
                use $crate::extension::ExtensionField;
                use core::any::{TypeId, Any};
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        if TypeId::of::<F::MESSAGE>() == TypeId::of::<$Msg>() {
                            let ext = self.[<alloc_ $Msg>][id].as_ref().ok_or($crate::extension::RegistryError::IdNotFound)?;
                            let field: Option<&dyn Any> = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => ext.[<has_$extname>].then_some(&ext.$extname),)+
                                _ => return Err($crate::extension::RegistryError::BadField),
                            };
                            return field.map(|f| f.downcast_ref::<F>().ok_or($crate::extension::RegistryError::BadField)).transpose();
                        } else {
                            return Err($crate::extension::RegistryError::BadField);
                        }
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Err($crate::extension::RegistryError::IdNotFound)
            }

            fn get_field_mut<F: $crate::extension::ExtensionField>(&mut self, id: $crate::extension::ExtensionId) -> Result<Option<&mut F>, $crate::extension::RegistryError>
            where
                Self: Sized
            {
                use $crate::extension::ExtensionField;
                use core::any::{TypeId, Any};
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        if TypeId::of::<F::MESSAGE>() == TypeId::of::<$Msg>() {
                            let ext = self.[<alloc_ $Msg>][id].as_mut().ok_or($crate::extension::RegistryError::IdNotFound)?;
                            let field: Option<&mut dyn Any> = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => ext.[<has_$extname>].then_some(&mut ext.$extname),)+
                                _ => return Err($crate::extension::RegistryError::BadField),
                            };
                            return field.map(|f| f.downcast_mut::<F>().ok_or($crate::extension::RegistryError::BadField)).transpose();
                        } else {
                            return Err($crate::extension::RegistryError::BadField);
                        }
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Err($crate::extension::RegistryError::IdNotFound)
            }

            fn add_field<F: $crate::extension::ExtensionField>(&mut self, id: $crate::extension::ExtensionId) -> Result<&mut F, $crate::extension::RegistryError>
            where
                Self: Sized
            {
                use $crate::extension::ExtensionField;
                use core::any::{TypeId, Any};
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        if TypeId::of::<F::MESSAGE>() == TypeId::of::<$Msg>() {
                            let ext = self.[<alloc_ $Msg>][id].as_mut().ok_or($crate::extension::RegistryError::IdNotFound)?;
                            let (field, has): (&mut dyn Any, &mut bool) = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => (&mut ext.$extname, &mut ext.[<has_$extname>]),)+
                                _ => return Err($crate::extension::RegistryError::BadField),
                            };
                            let field = field.downcast_mut::<F>().ok_or($crate::extension::RegistryError::BadField)?;
                            *has = true;
                            return Ok(field);
                        } else {
                            return Err($crate::extension::RegistryError::BadField);
                        }
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Err($crate::extension::RegistryError::IdNotFound)
            }

            fn clear_field<F: $crate::extension::ExtensionField>(&mut self, id: $crate::extension::ExtensionId) -> Result<(), $crate::extension::RegistryError>
            where
                Self: Sized
            {
                use core::any::{Any, TypeId};
                use $crate::extension::ExtensionField;
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        if TypeId::of::<F::MESSAGE>() == TypeId::of::<$Msg>() {
                            let ext = self.[<alloc_ $Msg>][id].as_mut().ok_or($crate::extension::RegistryError::IdNotFound)?;
                            let (field, has): (&mut dyn Any, &mut bool) = match F::FIELD_NUM {
                                $($Ext::FIELD_NUM => (&mut ext.$extname, &mut ext.[<has_$extname>]),)+
                                _ => return Err($crate::extension::RegistryError::BadField),
                            };
                            // Check if the field type is correct before clearing it
                            field.downcast_mut::<F>().ok_or($crate::extension::RegistryError::BadField)?;
                            *has = false;
                            return Ok(());
                        } else {
                            return Err($crate::extension::RegistryError::BadField);
                        }
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Err($crate::extension::RegistryError::IdNotFound)
            }

            #[must_use]
            fn dealloc_ext(&mut self, id: $crate::extension::ExtensionId) -> Result<(), $crate::extension::RegistryError> {
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        self.[<alloc_ $Msg>][id] = None;
                        return Ok(());
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Err($crate::extension::RegistryError::IdNotFound)
            }

            fn reset(&mut self) {
                *self = Default::default();
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
                use $crate::field::FieldDecode;
                use $crate::extension::ExtensionField;
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        let slot = &mut self.[<alloc_ $Msg>][id];
                        if slot.is_none() { return Ok(false); }
                        // Write `Some` into the slot temporarily so it doesn't get overwritten
                        let mut ext = core::mem::replace(slot, Some(Default::default())).unwrap();
                        let res = match tag.field_num() {
                            $($Ext::FIELD_NUM => {
                                let res = ext.$extname.decode_field(tag, decoder, Some(self));
                                if res.is_ok() {
                                    ext.[<has_ $extname>] = true;
                                }
                                res.map(|_| true)
                            })+
                            _ => Ok(false)
                        };
                        self.[<alloc_ $Msg>][id] = Some(ext);
                        return res;
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Ok(false)
            }
        }
    };

    (@encode $Name:ident, $($Msg:ident => { $($extname:ident : $Ext:ty),+ })+) => {
        impl $crate::extension::ExtensionRegistrySizeof for $Name {
            fn compute_ext_size(&self, id: $crate::extension::ExtensionId) -> Option<usize> {
                use $crate::field::FieldEncode;
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        let ext = self.[<alloc_ $Msg>][id].as_ref()?;
                        let mut size = 0;
                        $(if ext.[<has_ $extname>] {
                            size += ext.$extname.compute_field_size(Some(self));
                        })+
                        return Some(size);
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                None
            }
        }

        impl<W: $crate::PbWrite> $crate::extension::ExtensionRegistryEncode<W> for $Name {
            fn encode_ext(&self, id: $crate::extension::ExtensionId, encoder: &mut $crate::PbEncoder<W>) -> Result<bool, W::Error> {
                use $crate::field::FieldEncode;
                let mut id = id.0;
                paste::paste! {
                    $(if id < self.[<alloc_ $Msg>].len() {
                        let Some(ext) = self.[<alloc_ $Msg>][id].as_ref() else { return Ok(false) };
                        $(if ext.[<has_ $extname>] {
                            ext.$extname.encode_field(encoder, Some(self))?;
                        })+
                        return Ok(true);
                    }
                    id -= self.[<alloc_ $Msg>].len();)+
                }
                Ok(false)
            }
        }
    };

    ($Name:ident, $($Msg:ident [ $len:expr ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        $crate::map_extension_registry!(@base $Name, $($Msg[$len] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_decode_only {
    ($Name:ident, $($Msg:ident [ $len:expr ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        $crate::map_extension_registry!(@base $Name, $($Msg[$len] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@decode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}

#[macro_export]
macro_rules! map_extension_registry_encode_only {
    ($Name:ident, $($Msg:ident [ $len:expr ] => { $($extname:ident : $Ext:path),+ $(,)? })+) => {
        $crate::map_extension_registry!(@base $Name, $($Msg[$len] => { $($extname: $Ext),+ })+);
        $crate::map_extension_registry!(@encode $Name, $($Msg => { $($extname: $Ext),+ })+);
    }
}
