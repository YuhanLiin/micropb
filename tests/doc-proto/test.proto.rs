pub mod Msg_ {
    /// Inner message type nested inside Msg
    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct Inner {
        /// This field belongs to the Inner message
        pub r#num: i32,
    }
    impl Inner {
        ///Return a reference to `num`
        #[inline]
        pub fn r#num(&self) -> &i32 {
            &self.r#num
        }
        ///Return a mutable reference to `num`
        #[inline]
        pub fn mut_num(&mut self) -> &mut i32 {
            &mut self.r#num
        }
        ///Set the value of `num`
        #[inline]
        pub fn set_num(&mut self, value: i32) -> &mut Self {
            self.r#num = value.into();
            self
        }
        ///Builder method that sets the value of `num`. Useful for initializing the message.
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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Msg {
    /// This is the first field.
    /// Trailing comments should also be included.
    pub r#num: i32,
    /// This is the oneof type
    pub r#variant: ::core::option::Option<Msg_::Variant>,
}
impl Msg {
    ///Return a reference to `num`
    #[inline]
    pub fn r#num(&self) -> &i32 {
        &self.r#num
    }
    ///Return a mutable reference to `num`
    #[inline]
    pub fn mut_num(&mut self) -> &mut i32 {
        &mut self.r#num
    }
    ///Set the value of `num`
    #[inline]
    pub fn set_num(&mut self, value: i32) -> &mut Self {
        self.r#num = value.into();
        self
    }
    ///Builder method that sets the value of `num`. Useful for initializing the message.
    #[inline]
    pub fn init_num(mut self, value: i32) -> Self {
        self.r#num = value.into();
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
