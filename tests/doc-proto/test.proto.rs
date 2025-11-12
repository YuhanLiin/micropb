/// Inner types for `Msg`
pub mod Msg_ {
    /// Inner message type nested inside Msg
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct Inner {
        /// This field belongs to the Inner message
        pub r#num: i32,
    }
    impl Inner {
        /// Return a reference to `num`
        #[inline]
        pub fn r#num(&self) -> &i32 {
            &self.r#num
        }
        /// Return a mutable reference to `num`
        #[inline]
        pub fn mut_num(&mut self) -> &mut i32 {
            &mut self.r#num
        }
        /// Set the value of `num`
        #[inline]
        pub fn set_num(&mut self, value: i32) -> &mut Self {
            self.r#num = value.into();
            self
        }
        /// Builder method that sets the value of `num`. Useful for initializing the message.
        #[inline]
        pub fn init_num(mut self, value: i32) -> Self {
            self.r#num = value.into();
            self
        }
    }
    impl ::micropb::MessageDecode for Inner {
        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
            &mut self,
            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
            len: usize,
        ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
            use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
            let before = decoder.bytes_read();
            while decoder.bytes_read() - before < len {
                let tag = decoder.decode_tag()?;
                match tag.field_num() {
                    0 => return Err(::micropb::DecodeError::ZeroField),
                    2u32 => {
                        let mut_ref = &mut self.r#num;
                        {
                            let val = decoder.decode_int32()?;
                            let val_ref = &val;
                            if *val_ref != 0 {
                                *mut_ref = val as _;
                            }
                        };
                    }
                    _ => {
                        decoder.skip_wire_value(tag.wire_type())?;
                    }
                }
            }
            Ok(())
        }
    }
    impl ::micropb::MessageEncode for Inner {
        const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
            let mut max_size = 0;
            if let ::core::option::Option::Some(size) = ::micropb::const_map!(
                ::core::option::Option::Some(10usize), | size | size + 1usize
            ) {
                max_size += size;
            } else {
                break 'msg (::core::option::Option::<usize>::None);
            };
            ::core::option::Option::Some(max_size)
        };
        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
            &self,
            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
            use ::micropb::{PbMap, FieldEncode};
            {
                let val_ref = &self.r#num;
                if *val_ref != 0 {
                    encoder.encode_varint32(16u32)?;
                    encoder.encode_int32(*val_ref as _)?;
                }
            }
            Ok(())
        }
        fn compute_size(&self) -> usize {
            use ::micropb::{PbMap, FieldEncode};
            let mut size = 0;
            {
                let val_ref = &self.r#num;
                if *val_ref != 0 {
                    size += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
                }
            }
            size
        }
    }
    /// This is an inner enum, not the outer one
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct Count(pub i32);
    impl Count {
        /// Maximum encoded size of the enum
        pub const _MAX_SIZE: usize = 10usize;
        /// Inner variant 0
        pub const Zero: Self = Self(0);
        /// Inner variant 1
        pub const One: Self = Self(1);
    }
    impl core::default::Default for Count {
        fn default() -> Self {
            Self(0)
        }
    }
    impl core::convert::From<i32> for Count {
        fn from(val: i32) -> Self {
            Self(val)
        }
    }
    /// Compact bitfield for tracking presence of optional and message fields
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct _Hazzer([u8; 1]);
    impl _Hazzer {
        /// New hazzer with all fields set to off
        #[inline]
        pub const fn _new() -> Self {
            Self([0; 1])
        }
        /// Query presence of `opt`
        #[inline]
        pub const fn r#opt(&self) -> bool {
            (self.0[0] & 1) != 0
        }
        /// Set presence of `opt`
        #[inline]
        pub const fn set_opt(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem |= 1;
            self
        }
        /// Clear presence of `opt`
        #[inline]
        pub const fn clear_opt(&mut self) -> &mut Self {
            let elem = &mut self.0[0];
            *elem &= !1;
            self
        }
        /// Builder method that sets the presence of `opt`. Useful for initializing the Hazzer.
        #[inline]
        pub const fn init_opt(mut self) -> Self {
            self.set_opt();
            self
        }
    }
    /// This is the oneof type
    #[derive(Debug, PartialEq, Clone)]
    pub enum Variant {
        /// This is a "string" variant
        St(::std::string::String),
        /// This is a boolean variant
        Flag(bool),
    }
}
/// This is the outermost message.
///
/// Comments should be converted to rustdoc
#[derive(Debug, Default, Clone)]
pub struct Msg {
    /// This is the first field.
    ///
    /// Trailing comments should also be included.
    pub r#num: i32,
    /// This an optional field with a hazzer
    ///
    /// *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.
    pub r#opt: ::std::vec::Vec<u8>,
    /// This is the oneof type
    pub r#variant: ::core::option::Option<Msg_::Variant>,
    /// Tracks presence of optional and message fields
    pub _has: Msg_::_Hazzer,
}
impl ::core::cmp::PartialEq for Msg {
    fn eq(&self, other: &Self) -> bool {
        let mut ret = true;
        ret &= (self.r#num == other.r#num);
        ret &= (self.r#opt() == other.r#opt());
        ret &= (self.r#variant == other.r#variant);
        ret
    }
}
impl Msg {
    /// Return a reference to `num`
    #[inline]
    pub fn r#num(&self) -> &i32 {
        &self.r#num
    }
    /// Return a mutable reference to `num`
    #[inline]
    pub fn mut_num(&mut self) -> &mut i32 {
        &mut self.r#num
    }
    /// Set the value of `num`
    #[inline]
    pub fn set_num(&mut self, value: i32) -> &mut Self {
        self.r#num = value.into();
        self
    }
    /// Builder method that sets the value of `num`. Useful for initializing the message.
    #[inline]
    pub fn init_num(mut self, value: i32) -> Self {
        self.r#num = value.into();
        self
    }
    /// Return a reference to `opt` as an `Option`
    #[inline]
    pub fn r#opt(&self) -> ::core::option::Option<&::std::vec::Vec<u8>> {
        self._has.r#opt().then_some(&self.r#opt)
    }
    /// Set the value and presence of `opt`
    #[inline]
    pub fn set_opt(&mut self, value: ::std::vec::Vec<u8>) -> &mut Self {
        self._has.set_opt();
        self.r#opt = value.into();
        self
    }
    /// Return a mutable reference to `opt` as an `Option`
    #[inline]
    pub fn mut_opt(&mut self) -> ::core::option::Option<&mut ::std::vec::Vec<u8>> {
        self._has.r#opt().then_some(&mut self.r#opt)
    }
    /// Clear the presence of `opt`
    #[inline]
    pub fn clear_opt(&mut self) -> &mut Self {
        self._has.clear_opt();
        self
    }
    /// Take the value of `opt` and clear its presence
    #[inline]
    pub fn take_opt(&mut self) -> ::core::option::Option<::std::vec::Vec<u8>> {
        let val = self._has.r#opt().then(|| ::core::mem::take(&mut self.r#opt));
        self._has.clear_opt();
        val
    }
    /// Builder method that sets the value of `opt`. Useful for initializing the message.
    #[inline]
    pub fn init_opt(mut self, value: ::std::vec::Vec<u8>) -> Self {
        self.set_opt(value);
        self
    }
}
impl ::micropb::MessageDecode for Msg {
    fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
        &mut self,
        decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
        len: usize,
    ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
        use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};
        let before = decoder.bytes_read();
        while decoder.bytes_read() - before < len {
            let tag = decoder.decode_tag()?;
            match tag.field_num() {
                0 => return Err(::micropb::DecodeError::ZeroField),
                1u32 => {
                    let mut_ref = &mut self.r#num;
                    {
                        let val = decoder.decode_int32()?;
                        let val_ref = &val;
                        if *val_ref != 0 {
                            *mut_ref = val as _;
                        }
                    };
                }
                5u32 => {
                    let mut_ref = &mut self.r#opt;
                    {
                        decoder.decode_bytes(mut_ref, ::micropb::Presence::Explicit)?;
                    };
                    self._has.set_opt();
                }
                2u32 => {
                    let mut_ref = loop {
                        if let ::core::option::Option::Some(variant) = &mut self
                            .r#variant
                        {
                            if let Msg_::Variant::St(variant) = &mut *variant {
                                break &mut *variant;
                            }
                        }
                        self
                            .r#variant = ::core::option::Option::Some(
                            Msg_::Variant::St(::core::default::Default::default()),
                        );
                    };
                    decoder.decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                }
                3u32 => {
                    let mut_ref = loop {
                        if let ::core::option::Option::Some(variant) = &mut self
                            .r#variant
                        {
                            if let Msg_::Variant::Flag(variant) = &mut *variant {
                                break &mut *variant;
                            }
                        }
                        self
                            .r#variant = ::core::option::Option::Some(
                            Msg_::Variant::Flag(::core::default::Default::default()),
                        );
                    };
                    let val = decoder.decode_bool()?;
                    *mut_ref = val as _;
                }
                _ => {
                    decoder.skip_wire_value(tag.wire_type())?;
                }
            }
        }
        Ok(())
    }
}
impl ::micropb::MessageEncode for Msg {
    const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
        let mut max_size = 0;
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option::Some(10usize), | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = ::micropb::const_map!(
            ::core::option::Option:: < usize > ::None, | size | size + 1usize
        ) {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        if let ::core::option::Option::Some(size) = 'oneof: {
            let mut max_size = 0;
            if let ::core::option::Option::Some(size) = ::micropb::const_map!(
                ::core::option::Option:: < usize > ::None, | size | size + 1usize
            ) {
                if size > max_size {
                    max_size = size;
                }
            } else {
                break 'oneof (::core::option::Option::<usize>::None);
            }
            if let ::core::option::Option::Some(size) = ::micropb::const_map!(
                ::core::option::Option::Some(1usize), | size | size + 1usize
            ) {
                if size > max_size {
                    max_size = size;
                }
            } else {
                break 'oneof (::core::option::Option::<usize>::None);
            }
            ::core::option::Option::Some(max_size)
        } {
            max_size += size;
        } else {
            break 'msg (::core::option::Option::<usize>::None);
        };
        ::core::option::Option::Some(max_size)
    };
    fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
        &self,
        encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
    ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
        use ::micropb::{PbMap, FieldEncode};
        {
            let val_ref = &self.r#num;
            if *val_ref != 0 {
                encoder.encode_varint32(8u32)?;
                encoder.encode_int32(*val_ref as _)?;
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#opt() {
                encoder.encode_varint32(42u32)?;
                encoder.encode_bytes(val_ref)?;
            }
        }
        if let Some(oneof) = &self.r#variant {
            match &*oneof {
                Msg_::Variant::St(val_ref) => {
                    let val_ref = &*val_ref;
                    encoder.encode_varint32(18u32)?;
                    encoder.encode_string(val_ref)?;
                }
                Msg_::Variant::Flag(val_ref) => {
                    let val_ref = &*val_ref;
                    encoder.encode_varint32(24u32)?;
                    encoder.encode_bool(*val_ref)?;
                }
            }
        }
        Ok(())
    }
    fn compute_size(&self) -> usize {
        use ::micropb::{PbMap, FieldEncode};
        let mut size = 0;
        {
            let val_ref = &self.r#num;
            if *val_ref != 0 {
                size += 1usize + ::micropb::size::sizeof_int32(*val_ref as _);
            }
        }
        {
            if let ::core::option::Option::Some(val_ref) = self.r#opt() {
                size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
            }
        }
        if let Some(oneof) = &self.r#variant {
            match &*oneof {
                Msg_::Variant::St(val_ref) => {
                    let val_ref = &*val_ref;
                    size += 1usize + ::micropb::size::sizeof_len_record(val_ref.len());
                }
                Msg_::Variant::Flag(val_ref) => {
                    let val_ref = &*val_ref;
                    size += 1usize + 1;
                }
            }
        }
        size
    }
}
/// This is an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Count(pub i32);
impl Count {
    /// Maximum encoded size of the enum
    pub const _MAX_SIZE: usize = 10usize;
    /// Variant 0
    pub const Zero: Self = Self(0);
    /// Variant 1
    pub const One: Self = Self(1);
}
impl core::default::Default for Count {
    fn default() -> Self {
        Self(0)
    }
}
impl core::convert::From<i32> for Count {
    fn from(val: i32) -> Self {
        Self(val)
    }
}
