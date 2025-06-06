pub mod google_ {
    pub mod protobuf_ {
        #[derive(Debug, Default)]
        pub struct FileDescriptorSet {
            pub r#file: ::std::vec::Vec<FileDescriptorProto>,
        }
        impl FileDescriptorSet {}
        impl ::micropb::MessageDecode for FileDescriptorSet {
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
                            let mut val: FileDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#file.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FileDescriptorProto_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `package`
                #[inline]
                pub const fn r#package(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `package`
                #[inline]
                pub const fn set_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `package`
                #[inline]
                pub const fn clear_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `package`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_package(mut self) -> Self {
                    self.set_package();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
                ///Query presence of `source_code_info`
                #[inline]
                pub const fn r#source_code_info(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `source_code_info`
                #[inline]
                pub const fn set_source_code_info(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `source_code_info`
                #[inline]
                pub const fn clear_source_code_info(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `source_code_info`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_source_code_info(mut self) -> Self {
                    self.set_source_code_info();
                    self
                }
                ///Query presence of `syntax`
                #[inline]
                pub const fn r#syntax(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `syntax`
                #[inline]
                pub const fn set_syntax(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `syntax`
                #[inline]
                pub const fn clear_syntax(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `syntax`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_syntax(mut self) -> Self {
                    self.set_syntax();
                    self
                }
                ///Query presence of `edition`
                #[inline]
                pub const fn r#edition(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `edition`
                #[inline]
                pub const fn set_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `edition`
                #[inline]
                pub const fn clear_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `edition`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_edition(mut self) -> Self {
                    self.set_edition();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct FileDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#package: ::std::string::String,
            pub r#dependency: ::std::vec::Vec<::std::string::String>,
            pub r#public_dependency: ::std::vec::Vec<i32>,
            pub r#weak_dependency: ::std::vec::Vec<i32>,
            pub r#message_type: ::std::vec::Vec<DescriptorProto>,
            pub r#enum_type: ::std::vec::Vec<EnumDescriptorProto>,
            pub r#service: ::std::vec::Vec<ServiceDescriptorProto>,
            pub r#extension: ::std::vec::Vec<FieldDescriptorProto>,
            pub r#options: FileOptions,
            pub r#source_code_info: SourceCodeInfo,
            pub r#syntax: ::std::string::String,
            pub r#edition: Edition,
            pub _has: FileDescriptorProto_::_Hazzer,
        }
        impl FileDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Return a reference to `package` as an `Option`
            #[inline]
            pub fn r#package(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#package().then_some(&self.r#package)
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&FileOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Return a reference to `source_code_info` as an `Option`
            #[inline]
            pub fn r#source_code_info(&self) -> ::core::option::Option<&SourceCodeInfo> {
                self._has.r#source_code_info().then_some(&self.r#source_code_info)
            }
            ///Return a reference to `syntax` as an `Option`
            #[inline]
            pub fn r#syntax(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#syntax().then_some(&self.r#syntax)
            }
            ///Return a reference to `edition` as an `Option`
            #[inline]
            pub fn r#edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#edition().then_some(&self.r#edition)
            }
        }
        impl ::micropb::MessageDecode for FileDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_package();
                        }
                        3u32 => {
                            let mut val: ::std::string::String = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            if let (Err(_), false) = (
                                self.r#dependency.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        10u32 => {
                            if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                decoder
                                    .decode_packed(
                                        &mut self.r#public_dependency,
                                        |decoder| decoder.decode_int32().map(|v| v as _),
                                    )?;
                            } else {
                                if let (Err(_), false) = (
                                    self
                                        .r#public_dependency
                                        .pb_push(decoder.decode_int32()? as _),
                                    decoder.ignore_repeated_cap_err,
                                ) {
                                    return Err(::micropb::DecodeError::Capacity);
                                }
                            }
                        }
                        11u32 => {
                            if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                decoder
                                    .decode_packed(
                                        &mut self.r#weak_dependency,
                                        |decoder| decoder.decode_int32().map(|v| v as _),
                                    )?;
                            } else {
                                if let (Err(_), false) = (
                                    self
                                        .r#weak_dependency
                                        .pb_push(decoder.decode_int32()? as _),
                                    decoder.ignore_repeated_cap_err,
                                ) {
                                    return Err(::micropb::DecodeError::Capacity);
                                }
                            }
                        }
                        4u32 => {
                            let mut val: DescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#message_type.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        5u32 => {
                            let mut val: EnumDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#enum_type.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        6u32 => {
                            let mut val: ServiceDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#service.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        7u32 => {
                            let mut val: FieldDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#extension.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#source_code_info;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_source_code_info();
                        }
                        12u32 => {
                            let mut_ref = &mut self.r#syntax;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_syntax();
                        }
                        14u32 => {
                            let mut_ref = &mut self.r#edition;
                            {
                                let val = decoder.decode_int32().map(|n| Edition(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_edition();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod DescriptorProto_ {
            pub mod ExtensionRange_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `start`
                    #[inline]
                    pub const fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `start`
                    #[inline]
                    pub const fn set_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `start`
                    #[inline]
                    pub const fn clear_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `start`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_start(mut self) -> Self {
                        self.set_start();
                        self
                    }
                    ///Query presence of `end`
                    #[inline]
                    pub const fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `end`
                    #[inline]
                    pub const fn set_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `end`
                    #[inline]
                    pub const fn clear_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `end`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_end(mut self) -> Self {
                        self.set_end();
                        self
                    }
                    ///Query presence of `options`
                    #[inline]
                    pub const fn r#options(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    ///Set presence of `options`
                    #[inline]
                    pub const fn set_options(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 4;
                        self
                    }
                    ///Clear presence of `options`
                    #[inline]
                    pub const fn clear_options(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !4;
                        self
                    }
                    ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_options(mut self) -> Self {
                        self.set_options();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct ExtensionRange {
                pub r#start: i32,
                pub r#end: i32,
                pub r#options: super::ExtensionRangeOptions,
                pub _has: ExtensionRange_::_Hazzer,
            }
            impl ExtensionRange {
                ///Return a reference to `start` as an `Option`
                #[inline]
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                ///Set the value and presence of `start`
                #[inline]
                pub fn set_start(&mut self, value: i32) -> &mut Self {
                    self._has.set_start();
                    self.r#start = value.into();
                    self
                }
                ///Return a mutable reference to `start` as an `Option`
                #[inline]
                pub fn mut_start(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#start().then_some(&mut self.r#start)
                }
                ///Clear the presence of `start`
                #[inline]
                pub fn clear_start(&mut self) -> &mut Self {
                    self._has.clear_start();
                    self
                }
                ///Take the value of `start` and clear its presence
                #[inline]
                pub fn take_start(&mut self) -> ::core::option::Option<i32> {
                    let val = self
                        ._has
                        .r#start()
                        .then(|| ::core::mem::take(&mut self.r#start));
                    self._has.clear_start();
                    val
                }
                ///Builder method that sets the value of `start`. Useful for initializing the message.
                #[inline]
                pub fn init_start(mut self, value: i32) -> Self {
                    self.set_start(value);
                    self
                }
                ///Return a reference to `end` as an `Option`
                #[inline]
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                ///Set the value and presence of `end`
                #[inline]
                pub fn set_end(&mut self, value: i32) -> &mut Self {
                    self._has.set_end();
                    self.r#end = value.into();
                    self
                }
                ///Return a mutable reference to `end` as an `Option`
                #[inline]
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                ///Clear the presence of `end`
                #[inline]
                pub fn clear_end(&mut self) -> &mut Self {
                    self._has.clear_end();
                    self
                }
                ///Take the value of `end` and clear its presence
                #[inline]
                pub fn take_end(&mut self) -> ::core::option::Option<i32> {
                    let val = self
                        ._has
                        .r#end()
                        .then(|| ::core::mem::take(&mut self.r#end));
                    self._has.clear_end();
                    val
                }
                ///Builder method that sets the value of `end`. Useful for initializing the message.
                #[inline]
                pub fn init_end(mut self, value: i32) -> Self {
                    self.set_end(value);
                    self
                }
                ///Return a reference to `options` as an `Option`
                #[inline]
                pub fn r#options(
                    &self,
                ) -> ::core::option::Option<&super::ExtensionRangeOptions> {
                    self._has.r#options().then_some(&self.r#options)
                }
                ///Set the value and presence of `options`
                #[inline]
                pub fn set_options(
                    &mut self,
                    value: super::ExtensionRangeOptions,
                ) -> &mut Self {
                    self._has.set_options();
                    self.r#options = value.into();
                    self
                }
                ///Return a mutable reference to `options` as an `Option`
                #[inline]
                pub fn mut_options(
                    &mut self,
                ) -> ::core::option::Option<&mut super::ExtensionRangeOptions> {
                    self._has.r#options().then_some(&mut self.r#options)
                }
                ///Clear the presence of `options`
                #[inline]
                pub fn clear_options(&mut self) -> &mut Self {
                    self._has.clear_options();
                    self
                }
                ///Take the value of `options` and clear its presence
                #[inline]
                pub fn take_options(
                    &mut self,
                ) -> ::core::option::Option<super::ExtensionRangeOptions> {
                    let val = self
                        ._has
                        .r#options()
                        .then(|| ::core::mem::take(&mut self.r#options));
                    self._has.clear_options();
                    val
                }
                ///Builder method that sets the value of `options`. Useful for initializing the message.
                #[inline]
                pub fn init_options(
                    mut self,
                    value: super::ExtensionRangeOptions,
                ) -> Self {
                    self.set_options(value);
                    self
                }
            }
            impl ::micropb::MessageDecode for ExtensionRange {
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
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end();
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#options;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_options();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            pub mod ReservedRange_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `start`
                    #[inline]
                    pub const fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `start`
                    #[inline]
                    pub const fn set_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `start`
                    #[inline]
                    pub const fn clear_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `start`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_start(mut self) -> Self {
                        self.set_start();
                        self
                    }
                    ///Query presence of `end`
                    #[inline]
                    pub const fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `end`
                    #[inline]
                    pub const fn set_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `end`
                    #[inline]
                    pub const fn clear_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `end`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_end(mut self) -> Self {
                        self.set_end();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct ReservedRange {
                pub r#start: i32,
                pub r#end: i32,
                pub _has: ReservedRange_::_Hazzer,
            }
            impl ReservedRange {
                ///Return a reference to `start` as an `Option`
                #[inline]
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                ///Set the value and presence of `start`
                #[inline]
                pub fn set_start(&mut self, value: i32) -> &mut Self {
                    self._has.set_start();
                    self.r#start = value.into();
                    self
                }
                ///Return a mutable reference to `start` as an `Option`
                #[inline]
                pub fn mut_start(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#start().then_some(&mut self.r#start)
                }
                ///Clear the presence of `start`
                #[inline]
                pub fn clear_start(&mut self) -> &mut Self {
                    self._has.clear_start();
                    self
                }
                ///Take the value of `start` and clear its presence
                #[inline]
                pub fn take_start(&mut self) -> ::core::option::Option<i32> {
                    let val = self
                        ._has
                        .r#start()
                        .then(|| ::core::mem::take(&mut self.r#start));
                    self._has.clear_start();
                    val
                }
                ///Builder method that sets the value of `start`. Useful for initializing the message.
                #[inline]
                pub fn init_start(mut self, value: i32) -> Self {
                    self.set_start(value);
                    self
                }
                ///Return a reference to `end` as an `Option`
                #[inline]
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                ///Set the value and presence of `end`
                #[inline]
                pub fn set_end(&mut self, value: i32) -> &mut Self {
                    self._has.set_end();
                    self.r#end = value.into();
                    self
                }
                ///Return a mutable reference to `end` as an `Option`
                #[inline]
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                ///Clear the presence of `end`
                #[inline]
                pub fn clear_end(&mut self) -> &mut Self {
                    self._has.clear_end();
                    self
                }
                ///Take the value of `end` and clear its presence
                #[inline]
                pub fn take_end(&mut self) -> ::core::option::Option<i32> {
                    let val = self
                        ._has
                        .r#end()
                        .then(|| ::core::mem::take(&mut self.r#end));
                    self._has.clear_end();
                    val
                }
                ///Builder method that sets the value of `end`. Useful for initializing the message.
                #[inline]
                pub fn init_end(mut self, value: i32) -> Self {
                    self.set_end(value);
                    self
                }
            }
            impl ::micropb::MessageDecode for ReservedRange {
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
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct DescriptorProto {
            pub r#name: ::std::string::String,
            pub r#field: ::std::vec::Vec<FieldDescriptorProto>,
            pub r#extension: ::std::vec::Vec<FieldDescriptorProto>,
            pub r#nested_type: ::std::vec::Vec<DescriptorProto>,
            pub r#enum_type: ::std::vec::Vec<EnumDescriptorProto>,
            pub r#extension_range: ::std::vec::Vec<DescriptorProto_::ExtensionRange>,
            pub r#oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,
            pub r#options: MessageOptions,
            pub r#reserved_range: ::std::vec::Vec<DescriptorProto_::ReservedRange>,
            pub r#reserved_name: ::std::vec::Vec<::std::string::String>,
            pub _has: DescriptorProto_::_Hazzer,
        }
        impl DescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Set the value and presence of `name`
            #[inline]
            pub fn set_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_name();
                self.r#name = value.into();
                self
            }
            ///Return a mutable reference to `name` as an `Option`
            #[inline]
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            ///Clear the presence of `name`
            #[inline]
            pub fn clear_name(&mut self) -> &mut Self {
                self._has.clear_name();
                self
            }
            ///Take the value of `name` and clear its presence
            #[inline]
            pub fn take_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#name()
                    .then(|| ::core::mem::take(&mut self.r#name));
                self._has.clear_name();
                val
            }
            ///Builder method that sets the value of `name`. Useful for initializing the message.
            #[inline]
            pub fn init_name(mut self, value: ::std::string::String) -> Self {
                self.set_name(value);
                self
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&MessageOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Set the value and presence of `options`
            #[inline]
            pub fn set_options(&mut self, value: MessageOptions) -> &mut Self {
                self._has.set_options();
                self.r#options = value.into();
                self
            }
            ///Return a mutable reference to `options` as an `Option`
            #[inline]
            pub fn mut_options(
                &mut self,
            ) -> ::core::option::Option<&mut MessageOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            ///Clear the presence of `options`
            #[inline]
            pub fn clear_options(&mut self) -> &mut Self {
                self._has.clear_options();
                self
            }
            ///Take the value of `options` and clear its presence
            #[inline]
            pub fn take_options(&mut self) -> ::core::option::Option<MessageOptions> {
                let val = self
                    ._has
                    .r#options()
                    .then(|| ::core::mem::take(&mut self.r#options));
                self._has.clear_options();
                val
            }
            ///Builder method that sets the value of `options`. Useful for initializing the message.
            #[inline]
            pub fn init_options(mut self, value: MessageOptions) -> Self {
                self.set_options(value);
                self
            }
        }
        impl ::micropb::MessageDecode for DescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut val: FieldDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#field.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        6u32 => {
                            let mut val: FieldDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#extension.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        3u32 => {
                            let mut val: DescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#nested_type.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        4u32 => {
                            let mut val: EnumDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#enum_type.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        5u32 => {
                            let mut val: DescriptorProto_::ExtensionRange = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#extension_range.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        8u32 => {
                            let mut val: OneofDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#oneof_decl.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        9u32 => {
                            let mut val: DescriptorProto_::ReservedRange = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#reserved_range.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        10u32 => {
                            let mut val: ::std::string::String = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            if let (Err(_), false) = (
                                self.r#reserved_name.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod ExtensionRangeOptions_ {
            pub mod Declaration_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `number`
                    #[inline]
                    pub const fn r#number(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `number`
                    #[inline]
                    pub const fn set_number(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `number`
                    #[inline]
                    pub const fn clear_number(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `number`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_number(mut self) -> Self {
                        self.set_number();
                        self
                    }
                    ///Query presence of `full_name`
                    #[inline]
                    pub const fn r#full_name(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `full_name`
                    #[inline]
                    pub const fn set_full_name(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `full_name`
                    #[inline]
                    pub const fn clear_full_name(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `full_name`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_full_name(mut self) -> Self {
                        self.set_full_name();
                        self
                    }
                    ///Query presence of `type`
                    #[inline]
                    pub const fn r#type(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    ///Set presence of `type`
                    #[inline]
                    pub const fn set_type(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 4;
                        self
                    }
                    ///Clear presence of `type`
                    #[inline]
                    pub const fn clear_type(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !4;
                        self
                    }
                    ///Builder method that sets the presence of `type`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_type(mut self) -> Self {
                        self.set_type();
                        self
                    }
                    ///Query presence of `reserved`
                    #[inline]
                    pub const fn r#reserved(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    ///Set presence of `reserved`
                    #[inline]
                    pub const fn set_reserved(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 8;
                        self
                    }
                    ///Clear presence of `reserved`
                    #[inline]
                    pub const fn clear_reserved(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !8;
                        self
                    }
                    ///Builder method that sets the presence of `reserved`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_reserved(mut self) -> Self {
                        self.set_reserved();
                        self
                    }
                    ///Query presence of `repeated`
                    #[inline]
                    pub const fn r#repeated(&self) -> bool {
                        (self.0[0] & 16) != 0
                    }
                    ///Set presence of `repeated`
                    #[inline]
                    pub const fn set_repeated(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 16;
                        self
                    }
                    ///Clear presence of `repeated`
                    #[inline]
                    pub const fn clear_repeated(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !16;
                        self
                    }
                    ///Builder method that sets the presence of `repeated`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_repeated(mut self) -> Self {
                        self.set_repeated();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct Declaration {
                pub r#number: i32,
                pub r#full_name: ::std::string::String,
                pub r#type: ::std::string::String,
                pub r#reserved: bool,
                pub r#repeated: bool,
                pub _has: Declaration_::_Hazzer,
            }
            impl Declaration {
                ///Return a reference to `number` as an `Option`
                #[inline]
                pub fn r#number(&self) -> ::core::option::Option<&i32> {
                    self._has.r#number().then_some(&self.r#number)
                }
                ///Return a reference to `full_name` as an `Option`
                #[inline]
                pub fn r#full_name(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#full_name().then_some(&self.r#full_name)
                }
                ///Return a reference to `type` as an `Option`
                #[inline]
                pub fn r#type(&self) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#type().then_some(&self.r#type)
                }
                ///Return a reference to `reserved` as an `Option`
                #[inline]
                pub fn r#reserved(&self) -> ::core::option::Option<&bool> {
                    self._has.r#reserved().then_some(&self.r#reserved)
                }
                ///Return a reference to `repeated` as an `Option`
                #[inline]
                pub fn r#repeated(&self) -> ::core::option::Option<&bool> {
                    self._has.r#repeated().then_some(&self.r#repeated)
                }
            }
            impl ::micropb::MessageDecode for Declaration {
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
                                let mut_ref = &mut self.r#number;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_number();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#full_name;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_full_name();
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#type;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_type();
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#reserved;
                                {
                                    let val = decoder.decode_bool()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_reserved();
                            }
                            6u32 => {
                                let mut_ref = &mut self.r#repeated;
                                {
                                    let val = decoder.decode_bool()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_repeated();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct VerificationState(pub i32);
            impl VerificationState {
                pub const Declaration: Self = Self(0);
                pub const Unverified: Self = Self(1);
            }
            impl core::default::Default for VerificationState {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for VerificationState {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
                ///Query presence of `verification`
                #[inline]
                pub const fn r#verification(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `verification`
                #[inline]
                pub const fn set_verification(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `verification`
                #[inline]
                pub const fn clear_verification(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `verification`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_verification(mut self) -> Self {
                    self.set_verification();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct ExtensionRangeOptions {
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub r#declaration: ::std::vec::Vec<ExtensionRangeOptions_::Declaration>,
            pub r#features: FeatureSet,
            pub r#verification: ExtensionRangeOptions_::VerificationState,
            pub _has: ExtensionRangeOptions_::_Hazzer,
        }
        impl ::core::default::Default for ExtensionRangeOptions {
            fn default() -> Self {
                Self {
                    r#uninterpreted_option: ::core::default::Default::default(),
                    r#declaration: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#verification: ExtensionRangeOptions_::VerificationState::Unverified,
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl ExtensionRangeOptions {
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            ///Return a reference to `verification` as an `Option`
            #[inline]
            pub fn r#verification(
                &self,
            ) -> ::core::option::Option<&ExtensionRangeOptions_::VerificationState> {
                self._has.r#verification().then_some(&self.r#verification)
            }
        }
        impl ::micropb::MessageDecode for ExtensionRangeOptions {
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
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        2u32 => {
                            let mut val: ExtensionRangeOptions_::Declaration = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#declaration.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        50u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#verification;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| ExtensionRangeOptions_::VerificationState(
                                        n as _,
                                    ))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_verification();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FieldDescriptorProto_ {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct Type(pub i32);
            impl Type {
                pub const Double: Self = Self(1);
                pub const Float: Self = Self(2);
                pub const Int64: Self = Self(3);
                pub const Uint64: Self = Self(4);
                pub const Int32: Self = Self(5);
                pub const Fixed64: Self = Self(6);
                pub const Fixed32: Self = Self(7);
                pub const Bool: Self = Self(8);
                pub const String: Self = Self(9);
                pub const Group: Self = Self(10);
                pub const Message: Self = Self(11);
                pub const Bytes: Self = Self(12);
                pub const Uint32: Self = Self(13);
                pub const Enum: Self = Self(14);
                pub const Sfixed32: Self = Self(15);
                pub const Sfixed64: Self = Self(16);
                pub const Sint32: Self = Self(17);
                pub const Sint64: Self = Self(18);
            }
            impl core::default::Default for Type {
                fn default() -> Self {
                    Self(1)
                }
            }
            impl core::convert::From<i32> for Type {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct Label(pub i32);
            impl Label {
                pub const Optional: Self = Self(1);
                pub const Repeated: Self = Self(3);
                pub const Required: Self = Self(2);
            }
            impl core::default::Default for Label {
                fn default() -> Self {
                    Self(1)
                }
            }
            impl core::convert::From<i32> for Label {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 2]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 2])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `number`
                #[inline]
                pub const fn r#number(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `number`
                #[inline]
                pub const fn set_number(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `number`
                #[inline]
                pub const fn clear_number(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `number`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_number(mut self) -> Self {
                    self.set_number();
                    self
                }
                ///Query presence of `label`
                #[inline]
                pub const fn r#label(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `label`
                #[inline]
                pub const fn set_label(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `label`
                #[inline]
                pub const fn clear_label(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `label`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_label(mut self) -> Self {
                    self.set_label();
                    self
                }
                ///Query presence of `type`
                #[inline]
                pub const fn r#type(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `type`
                #[inline]
                pub const fn set_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `type`
                #[inline]
                pub const fn clear_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `type`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_type(mut self) -> Self {
                    self.set_type();
                    self
                }
                ///Query presence of `type_name`
                #[inline]
                pub const fn r#type_name(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `type_name`
                #[inline]
                pub const fn set_type_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `type_name`
                #[inline]
                pub const fn clear_type_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `type_name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_type_name(mut self) -> Self {
                    self.set_type_name();
                    self
                }
                ///Query presence of `extendee`
                #[inline]
                pub const fn r#extendee(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `extendee`
                #[inline]
                pub const fn set_extendee(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `extendee`
                #[inline]
                pub const fn clear_extendee(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `extendee`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_extendee(mut self) -> Self {
                    self.set_extendee();
                    self
                }
                ///Query presence of `default_value`
                #[inline]
                pub const fn r#default_value(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                ///Set presence of `default_value`
                #[inline]
                pub const fn set_default_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 64;
                    self
                }
                ///Clear presence of `default_value`
                #[inline]
                pub const fn clear_default_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !64;
                    self
                }
                ///Builder method that sets the presence of `default_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_default_value(mut self) -> Self {
                    self.set_default_value();
                    self
                }
                ///Query presence of `oneof_index`
                #[inline]
                pub const fn r#oneof_index(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                ///Set presence of `oneof_index`
                #[inline]
                pub const fn set_oneof_index(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 128;
                    self
                }
                ///Clear presence of `oneof_index`
                #[inline]
                pub const fn clear_oneof_index(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !128;
                    self
                }
                ///Builder method that sets the presence of `oneof_index`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_oneof_index(mut self) -> Self {
                    self.set_oneof_index();
                    self
                }
                ///Query presence of `json_name`
                #[inline]
                pub const fn r#json_name(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                ///Set presence of `json_name`
                #[inline]
                pub const fn set_json_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `json_name`
                #[inline]
                pub const fn clear_json_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `json_name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_json_name(mut self) -> Self {
                    self.set_json_name();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
                ///Query presence of `proto3_optional`
                #[inline]
                pub const fn r#proto3_optional(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                ///Set presence of `proto3_optional`
                #[inline]
                pub const fn set_proto3_optional(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `proto3_optional`
                #[inline]
                pub const fn clear_proto3_optional(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `proto3_optional`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_proto3_optional(mut self) -> Self {
                    self.set_proto3_optional();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct FieldDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#number: i32,
            pub r#label: FieldDescriptorProto_::Label,
            pub r#type: FieldDescriptorProto_::Type,
            pub r#type_name: ::std::string::String,
            pub r#extendee: ::std::string::String,
            pub r#default_value: ::std::string::String,
            pub r#oneof_index: i32,
            pub r#json_name: ::std::string::String,
            pub r#options: FieldOptions,
            pub r#proto3_optional: bool,
            pub _has: FieldDescriptorProto_::_Hazzer,
        }
        impl FieldDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Set the value and presence of `name`
            #[inline]
            pub fn set_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_name();
                self.r#name = value.into();
                self
            }
            ///Return a mutable reference to `name` as an `Option`
            #[inline]
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            ///Clear the presence of `name`
            #[inline]
            pub fn clear_name(&mut self) -> &mut Self {
                self._has.clear_name();
                self
            }
            ///Take the value of `name` and clear its presence
            #[inline]
            pub fn take_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#name()
                    .then(|| ::core::mem::take(&mut self.r#name));
                self._has.clear_name();
                val
            }
            ///Builder method that sets the value of `name`. Useful for initializing the message.
            #[inline]
            pub fn init_name(mut self, value: ::std::string::String) -> Self {
                self.set_name(value);
                self
            }
            ///Return a reference to `number` as an `Option`
            #[inline]
            pub fn r#number(&self) -> ::core::option::Option<&i32> {
                self._has.r#number().then_some(&self.r#number)
            }
            ///Set the value and presence of `number`
            #[inline]
            pub fn set_number(&mut self, value: i32) -> &mut Self {
                self._has.set_number();
                self.r#number = value.into();
                self
            }
            ///Return a mutable reference to `number` as an `Option`
            #[inline]
            pub fn mut_number(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#number().then_some(&mut self.r#number)
            }
            ///Clear the presence of `number`
            #[inline]
            pub fn clear_number(&mut self) -> &mut Self {
                self._has.clear_number();
                self
            }
            ///Take the value of `number` and clear its presence
            #[inline]
            pub fn take_number(&mut self) -> ::core::option::Option<i32> {
                let val = self
                    ._has
                    .r#number()
                    .then(|| ::core::mem::take(&mut self.r#number));
                self._has.clear_number();
                val
            }
            ///Builder method that sets the value of `number`. Useful for initializing the message.
            #[inline]
            pub fn init_number(mut self, value: i32) -> Self {
                self.set_number(value);
                self
            }
            ///Return a reference to `label` as an `Option`
            #[inline]
            pub fn r#label(
                &self,
            ) -> ::core::option::Option<&FieldDescriptorProto_::Label> {
                self._has.r#label().then_some(&self.r#label)
            }
            ///Set the value and presence of `label`
            #[inline]
            pub fn set_label(
                &mut self,
                value: FieldDescriptorProto_::Label,
            ) -> &mut Self {
                self._has.set_label();
                self.r#label = value.into();
                self
            }
            ///Return a mutable reference to `label` as an `Option`
            #[inline]
            pub fn mut_label(
                &mut self,
            ) -> ::core::option::Option<&mut FieldDescriptorProto_::Label> {
                self._has.r#label().then_some(&mut self.r#label)
            }
            ///Clear the presence of `label`
            #[inline]
            pub fn clear_label(&mut self) -> &mut Self {
                self._has.clear_label();
                self
            }
            ///Take the value of `label` and clear its presence
            #[inline]
            pub fn take_label(
                &mut self,
            ) -> ::core::option::Option<FieldDescriptorProto_::Label> {
                let val = self
                    ._has
                    .r#label()
                    .then(|| ::core::mem::take(&mut self.r#label));
                self._has.clear_label();
                val
            }
            ///Builder method that sets the value of `label`. Useful for initializing the message.
            #[inline]
            pub fn init_label(mut self, value: FieldDescriptorProto_::Label) -> Self {
                self.set_label(value);
                self
            }
            ///Return a reference to `type` as an `Option`
            #[inline]
            pub fn r#type(
                &self,
            ) -> ::core::option::Option<&FieldDescriptorProto_::Type> {
                self._has.r#type().then_some(&self.r#type)
            }
            ///Set the value and presence of `type`
            #[inline]
            pub fn set_type(&mut self, value: FieldDescriptorProto_::Type) -> &mut Self {
                self._has.set_type();
                self.r#type = value.into();
                self
            }
            ///Return a mutable reference to `type` as an `Option`
            #[inline]
            pub fn mut_type(
                &mut self,
            ) -> ::core::option::Option<&mut FieldDescriptorProto_::Type> {
                self._has.r#type().then_some(&mut self.r#type)
            }
            ///Clear the presence of `type`
            #[inline]
            pub fn clear_type(&mut self) -> &mut Self {
                self._has.clear_type();
                self
            }
            ///Take the value of `type` and clear its presence
            #[inline]
            pub fn take_type(
                &mut self,
            ) -> ::core::option::Option<FieldDescriptorProto_::Type> {
                let val = self
                    ._has
                    .r#type()
                    .then(|| ::core::mem::take(&mut self.r#type));
                self._has.clear_type();
                val
            }
            ///Builder method that sets the value of `type`. Useful for initializing the message.
            #[inline]
            pub fn init_type(mut self, value: FieldDescriptorProto_::Type) -> Self {
                self.set_type(value);
                self
            }
            ///Return a reference to `type_name` as an `Option`
            #[inline]
            pub fn r#type_name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#type_name().then_some(&self.r#type_name)
            }
            ///Set the value and presence of `type_name`
            #[inline]
            pub fn set_type_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_type_name();
                self.r#type_name = value.into();
                self
            }
            ///Return a mutable reference to `type_name` as an `Option`
            #[inline]
            pub fn mut_type_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#type_name().then_some(&mut self.r#type_name)
            }
            ///Clear the presence of `type_name`
            #[inline]
            pub fn clear_type_name(&mut self) -> &mut Self {
                self._has.clear_type_name();
                self
            }
            ///Take the value of `type_name` and clear its presence
            #[inline]
            pub fn take_type_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#type_name()
                    .then(|| ::core::mem::take(&mut self.r#type_name));
                self._has.clear_type_name();
                val
            }
            ///Builder method that sets the value of `type_name`. Useful for initializing the message.
            #[inline]
            pub fn init_type_name(mut self, value: ::std::string::String) -> Self {
                self.set_type_name(value);
                self
            }
            ///Return a reference to `extendee` as an `Option`
            #[inline]
            pub fn r#extendee(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#extendee().then_some(&self.r#extendee)
            }
            ///Set the value and presence of `extendee`
            #[inline]
            pub fn set_extendee(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_extendee();
                self.r#extendee = value.into();
                self
            }
            ///Return a mutable reference to `extendee` as an `Option`
            #[inline]
            pub fn mut_extendee(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#extendee().then_some(&mut self.r#extendee)
            }
            ///Clear the presence of `extendee`
            #[inline]
            pub fn clear_extendee(&mut self) -> &mut Self {
                self._has.clear_extendee();
                self
            }
            ///Take the value of `extendee` and clear its presence
            #[inline]
            pub fn take_extendee(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#extendee()
                    .then(|| ::core::mem::take(&mut self.r#extendee));
                self._has.clear_extendee();
                val
            }
            ///Builder method that sets the value of `extendee`. Useful for initializing the message.
            #[inline]
            pub fn init_extendee(mut self, value: ::std::string::String) -> Self {
                self.set_extendee(value);
                self
            }
            ///Return a reference to `default_value` as an `Option`
            #[inline]
            pub fn r#default_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#default_value().then_some(&self.r#default_value)
            }
            ///Set the value and presence of `default_value`
            #[inline]
            pub fn set_default_value(
                &mut self,
                value: ::std::string::String,
            ) -> &mut Self {
                self._has.set_default_value();
                self.r#default_value = value.into();
                self
            }
            ///Return a mutable reference to `default_value` as an `Option`
            #[inline]
            pub fn mut_default_value(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#default_value().then_some(&mut self.r#default_value)
            }
            ///Clear the presence of `default_value`
            #[inline]
            pub fn clear_default_value(&mut self) -> &mut Self {
                self._has.clear_default_value();
                self
            }
            ///Take the value of `default_value` and clear its presence
            #[inline]
            pub fn take_default_value(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#default_value()
                    .then(|| ::core::mem::take(&mut self.r#default_value));
                self._has.clear_default_value();
                val
            }
            ///Builder method that sets the value of `default_value`. Useful for initializing the message.
            #[inline]
            pub fn init_default_value(mut self, value: ::std::string::String) -> Self {
                self.set_default_value(value);
                self
            }
            ///Return a reference to `oneof_index` as an `Option`
            #[inline]
            pub fn r#oneof_index(&self) -> ::core::option::Option<&i32> {
                self._has.r#oneof_index().then_some(&self.r#oneof_index)
            }
            ///Set the value and presence of `oneof_index`
            #[inline]
            pub fn set_oneof_index(&mut self, value: i32) -> &mut Self {
                self._has.set_oneof_index();
                self.r#oneof_index = value.into();
                self
            }
            ///Return a mutable reference to `oneof_index` as an `Option`
            #[inline]
            pub fn mut_oneof_index(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#oneof_index().then_some(&mut self.r#oneof_index)
            }
            ///Clear the presence of `oneof_index`
            #[inline]
            pub fn clear_oneof_index(&mut self) -> &mut Self {
                self._has.clear_oneof_index();
                self
            }
            ///Take the value of `oneof_index` and clear its presence
            #[inline]
            pub fn take_oneof_index(&mut self) -> ::core::option::Option<i32> {
                let val = self
                    ._has
                    .r#oneof_index()
                    .then(|| ::core::mem::take(&mut self.r#oneof_index));
                self._has.clear_oneof_index();
                val
            }
            ///Builder method that sets the value of `oneof_index`. Useful for initializing the message.
            #[inline]
            pub fn init_oneof_index(mut self, value: i32) -> Self {
                self.set_oneof_index(value);
                self
            }
            ///Return a reference to `json_name` as an `Option`
            #[inline]
            pub fn r#json_name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#json_name().then_some(&self.r#json_name)
            }
            ///Set the value and presence of `json_name`
            #[inline]
            pub fn set_json_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_json_name();
                self.r#json_name = value.into();
                self
            }
            ///Return a mutable reference to `json_name` as an `Option`
            #[inline]
            pub fn mut_json_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#json_name().then_some(&mut self.r#json_name)
            }
            ///Clear the presence of `json_name`
            #[inline]
            pub fn clear_json_name(&mut self) -> &mut Self {
                self._has.clear_json_name();
                self
            }
            ///Take the value of `json_name` and clear its presence
            #[inline]
            pub fn take_json_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#json_name()
                    .then(|| ::core::mem::take(&mut self.r#json_name));
                self._has.clear_json_name();
                val
            }
            ///Builder method that sets the value of `json_name`. Useful for initializing the message.
            #[inline]
            pub fn init_json_name(mut self, value: ::std::string::String) -> Self {
                self.set_json_name(value);
                self
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&FieldOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Set the value and presence of `options`
            #[inline]
            pub fn set_options(&mut self, value: FieldOptions) -> &mut Self {
                self._has.set_options();
                self.r#options = value.into();
                self
            }
            ///Return a mutable reference to `options` as an `Option`
            #[inline]
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut FieldOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            ///Clear the presence of `options`
            #[inline]
            pub fn clear_options(&mut self) -> &mut Self {
                self._has.clear_options();
                self
            }
            ///Take the value of `options` and clear its presence
            #[inline]
            pub fn take_options(&mut self) -> ::core::option::Option<FieldOptions> {
                let val = self
                    ._has
                    .r#options()
                    .then(|| ::core::mem::take(&mut self.r#options));
                self._has.clear_options();
                val
            }
            ///Builder method that sets the value of `options`. Useful for initializing the message.
            #[inline]
            pub fn init_options(mut self, value: FieldOptions) -> Self {
                self.set_options(value);
                self
            }
            ///Return a reference to `proto3_optional` as an `Option`
            #[inline]
            pub fn r#proto3_optional(&self) -> ::core::option::Option<&bool> {
                self._has.r#proto3_optional().then_some(&self.r#proto3_optional)
            }
            ///Set the value and presence of `proto3_optional`
            #[inline]
            pub fn set_proto3_optional(&mut self, value: bool) -> &mut Self {
                self._has.set_proto3_optional();
                self.r#proto3_optional = value.into();
                self
            }
            ///Return a mutable reference to `proto3_optional` as an `Option`
            #[inline]
            pub fn mut_proto3_optional(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#proto3_optional().then_some(&mut self.r#proto3_optional)
            }
            ///Clear the presence of `proto3_optional`
            #[inline]
            pub fn clear_proto3_optional(&mut self) -> &mut Self {
                self._has.clear_proto3_optional();
                self
            }
            ///Take the value of `proto3_optional` and clear its presence
            #[inline]
            pub fn take_proto3_optional(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#proto3_optional()
                    .then(|| ::core::mem::take(&mut self.r#proto3_optional));
                self._has.clear_proto3_optional();
                val
            }
            ///Builder method that sets the value of `proto3_optional`. Useful for initializing the message.
            #[inline]
            pub fn init_proto3_optional(mut self, value: bool) -> Self {
                self.set_proto3_optional(value);
                self
            }
        }
        impl ::micropb::MessageDecode for FieldDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#number;
                            {
                                let val = decoder.decode_int32()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_number();
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#label;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FieldDescriptorProto_::Label(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_label();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#type;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FieldDescriptorProto_::Type(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_type();
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#type_name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_type_name();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#extendee;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_extendee();
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#default_value;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_default_value();
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#oneof_index;
                            {
                                let val = decoder.decode_int32()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_oneof_index();
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#json_name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_json_name();
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#proto3_optional;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_proto3_optional();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod OneofDescriptorProto_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct OneofDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#options: OneofOptions,
            pub _has: OneofDescriptorProto_::_Hazzer,
        }
        impl OneofDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Set the value and presence of `name`
            #[inline]
            pub fn set_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_name();
                self.r#name = value.into();
                self
            }
            ///Return a mutable reference to `name` as an `Option`
            #[inline]
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            ///Clear the presence of `name`
            #[inline]
            pub fn clear_name(&mut self) -> &mut Self {
                self._has.clear_name();
                self
            }
            ///Take the value of `name` and clear its presence
            #[inline]
            pub fn take_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#name()
                    .then(|| ::core::mem::take(&mut self.r#name));
                self._has.clear_name();
                val
            }
            ///Builder method that sets the value of `name`. Useful for initializing the message.
            #[inline]
            pub fn init_name(mut self, value: ::std::string::String) -> Self {
                self.set_name(value);
                self
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&OneofOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Set the value and presence of `options`
            #[inline]
            pub fn set_options(&mut self, value: OneofOptions) -> &mut Self {
                self._has.set_options();
                self.r#options = value.into();
                self
            }
            ///Return a mutable reference to `options` as an `Option`
            #[inline]
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut OneofOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            ///Clear the presence of `options`
            #[inline]
            pub fn clear_options(&mut self) -> &mut Self {
                self._has.clear_options();
                self
            }
            ///Take the value of `options` and clear its presence
            #[inline]
            pub fn take_options(&mut self) -> ::core::option::Option<OneofOptions> {
                let val = self
                    ._has
                    .r#options()
                    .then(|| ::core::mem::take(&mut self.r#options));
                self._has.clear_options();
                val
            }
            ///Builder method that sets the value of `options`. Useful for initializing the message.
            #[inline]
            pub fn init_options(mut self, value: OneofOptions) -> Self {
                self.set_options(value);
                self
            }
        }
        impl ::micropb::MessageDecode for OneofDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod EnumDescriptorProto_ {
            pub mod EnumReservedRange_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `start`
                    #[inline]
                    pub const fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `start`
                    #[inline]
                    pub const fn set_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `start`
                    #[inline]
                    pub const fn clear_start(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `start`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_start(mut self) -> Self {
                        self.set_start();
                        self
                    }
                    ///Query presence of `end`
                    #[inline]
                    pub const fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `end`
                    #[inline]
                    pub const fn set_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `end`
                    #[inline]
                    pub const fn clear_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `end`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_end(mut self) -> Self {
                        self.set_end();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct EnumReservedRange {
                pub r#start: i32,
                pub r#end: i32,
                pub _has: EnumReservedRange_::_Hazzer,
            }
            impl EnumReservedRange {
                ///Return a reference to `start` as an `Option`
                #[inline]
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                ///Return a reference to `end` as an `Option`
                #[inline]
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
            }
            impl ::micropb::MessageDecode for EnumReservedRange {
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
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct EnumDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#value: ::std::vec::Vec<EnumValueDescriptorProto>,
            pub r#options: EnumOptions,
            pub r#reserved_range: ::std::vec::Vec<
                EnumDescriptorProto_::EnumReservedRange,
            >,
            pub r#reserved_name: ::std::vec::Vec<::std::string::String>,
            pub _has: EnumDescriptorProto_::_Hazzer,
        }
        impl EnumDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&EnumOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
        }
        impl ::micropb::MessageDecode for EnumDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut val: EnumValueDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#value.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        4u32 => {
                            let mut val: EnumDescriptorProto_::EnumReservedRange = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#reserved_range.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        5u32 => {
                            let mut val: ::std::string::String = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            if let (Err(_), false) = (
                                self.r#reserved_name.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod EnumValueDescriptorProto_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `number`
                #[inline]
                pub const fn r#number(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `number`
                #[inline]
                pub const fn set_number(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `number`
                #[inline]
                pub const fn clear_number(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `number`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_number(mut self) -> Self {
                    self.set_number();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct EnumValueDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#number: i32,
            pub r#options: EnumValueOptions,
            pub _has: EnumValueDescriptorProto_::_Hazzer,
        }
        impl EnumValueDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Set the value and presence of `name`
            #[inline]
            pub fn set_name(&mut self, value: ::std::string::String) -> &mut Self {
                self._has.set_name();
                self.r#name = value.into();
                self
            }
            ///Return a mutable reference to `name` as an `Option`
            #[inline]
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            ///Clear the presence of `name`
            #[inline]
            pub fn clear_name(&mut self) -> &mut Self {
                self._has.clear_name();
                self
            }
            ///Take the value of `name` and clear its presence
            #[inline]
            pub fn take_name(
                &mut self,
            ) -> ::core::option::Option<::std::string::String> {
                let val = self
                    ._has
                    .r#name()
                    .then(|| ::core::mem::take(&mut self.r#name));
                self._has.clear_name();
                val
            }
            ///Builder method that sets the value of `name`. Useful for initializing the message.
            #[inline]
            pub fn init_name(mut self, value: ::std::string::String) -> Self {
                self.set_name(value);
                self
            }
            ///Return a reference to `number` as an `Option`
            #[inline]
            pub fn r#number(&self) -> ::core::option::Option<&i32> {
                self._has.r#number().then_some(&self.r#number)
            }
            ///Set the value and presence of `number`
            #[inline]
            pub fn set_number(&mut self, value: i32) -> &mut Self {
                self._has.set_number();
                self.r#number = value.into();
                self
            }
            ///Return a mutable reference to `number` as an `Option`
            #[inline]
            pub fn mut_number(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#number().then_some(&mut self.r#number)
            }
            ///Clear the presence of `number`
            #[inline]
            pub fn clear_number(&mut self) -> &mut Self {
                self._has.clear_number();
                self
            }
            ///Take the value of `number` and clear its presence
            #[inline]
            pub fn take_number(&mut self) -> ::core::option::Option<i32> {
                let val = self
                    ._has
                    .r#number()
                    .then(|| ::core::mem::take(&mut self.r#number));
                self._has.clear_number();
                val
            }
            ///Builder method that sets the value of `number`. Useful for initializing the message.
            #[inline]
            pub fn init_number(mut self, value: i32) -> Self {
                self.set_number(value);
                self
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&EnumValueOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Set the value and presence of `options`
            #[inline]
            pub fn set_options(&mut self, value: EnumValueOptions) -> &mut Self {
                self._has.set_options();
                self.r#options = value.into();
                self
            }
            ///Return a mutable reference to `options` as an `Option`
            #[inline]
            pub fn mut_options(
                &mut self,
            ) -> ::core::option::Option<&mut EnumValueOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            ///Clear the presence of `options`
            #[inline]
            pub fn clear_options(&mut self) -> &mut Self {
                self._has.clear_options();
                self
            }
            ///Take the value of `options` and clear its presence
            #[inline]
            pub fn take_options(&mut self) -> ::core::option::Option<EnumValueOptions> {
                let val = self
                    ._has
                    .r#options()
                    .then(|| ::core::mem::take(&mut self.r#options));
                self._has.clear_options();
                val
            }
            ///Builder method that sets the value of `options`. Useful for initializing the message.
            #[inline]
            pub fn init_options(mut self, value: EnumValueOptions) -> Self {
                self.set_options(value);
                self
            }
        }
        impl ::micropb::MessageDecode for EnumValueDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#number;
                            {
                                let val = decoder.decode_int32()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_number();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod ServiceDescriptorProto_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct ServiceDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#method: ::std::vec::Vec<MethodDescriptorProto>,
            pub r#options: ServiceOptions,
            pub _has: ServiceDescriptorProto_::_Hazzer,
        }
        impl ServiceDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&ServiceOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
        }
        impl ::micropb::MessageDecode for ServiceDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut val: MethodDescriptorProto = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#method.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod MethodDescriptorProto_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `name`
                #[inline]
                pub const fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `name`
                #[inline]
                pub const fn set_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `name`
                #[inline]
                pub const fn clear_name(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `name`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_name(mut self) -> Self {
                    self.set_name();
                    self
                }
                ///Query presence of `input_type`
                #[inline]
                pub const fn r#input_type(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `input_type`
                #[inline]
                pub const fn set_input_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `input_type`
                #[inline]
                pub const fn clear_input_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `input_type`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_input_type(mut self) -> Self {
                    self.set_input_type();
                    self
                }
                ///Query presence of `output_type`
                #[inline]
                pub const fn r#output_type(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `output_type`
                #[inline]
                pub const fn set_output_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `output_type`
                #[inline]
                pub const fn clear_output_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `output_type`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_output_type(mut self) -> Self {
                    self.set_output_type();
                    self
                }
                ///Query presence of `options`
                #[inline]
                pub const fn r#options(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `options`
                #[inline]
                pub const fn set_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `options`
                #[inline]
                pub const fn clear_options(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `options`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_options(mut self) -> Self {
                    self.set_options();
                    self
                }
                ///Query presence of `client_streaming`
                #[inline]
                pub const fn r#client_streaming(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `client_streaming`
                #[inline]
                pub const fn set_client_streaming(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `client_streaming`
                #[inline]
                pub const fn clear_client_streaming(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `client_streaming`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_client_streaming(mut self) -> Self {
                    self.set_client_streaming();
                    self
                }
                ///Query presence of `server_streaming`
                #[inline]
                pub const fn r#server_streaming(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `server_streaming`
                #[inline]
                pub const fn set_server_streaming(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `server_streaming`
                #[inline]
                pub const fn clear_server_streaming(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `server_streaming`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_server_streaming(mut self) -> Self {
                    self.set_server_streaming();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct MethodDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#input_type: ::std::string::String,
            pub r#output_type: ::std::string::String,
            pub r#options: MethodOptions,
            pub r#client_streaming: bool,
            pub r#server_streaming: bool,
            pub _has: MethodDescriptorProto_::_Hazzer,
        }
        impl ::core::default::Default for MethodDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#input_type: ::core::default::Default::default(),
                    r#output_type: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    r#client_streaming: false as _,
                    r#server_streaming: false as _,
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl MethodDescriptorProto {
            ///Return a reference to `name` as an `Option`
            #[inline]
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            ///Return a reference to `input_type` as an `Option`
            #[inline]
            pub fn r#input_type(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#input_type().then_some(&self.r#input_type)
            }
            ///Return a reference to `output_type` as an `Option`
            #[inline]
            pub fn r#output_type(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#output_type().then_some(&self.r#output_type)
            }
            ///Return a reference to `options` as an `Option`
            #[inline]
            pub fn r#options(&self) -> ::core::option::Option<&MethodOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            ///Return a reference to `client_streaming` as an `Option`
            #[inline]
            pub fn r#client_streaming(&self) -> ::core::option::Option<&bool> {
                self._has.r#client_streaming().then_some(&self.r#client_streaming)
            }
            ///Return a reference to `server_streaming` as an `Option`
            #[inline]
            pub fn r#server_streaming(&self) -> ::core::option::Option<&bool> {
                self._has.r#server_streaming().then_some(&self.r#server_streaming)
            }
        }
        impl ::micropb::MessageDecode for MethodDescriptorProto {
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
                            let mut_ref = &mut self.r#name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_name();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#input_type;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_input_type();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#output_type;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_output_type();
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#client_streaming;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_client_streaming();
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#server_streaming;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_server_streaming();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FileOptions_ {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct OptimizeMode(pub i32);
            impl OptimizeMode {
                pub const Speed: Self = Self(1);
                pub const CodeSize: Self = Self(2);
                pub const LiteRuntime: Self = Self(3);
            }
            impl core::default::Default for OptimizeMode {
                fn default() -> Self {
                    Self(1)
                }
            }
            impl core::convert::From<i32> for OptimizeMode {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 3]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 3])
                }
                ///Query presence of `java_package`
                #[inline]
                pub const fn r#java_package(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `java_package`
                #[inline]
                pub const fn set_java_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `java_package`
                #[inline]
                pub const fn clear_java_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `java_package`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_package(mut self) -> Self {
                    self.set_java_package();
                    self
                }
                ///Query presence of `java_outer_classname`
                #[inline]
                pub const fn r#java_outer_classname(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `java_outer_classname`
                #[inline]
                pub const fn set_java_outer_classname(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `java_outer_classname`
                #[inline]
                pub const fn clear_java_outer_classname(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `java_outer_classname`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_outer_classname(mut self) -> Self {
                    self.set_java_outer_classname();
                    self
                }
                ///Query presence of `java_multiple_files`
                #[inline]
                pub const fn r#java_multiple_files(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `java_multiple_files`
                #[inline]
                pub const fn set_java_multiple_files(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `java_multiple_files`
                #[inline]
                pub const fn clear_java_multiple_files(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `java_multiple_files`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_multiple_files(mut self) -> Self {
                    self.set_java_multiple_files();
                    self
                }
                ///Query presence of `java_generate_equals_and_hash`
                #[inline]
                pub const fn r#java_generate_equals_and_hash(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `java_generate_equals_and_hash`
                #[inline]
                pub const fn set_java_generate_equals_and_hash(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `java_generate_equals_and_hash`
                #[inline]
                pub const fn clear_java_generate_equals_and_hash(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `java_generate_equals_and_hash`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_generate_equals_and_hash(mut self) -> Self {
                    self.set_java_generate_equals_and_hash();
                    self
                }
                ///Query presence of `java_string_check_utf8`
                #[inline]
                pub const fn r#java_string_check_utf8(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `java_string_check_utf8`
                #[inline]
                pub const fn set_java_string_check_utf8(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `java_string_check_utf8`
                #[inline]
                pub const fn clear_java_string_check_utf8(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `java_string_check_utf8`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_string_check_utf8(mut self) -> Self {
                    self.set_java_string_check_utf8();
                    self
                }
                ///Query presence of `optimize_for`
                #[inline]
                pub const fn r#optimize_for(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `optimize_for`
                #[inline]
                pub const fn set_optimize_for(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `optimize_for`
                #[inline]
                pub const fn clear_optimize_for(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `optimize_for`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_optimize_for(mut self) -> Self {
                    self.set_optimize_for();
                    self
                }
                ///Query presence of `go_package`
                #[inline]
                pub const fn r#go_package(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                ///Set presence of `go_package`
                #[inline]
                pub const fn set_go_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 64;
                    self
                }
                ///Clear presence of `go_package`
                #[inline]
                pub const fn clear_go_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !64;
                    self
                }
                ///Builder method that sets the presence of `go_package`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_go_package(mut self) -> Self {
                    self.set_go_package();
                    self
                }
                ///Query presence of `cc_generic_services`
                #[inline]
                pub const fn r#cc_generic_services(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                ///Set presence of `cc_generic_services`
                #[inline]
                pub const fn set_cc_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 128;
                    self
                }
                ///Clear presence of `cc_generic_services`
                #[inline]
                pub const fn clear_cc_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !128;
                    self
                }
                ///Builder method that sets the presence of `cc_generic_services`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_cc_generic_services(mut self) -> Self {
                    self.set_cc_generic_services();
                    self
                }
                ///Query presence of `java_generic_services`
                #[inline]
                pub const fn r#java_generic_services(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                ///Set presence of `java_generic_services`
                #[inline]
                pub const fn set_java_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `java_generic_services`
                #[inline]
                pub const fn clear_java_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `java_generic_services`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_java_generic_services(mut self) -> Self {
                    self.set_java_generic_services();
                    self
                }
                ///Query presence of `py_generic_services`
                #[inline]
                pub const fn r#py_generic_services(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                ///Set presence of `py_generic_services`
                #[inline]
                pub const fn set_py_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `py_generic_services`
                #[inline]
                pub const fn clear_py_generic_services(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `py_generic_services`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_py_generic_services(mut self) -> Self {
                    self.set_py_generic_services();
                    self
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `cc_enable_arenas`
                #[inline]
                pub const fn r#cc_enable_arenas(&self) -> bool {
                    (self.0[1] & 8) != 0
                }
                ///Set presence of `cc_enable_arenas`
                #[inline]
                pub const fn set_cc_enable_arenas(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `cc_enable_arenas`
                #[inline]
                pub const fn clear_cc_enable_arenas(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `cc_enable_arenas`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_cc_enable_arenas(mut self) -> Self {
                    self.set_cc_enable_arenas();
                    self
                }
                ///Query presence of `objc_class_prefix`
                #[inline]
                pub const fn r#objc_class_prefix(&self) -> bool {
                    (self.0[1] & 16) != 0
                }
                ///Set presence of `objc_class_prefix`
                #[inline]
                pub const fn set_objc_class_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `objc_class_prefix`
                #[inline]
                pub const fn clear_objc_class_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `objc_class_prefix`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_objc_class_prefix(mut self) -> Self {
                    self.set_objc_class_prefix();
                    self
                }
                ///Query presence of `csharp_namespace`
                #[inline]
                pub const fn r#csharp_namespace(&self) -> bool {
                    (self.0[1] & 32) != 0
                }
                ///Set presence of `csharp_namespace`
                #[inline]
                pub const fn set_csharp_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `csharp_namespace`
                #[inline]
                pub const fn clear_csharp_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `csharp_namespace`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_csharp_namespace(mut self) -> Self {
                    self.set_csharp_namespace();
                    self
                }
                ///Query presence of `swift_prefix`
                #[inline]
                pub const fn r#swift_prefix(&self) -> bool {
                    (self.0[1] & 64) != 0
                }
                ///Set presence of `swift_prefix`
                #[inline]
                pub const fn set_swift_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 64;
                    self
                }
                ///Clear presence of `swift_prefix`
                #[inline]
                pub const fn clear_swift_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !64;
                    self
                }
                ///Builder method that sets the presence of `swift_prefix`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_swift_prefix(mut self) -> Self {
                    self.set_swift_prefix();
                    self
                }
                ///Query presence of `php_class_prefix`
                #[inline]
                pub const fn r#php_class_prefix(&self) -> bool {
                    (self.0[1] & 128) != 0
                }
                ///Set presence of `php_class_prefix`
                #[inline]
                pub const fn set_php_class_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 128;
                    self
                }
                ///Clear presence of `php_class_prefix`
                #[inline]
                pub const fn clear_php_class_prefix(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !128;
                    self
                }
                ///Builder method that sets the presence of `php_class_prefix`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_php_class_prefix(mut self) -> Self {
                    self.set_php_class_prefix();
                    self
                }
                ///Query presence of `php_namespace`
                #[inline]
                pub const fn r#php_namespace(&self) -> bool {
                    (self.0[2] & 1) != 0
                }
                ///Set presence of `php_namespace`
                #[inline]
                pub const fn set_php_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `php_namespace`
                #[inline]
                pub const fn clear_php_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `php_namespace`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_php_namespace(mut self) -> Self {
                    self.set_php_namespace();
                    self
                }
                ///Query presence of `php_metadata_namespace`
                #[inline]
                pub const fn r#php_metadata_namespace(&self) -> bool {
                    (self.0[2] & 2) != 0
                }
                ///Set presence of `php_metadata_namespace`
                #[inline]
                pub const fn set_php_metadata_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `php_metadata_namespace`
                #[inline]
                pub const fn clear_php_metadata_namespace(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `php_metadata_namespace`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_php_metadata_namespace(mut self) -> Self {
                    self.set_php_metadata_namespace();
                    self
                }
                ///Query presence of `ruby_package`
                #[inline]
                pub const fn r#ruby_package(&self) -> bool {
                    (self.0[2] & 4) != 0
                }
                ///Set presence of `ruby_package`
                #[inline]
                pub const fn set_ruby_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `ruby_package`
                #[inline]
                pub const fn clear_ruby_package(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `ruby_package`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_ruby_package(mut self) -> Self {
                    self.set_ruby_package();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[2] & 8) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[2];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct FileOptions {
            pub r#java_package: ::std::string::String,
            pub r#java_outer_classname: ::std::string::String,
            pub r#java_multiple_files: bool,
            pub r#java_generate_equals_and_hash: bool,
            pub r#java_string_check_utf8: bool,
            pub r#optimize_for: FileOptions_::OptimizeMode,
            pub r#go_package: ::std::string::String,
            pub r#cc_generic_services: bool,
            pub r#java_generic_services: bool,
            pub r#py_generic_services: bool,
            pub r#deprecated: bool,
            pub r#cc_enable_arenas: bool,
            pub r#objc_class_prefix: ::std::string::String,
            pub r#csharp_namespace: ::std::string::String,
            pub r#swift_prefix: ::std::string::String,
            pub r#php_class_prefix: ::std::string::String,
            pub r#php_namespace: ::std::string::String,
            pub r#php_metadata_namespace: ::std::string::String,
            pub r#ruby_package: ::std::string::String,
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: FileOptions_::_Hazzer,
        }
        impl ::core::default::Default for FileOptions {
            fn default() -> Self {
                Self {
                    r#java_package: ::core::default::Default::default(),
                    r#java_outer_classname: ::core::default::Default::default(),
                    r#java_multiple_files: false as _,
                    r#java_generate_equals_and_hash: ::core::default::Default::default(),
                    r#java_string_check_utf8: false as _,
                    r#optimize_for: FileOptions_::OptimizeMode::Speed,
                    r#go_package: ::core::default::Default::default(),
                    r#cc_generic_services: false as _,
                    r#java_generic_services: false as _,
                    r#py_generic_services: false as _,
                    r#deprecated: false as _,
                    r#cc_enable_arenas: true as _,
                    r#objc_class_prefix: ::core::default::Default::default(),
                    r#csharp_namespace: ::core::default::Default::default(),
                    r#swift_prefix: ::core::default::Default::default(),
                    r#php_class_prefix: ::core::default::Default::default(),
                    r#php_namespace: ::core::default::Default::default(),
                    r#php_metadata_namespace: ::core::default::Default::default(),
                    r#ruby_package: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FileOptions {
            ///Return a reference to `java_package` as an `Option`
            #[inline]
            pub fn r#java_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#java_package().then_some(&self.r#java_package)
            }
            ///Return a reference to `java_outer_classname` as an `Option`
            #[inline]
            pub fn r#java_outer_classname(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has
                    .r#java_outer_classname()
                    .then_some(&self.r#java_outer_classname)
            }
            ///Return a reference to `java_multiple_files` as an `Option`
            #[inline]
            pub fn r#java_multiple_files(&self) -> ::core::option::Option<&bool> {
                self._has.r#java_multiple_files().then_some(&self.r#java_multiple_files)
            }
            ///Return a reference to `java_generate_equals_and_hash` as an `Option`
            #[inline]
            pub fn r#java_generate_equals_and_hash(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_generate_equals_and_hash()
                    .then_some(&self.r#java_generate_equals_and_hash)
            }
            ///Return a reference to `java_string_check_utf8` as an `Option`
            #[inline]
            pub fn r#java_string_check_utf8(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_string_check_utf8()
                    .then_some(&self.r#java_string_check_utf8)
            }
            ///Return a reference to `optimize_for` as an `Option`
            #[inline]
            pub fn r#optimize_for(
                &self,
            ) -> ::core::option::Option<&FileOptions_::OptimizeMode> {
                self._has.r#optimize_for().then_some(&self.r#optimize_for)
            }
            ///Return a reference to `go_package` as an `Option`
            #[inline]
            pub fn r#go_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#go_package().then_some(&self.r#go_package)
            }
            ///Return a reference to `cc_generic_services` as an `Option`
            #[inline]
            pub fn r#cc_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has.r#cc_generic_services().then_some(&self.r#cc_generic_services)
            }
            ///Return a reference to `java_generic_services` as an `Option`
            #[inline]
            pub fn r#java_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_generic_services()
                    .then_some(&self.r#java_generic_services)
            }
            ///Return a reference to `py_generic_services` as an `Option`
            #[inline]
            pub fn r#py_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has.r#py_generic_services().then_some(&self.r#py_generic_services)
            }
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Return a reference to `cc_enable_arenas` as an `Option`
            #[inline]
            pub fn r#cc_enable_arenas(&self) -> ::core::option::Option<&bool> {
                self._has.r#cc_enable_arenas().then_some(&self.r#cc_enable_arenas)
            }
            ///Return a reference to `objc_class_prefix` as an `Option`
            #[inline]
            pub fn r#objc_class_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#objc_class_prefix().then_some(&self.r#objc_class_prefix)
            }
            ///Return a reference to `csharp_namespace` as an `Option`
            #[inline]
            pub fn r#csharp_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#csharp_namespace().then_some(&self.r#csharp_namespace)
            }
            ///Return a reference to `swift_prefix` as an `Option`
            #[inline]
            pub fn r#swift_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#swift_prefix().then_some(&self.r#swift_prefix)
            }
            ///Return a reference to `php_class_prefix` as an `Option`
            #[inline]
            pub fn r#php_class_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#php_class_prefix().then_some(&self.r#php_class_prefix)
            }
            ///Return a reference to `php_namespace` as an `Option`
            #[inline]
            pub fn r#php_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#php_namespace().then_some(&self.r#php_namespace)
            }
            ///Return a reference to `php_metadata_namespace` as an `Option`
            #[inline]
            pub fn r#php_metadata_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has
                    .r#php_metadata_namespace()
                    .then_some(&self.r#php_metadata_namespace)
            }
            ///Return a reference to `ruby_package` as an `Option`
            #[inline]
            pub fn r#ruby_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#ruby_package().then_some(&self.r#ruby_package)
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
        }
        impl ::micropb::MessageDecode for FileOptions {
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
                            let mut_ref = &mut self.r#java_package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_java_package();
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#java_outer_classname;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_java_outer_classname();
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#java_multiple_files;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_multiple_files();
                        }
                        20u32 => {
                            let mut_ref = &mut self.r#java_generate_equals_and_hash;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_generate_equals_and_hash();
                        }
                        27u32 => {
                            let mut_ref = &mut self.r#java_string_check_utf8;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_string_check_utf8();
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#optimize_for;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FileOptions_::OptimizeMode(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_optimize_for();
                        }
                        11u32 => {
                            let mut_ref = &mut self.r#go_package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_go_package();
                        }
                        16u32 => {
                            let mut_ref = &mut self.r#cc_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_cc_generic_services();
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#java_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_generic_services();
                        }
                        18u32 => {
                            let mut_ref = &mut self.r#py_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_py_generic_services();
                        }
                        23u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        31u32 => {
                            let mut_ref = &mut self.r#cc_enable_arenas;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_cc_enable_arenas();
                        }
                        36u32 => {
                            let mut_ref = &mut self.r#objc_class_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_objc_class_prefix();
                        }
                        37u32 => {
                            let mut_ref = &mut self.r#csharp_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_csharp_namespace();
                        }
                        39u32 => {
                            let mut_ref = &mut self.r#swift_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_swift_prefix();
                        }
                        40u32 => {
                            let mut_ref = &mut self.r#php_class_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_class_prefix();
                        }
                        41u32 => {
                            let mut_ref = &mut self.r#php_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_namespace();
                        }
                        44u32 => {
                            let mut_ref = &mut self.r#php_metadata_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_metadata_namespace();
                        }
                        45u32 => {
                            let mut_ref = &mut self.r#ruby_package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_ruby_package();
                        }
                        50u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod MessageOptions_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `message_set_wire_format`
                #[inline]
                pub const fn r#message_set_wire_format(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `message_set_wire_format`
                #[inline]
                pub const fn set_message_set_wire_format(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `message_set_wire_format`
                #[inline]
                pub const fn clear_message_set_wire_format(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `message_set_wire_format`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_message_set_wire_format(mut self) -> Self {
                    self.set_message_set_wire_format();
                    self
                }
                ///Query presence of `no_standard_descriptor_accessor`
                #[inline]
                pub const fn r#no_standard_descriptor_accessor(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `no_standard_descriptor_accessor`
                #[inline]
                pub const fn set_no_standard_descriptor_accessor(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `no_standard_descriptor_accessor`
                #[inline]
                pub const fn clear_no_standard_descriptor_accessor(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `no_standard_descriptor_accessor`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_no_standard_descriptor_accessor(mut self) -> Self {
                    self.set_no_standard_descriptor_accessor();
                    self
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `map_entry`
                #[inline]
                pub const fn r#map_entry(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `map_entry`
                #[inline]
                pub const fn set_map_entry(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `map_entry`
                #[inline]
                pub const fn clear_map_entry(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `map_entry`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_map_entry(mut self) -> Self {
                    self.set_map_entry();
                    self
                }
                ///Query presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn r#deprecated_legacy_json_field_conflicts(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn set_deprecated_legacy_json_field_conflicts(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn clear_deprecated_legacy_json_field_conflicts(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `deprecated_legacy_json_field_conflicts`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated_legacy_json_field_conflicts(
                    mut self,
                ) -> Self {
                    self.set_deprecated_legacy_json_field_conflicts();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct MessageOptions {
            pub r#message_set_wire_format: bool,
            pub r#no_standard_descriptor_accessor: bool,
            pub r#deprecated: bool,
            pub r#map_entry: bool,
            pub r#deprecated_legacy_json_field_conflicts: bool,
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: MessageOptions_::_Hazzer,
        }
        impl ::core::default::Default for MessageOptions {
            fn default() -> Self {
                Self {
                    r#message_set_wire_format: false as _,
                    r#no_standard_descriptor_accessor: false as _,
                    r#deprecated: false as _,
                    r#map_entry: ::core::default::Default::default(),
                    r#deprecated_legacy_json_field_conflicts: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl MessageOptions {
            ///Return a reference to `message_set_wire_format` as an `Option`
            #[inline]
            pub fn r#message_set_wire_format(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#message_set_wire_format()
                    .then_some(&self.r#message_set_wire_format)
            }
            ///Set the value and presence of `message_set_wire_format`
            #[inline]
            pub fn set_message_set_wire_format(&mut self, value: bool) -> &mut Self {
                self._has.set_message_set_wire_format();
                self.r#message_set_wire_format = value.into();
                self
            }
            ///Return a mutable reference to `message_set_wire_format` as an `Option`
            #[inline]
            pub fn mut_message_set_wire_format(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#message_set_wire_format()
                    .then_some(&mut self.r#message_set_wire_format)
            }
            ///Clear the presence of `message_set_wire_format`
            #[inline]
            pub fn clear_message_set_wire_format(&mut self) -> &mut Self {
                self._has.clear_message_set_wire_format();
                self
            }
            ///Take the value of `message_set_wire_format` and clear its presence
            #[inline]
            pub fn take_message_set_wire_format(
                &mut self,
            ) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#message_set_wire_format()
                    .then(|| ::core::mem::take(&mut self.r#message_set_wire_format));
                self._has.clear_message_set_wire_format();
                val
            }
            ///Builder method that sets the value of `message_set_wire_format`. Useful for initializing the message.
            #[inline]
            pub fn init_message_set_wire_format(mut self, value: bool) -> Self {
                self.set_message_set_wire_format(value);
                self
            }
            ///Return a reference to `no_standard_descriptor_accessor` as an `Option`
            #[inline]
            pub fn r#no_standard_descriptor_accessor(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#no_standard_descriptor_accessor()
                    .then_some(&self.r#no_standard_descriptor_accessor)
            }
            ///Set the value and presence of `no_standard_descriptor_accessor`
            #[inline]
            pub fn set_no_standard_descriptor_accessor(
                &mut self,
                value: bool,
            ) -> &mut Self {
                self._has.set_no_standard_descriptor_accessor();
                self.r#no_standard_descriptor_accessor = value.into();
                self
            }
            ///Return a mutable reference to `no_standard_descriptor_accessor` as an `Option`
            #[inline]
            pub fn mut_no_standard_descriptor_accessor(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#no_standard_descriptor_accessor()
                    .then_some(&mut self.r#no_standard_descriptor_accessor)
            }
            ///Clear the presence of `no_standard_descriptor_accessor`
            #[inline]
            pub fn clear_no_standard_descriptor_accessor(&mut self) -> &mut Self {
                self._has.clear_no_standard_descriptor_accessor();
                self
            }
            ///Take the value of `no_standard_descriptor_accessor` and clear its presence
            #[inline]
            pub fn take_no_standard_descriptor_accessor(
                &mut self,
            ) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#no_standard_descriptor_accessor()
                    .then(|| ::core::mem::take(
                        &mut self.r#no_standard_descriptor_accessor,
                    ));
                self._has.clear_no_standard_descriptor_accessor();
                val
            }
            ///Builder method that sets the value of `no_standard_descriptor_accessor`. Useful for initializing the message.
            #[inline]
            pub fn init_no_standard_descriptor_accessor(mut self, value: bool) -> Self {
                self.set_no_standard_descriptor_accessor(value);
                self
            }
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Set the value and presence of `deprecated`
            #[inline]
            pub fn set_deprecated(&mut self, value: bool) -> &mut Self {
                self._has.set_deprecated();
                self.r#deprecated = value.into();
                self
            }
            ///Return a mutable reference to `deprecated` as an `Option`
            #[inline]
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            ///Clear the presence of `deprecated`
            #[inline]
            pub fn clear_deprecated(&mut self) -> &mut Self {
                self._has.clear_deprecated();
                self
            }
            ///Take the value of `deprecated` and clear its presence
            #[inline]
            pub fn take_deprecated(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#deprecated()
                    .then(|| ::core::mem::take(&mut self.r#deprecated));
                self._has.clear_deprecated();
                val
            }
            ///Builder method that sets the value of `deprecated`. Useful for initializing the message.
            #[inline]
            pub fn init_deprecated(mut self, value: bool) -> Self {
                self.set_deprecated(value);
                self
            }
            ///Return a reference to `map_entry` as an `Option`
            #[inline]
            pub fn r#map_entry(&self) -> ::core::option::Option<&bool> {
                self._has.r#map_entry().then_some(&self.r#map_entry)
            }
            ///Set the value and presence of `map_entry`
            #[inline]
            pub fn set_map_entry(&mut self, value: bool) -> &mut Self {
                self._has.set_map_entry();
                self.r#map_entry = value.into();
                self
            }
            ///Return a mutable reference to `map_entry` as an `Option`
            #[inline]
            pub fn mut_map_entry(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#map_entry().then_some(&mut self.r#map_entry)
            }
            ///Clear the presence of `map_entry`
            #[inline]
            pub fn clear_map_entry(&mut self) -> &mut Self {
                self._has.clear_map_entry();
                self
            }
            ///Take the value of `map_entry` and clear its presence
            #[inline]
            pub fn take_map_entry(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#map_entry()
                    .then(|| ::core::mem::take(&mut self.r#map_entry));
                self._has.clear_map_entry();
                val
            }
            ///Builder method that sets the value of `map_entry`. Useful for initializing the message.
            #[inline]
            pub fn init_map_entry(mut self, value: bool) -> Self {
                self.set_map_entry(value);
                self
            }
            ///Return a reference to `deprecated_legacy_json_field_conflicts` as an `Option`
            #[inline]
            pub fn r#deprecated_legacy_json_field_conflicts(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&self.r#deprecated_legacy_json_field_conflicts)
            }
            ///Set the value and presence of `deprecated_legacy_json_field_conflicts`
            #[inline]
            pub fn set_deprecated_legacy_json_field_conflicts(
                &mut self,
                value: bool,
            ) -> &mut Self {
                self._has.set_deprecated_legacy_json_field_conflicts();
                self.r#deprecated_legacy_json_field_conflicts = value.into();
                self
            }
            ///Return a mutable reference to `deprecated_legacy_json_field_conflicts` as an `Option`
            #[inline]
            pub fn mut_deprecated_legacy_json_field_conflicts(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&mut self.r#deprecated_legacy_json_field_conflicts)
            }
            ///Clear the presence of `deprecated_legacy_json_field_conflicts`
            #[inline]
            pub fn clear_deprecated_legacy_json_field_conflicts(&mut self) -> &mut Self {
                self._has.clear_deprecated_legacy_json_field_conflicts();
                self
            }
            ///Take the value of `deprecated_legacy_json_field_conflicts` and clear its presence
            #[inline]
            pub fn take_deprecated_legacy_json_field_conflicts(
                &mut self,
            ) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then(|| ::core::mem::take(
                        &mut self.r#deprecated_legacy_json_field_conflicts,
                    ));
                self._has.clear_deprecated_legacy_json_field_conflicts();
                val
            }
            ///Builder method that sets the value of `deprecated_legacy_json_field_conflicts`. Useful for initializing the message.
            #[inline]
            pub fn init_deprecated_legacy_json_field_conflicts(
                mut self,
                value: bool,
            ) -> Self {
                self.set_deprecated_legacy_json_field_conflicts(value);
                self
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            ///Set the value and presence of `features`
            #[inline]
            pub fn set_features(&mut self, value: FeatureSet) -> &mut Self {
                self._has.set_features();
                self.r#features = value.into();
                self
            }
            ///Return a mutable reference to `features` as an `Option`
            #[inline]
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            ///Clear the presence of `features`
            #[inline]
            pub fn clear_features(&mut self) -> &mut Self {
                self._has.clear_features();
                self
            }
            ///Take the value of `features` and clear its presence
            #[inline]
            pub fn take_features(&mut self) -> ::core::option::Option<FeatureSet> {
                let val = self
                    ._has
                    .r#features()
                    .then(|| ::core::mem::take(&mut self.r#features));
                self._has.clear_features();
                val
            }
            ///Builder method that sets the value of `features`. Useful for initializing the message.
            #[inline]
            pub fn init_features(mut self, value: FeatureSet) -> Self {
                self.set_features(value);
                self
            }
        }
        impl ::micropb::MessageDecode for MessageOptions {
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
                            let mut_ref = &mut self.r#message_set_wire_format;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_message_set_wire_format();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#no_standard_descriptor_accessor;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_no_standard_descriptor_accessor();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#map_entry;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_map_entry();
                        }
                        11u32 => {
                            let mut_ref = &mut self
                                .r#deprecated_legacy_json_field_conflicts;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated_legacy_json_field_conflicts();
                        }
                        12u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FieldOptions_ {
            pub mod EditionDefault_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `edition`
                    #[inline]
                    pub const fn r#edition(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `edition`
                    #[inline]
                    pub const fn set_edition(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `edition`
                    #[inline]
                    pub const fn clear_edition(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `edition`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_edition(mut self) -> Self {
                        self.set_edition();
                        self
                    }
                    ///Query presence of `value`
                    #[inline]
                    pub const fn r#value(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `value`
                    #[inline]
                    pub const fn set_value(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `value`
                    #[inline]
                    pub const fn clear_value(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `value`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_value(mut self) -> Self {
                        self.set_value();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct EditionDefault {
                pub r#edition: super::Edition,
                pub r#value: ::std::string::String,
                pub _has: EditionDefault_::_Hazzer,
            }
            impl EditionDefault {
                ///Return a reference to `edition` as an `Option`
                #[inline]
                pub fn r#edition(&self) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition().then_some(&self.r#edition)
                }
                ///Set the value and presence of `edition`
                #[inline]
                pub fn set_edition(&mut self, value: super::Edition) -> &mut Self {
                    self._has.set_edition();
                    self.r#edition = value.into();
                    self
                }
                ///Return a mutable reference to `edition` as an `Option`
                #[inline]
                pub fn mut_edition(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has.r#edition().then_some(&mut self.r#edition)
                }
                ///Clear the presence of `edition`
                #[inline]
                pub fn clear_edition(&mut self) -> &mut Self {
                    self._has.clear_edition();
                    self
                }
                ///Take the value of `edition` and clear its presence
                #[inline]
                pub fn take_edition(
                    &mut self,
                ) -> ::core::option::Option<super::Edition> {
                    let val = self
                        ._has
                        .r#edition()
                        .then(|| ::core::mem::take(&mut self.r#edition));
                    self._has.clear_edition();
                    val
                }
                ///Builder method that sets the value of `edition`. Useful for initializing the message.
                #[inline]
                pub fn init_edition(mut self, value: super::Edition) -> Self {
                    self.set_edition(value);
                    self
                }
                ///Return a reference to `value` as an `Option`
                #[inline]
                pub fn r#value(&self) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#value().then_some(&self.r#value)
                }
                ///Set the value and presence of `value`
                #[inline]
                pub fn set_value(&mut self, value: ::std::string::String) -> &mut Self {
                    self._has.set_value();
                    self.r#value = value.into();
                    self
                }
                ///Return a mutable reference to `value` as an `Option`
                #[inline]
                pub fn mut_value(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#value().then_some(&mut self.r#value)
                }
                ///Clear the presence of `value`
                #[inline]
                pub fn clear_value(&mut self) -> &mut Self {
                    self._has.clear_value();
                    self
                }
                ///Take the value of `value` and clear its presence
                #[inline]
                pub fn take_value(
                    &mut self,
                ) -> ::core::option::Option<::std::string::String> {
                    let val = self
                        ._has
                        .r#value()
                        .then(|| ::core::mem::take(&mut self.r#value));
                    self._has.clear_value();
                    val
                }
                ///Builder method that sets the value of `value`. Useful for initializing the message.
                #[inline]
                pub fn init_value(mut self, value: ::std::string::String) -> Self {
                    self.set_value(value);
                    self
                }
            }
            impl ::micropb::MessageDecode for EditionDefault {
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
                            3u32 => {
                                let mut_ref = &mut self.r#edition;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#value;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_value();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            pub mod FeatureSupport_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `edition_introduced`
                    #[inline]
                    pub const fn r#edition_introduced(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `edition_introduced`
                    #[inline]
                    pub const fn set_edition_introduced(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `edition_introduced`
                    #[inline]
                    pub const fn clear_edition_introduced(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `edition_introduced`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_edition_introduced(mut self) -> Self {
                        self.set_edition_introduced();
                        self
                    }
                    ///Query presence of `edition_deprecated`
                    #[inline]
                    pub const fn r#edition_deprecated(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `edition_deprecated`
                    #[inline]
                    pub const fn set_edition_deprecated(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `edition_deprecated`
                    #[inline]
                    pub const fn clear_edition_deprecated(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `edition_deprecated`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_edition_deprecated(mut self) -> Self {
                        self.set_edition_deprecated();
                        self
                    }
                    ///Query presence of `deprecation_warning`
                    #[inline]
                    pub const fn r#deprecation_warning(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    ///Set presence of `deprecation_warning`
                    #[inline]
                    pub const fn set_deprecation_warning(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 4;
                        self
                    }
                    ///Clear presence of `deprecation_warning`
                    #[inline]
                    pub const fn clear_deprecation_warning(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !4;
                        self
                    }
                    ///Builder method that sets the presence of `deprecation_warning`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_deprecation_warning(mut self) -> Self {
                        self.set_deprecation_warning();
                        self
                    }
                    ///Query presence of `edition_removed`
                    #[inline]
                    pub const fn r#edition_removed(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    ///Set presence of `edition_removed`
                    #[inline]
                    pub const fn set_edition_removed(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 8;
                        self
                    }
                    ///Clear presence of `edition_removed`
                    #[inline]
                    pub const fn clear_edition_removed(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !8;
                        self
                    }
                    ///Builder method that sets the presence of `edition_removed`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_edition_removed(mut self) -> Self {
                        self.set_edition_removed();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct FeatureSupport {
                pub r#edition_introduced: super::Edition,
                pub r#edition_deprecated: super::Edition,
                pub r#deprecation_warning: ::std::string::String,
                pub r#edition_removed: super::Edition,
                pub _has: FeatureSupport_::_Hazzer,
            }
            impl FeatureSupport {
                ///Return a reference to `edition_introduced` as an `Option`
                #[inline]
                pub fn r#edition_introduced(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has
                        .r#edition_introduced()
                        .then_some(&self.r#edition_introduced)
                }
                ///Set the value and presence of `edition_introduced`
                #[inline]
                pub fn set_edition_introduced(
                    &mut self,
                    value: super::Edition,
                ) -> &mut Self {
                    self._has.set_edition_introduced();
                    self.r#edition_introduced = value.into();
                    self
                }
                ///Return a mutable reference to `edition_introduced` as an `Option`
                #[inline]
                pub fn mut_edition_introduced(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has
                        .r#edition_introduced()
                        .then_some(&mut self.r#edition_introduced)
                }
                ///Clear the presence of `edition_introduced`
                #[inline]
                pub fn clear_edition_introduced(&mut self) -> &mut Self {
                    self._has.clear_edition_introduced();
                    self
                }
                ///Take the value of `edition_introduced` and clear its presence
                #[inline]
                pub fn take_edition_introduced(
                    &mut self,
                ) -> ::core::option::Option<super::Edition> {
                    let val = self
                        ._has
                        .r#edition_introduced()
                        .then(|| ::core::mem::take(&mut self.r#edition_introduced));
                    self._has.clear_edition_introduced();
                    val
                }
                ///Builder method that sets the value of `edition_introduced`. Useful for initializing the message.
                #[inline]
                pub fn init_edition_introduced(mut self, value: super::Edition) -> Self {
                    self.set_edition_introduced(value);
                    self
                }
                ///Return a reference to `edition_deprecated` as an `Option`
                #[inline]
                pub fn r#edition_deprecated(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has
                        .r#edition_deprecated()
                        .then_some(&self.r#edition_deprecated)
                }
                ///Set the value and presence of `edition_deprecated`
                #[inline]
                pub fn set_edition_deprecated(
                    &mut self,
                    value: super::Edition,
                ) -> &mut Self {
                    self._has.set_edition_deprecated();
                    self.r#edition_deprecated = value.into();
                    self
                }
                ///Return a mutable reference to `edition_deprecated` as an `Option`
                #[inline]
                pub fn mut_edition_deprecated(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has
                        .r#edition_deprecated()
                        .then_some(&mut self.r#edition_deprecated)
                }
                ///Clear the presence of `edition_deprecated`
                #[inline]
                pub fn clear_edition_deprecated(&mut self) -> &mut Self {
                    self._has.clear_edition_deprecated();
                    self
                }
                ///Take the value of `edition_deprecated` and clear its presence
                #[inline]
                pub fn take_edition_deprecated(
                    &mut self,
                ) -> ::core::option::Option<super::Edition> {
                    let val = self
                        ._has
                        .r#edition_deprecated()
                        .then(|| ::core::mem::take(&mut self.r#edition_deprecated));
                    self._has.clear_edition_deprecated();
                    val
                }
                ///Builder method that sets the value of `edition_deprecated`. Useful for initializing the message.
                #[inline]
                pub fn init_edition_deprecated(mut self, value: super::Edition) -> Self {
                    self.set_edition_deprecated(value);
                    self
                }
                ///Return a reference to `deprecation_warning` as an `Option`
                #[inline]
                pub fn r#deprecation_warning(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has
                        .r#deprecation_warning()
                        .then_some(&self.r#deprecation_warning)
                }
                ///Set the value and presence of `deprecation_warning`
                #[inline]
                pub fn set_deprecation_warning(
                    &mut self,
                    value: ::std::string::String,
                ) -> &mut Self {
                    self._has.set_deprecation_warning();
                    self.r#deprecation_warning = value.into();
                    self
                }
                ///Return a mutable reference to `deprecation_warning` as an `Option`
                #[inline]
                pub fn mut_deprecation_warning(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has
                        .r#deprecation_warning()
                        .then_some(&mut self.r#deprecation_warning)
                }
                ///Clear the presence of `deprecation_warning`
                #[inline]
                pub fn clear_deprecation_warning(&mut self) -> &mut Self {
                    self._has.clear_deprecation_warning();
                    self
                }
                ///Take the value of `deprecation_warning` and clear its presence
                #[inline]
                pub fn take_deprecation_warning(
                    &mut self,
                ) -> ::core::option::Option<::std::string::String> {
                    let val = self
                        ._has
                        .r#deprecation_warning()
                        .then(|| ::core::mem::take(&mut self.r#deprecation_warning));
                    self._has.clear_deprecation_warning();
                    val
                }
                ///Builder method that sets the value of `deprecation_warning`. Useful for initializing the message.
                #[inline]
                pub fn init_deprecation_warning(
                    mut self,
                    value: ::std::string::String,
                ) -> Self {
                    self.set_deprecation_warning(value);
                    self
                }
                ///Return a reference to `edition_removed` as an `Option`
                #[inline]
                pub fn r#edition_removed(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition_removed().then_some(&self.r#edition_removed)
                }
                ///Set the value and presence of `edition_removed`
                #[inline]
                pub fn set_edition_removed(
                    &mut self,
                    value: super::Edition,
                ) -> &mut Self {
                    self._has.set_edition_removed();
                    self.r#edition_removed = value.into();
                    self
                }
                ///Return a mutable reference to `edition_removed` as an `Option`
                #[inline]
                pub fn mut_edition_removed(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has.r#edition_removed().then_some(&mut self.r#edition_removed)
                }
                ///Clear the presence of `edition_removed`
                #[inline]
                pub fn clear_edition_removed(&mut self) -> &mut Self {
                    self._has.clear_edition_removed();
                    self
                }
                ///Take the value of `edition_removed` and clear its presence
                #[inline]
                pub fn take_edition_removed(
                    &mut self,
                ) -> ::core::option::Option<super::Edition> {
                    let val = self
                        ._has
                        .r#edition_removed()
                        .then(|| ::core::mem::take(&mut self.r#edition_removed));
                    self._has.clear_edition_removed();
                    val
                }
                ///Builder method that sets the value of `edition_removed`. Useful for initializing the message.
                #[inline]
                pub fn init_edition_removed(mut self, value: super::Edition) -> Self {
                    self.set_edition_removed(value);
                    self
                }
            }
            impl ::micropb::MessageDecode for FeatureSupport {
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
                                let mut_ref = &mut self.r#edition_introduced;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_introduced();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#edition_deprecated;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_deprecated();
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#deprecation_warning;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_deprecation_warning();
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#edition_removed;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_removed();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct CType(pub i32);
            impl CType {
                pub const String: Self = Self(0);
                pub const Cord: Self = Self(1);
                pub const StringPiece: Self = Self(2);
            }
            impl core::default::Default for CType {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for CType {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct JSType(pub i32);
            impl JSType {
                pub const JsNormal: Self = Self(0);
                pub const JsString: Self = Self(1);
                pub const JsNumber: Self = Self(2);
            }
            impl core::default::Default for JSType {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for JSType {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct OptionRetention(pub i32);
            impl OptionRetention {
                pub const RetentionUnknown: Self = Self(0);
                pub const RetentionRuntime: Self = Self(1);
                pub const RetentionSource: Self = Self(2);
            }
            impl core::default::Default for OptionRetention {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for OptionRetention {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct OptionTargetType(pub i32);
            impl OptionTargetType {
                pub const TargetTypeUnknown: Self = Self(0);
                pub const TargetTypeFile: Self = Self(1);
                pub const TargetTypeExtensionRange: Self = Self(2);
                pub const TargetTypeMessage: Self = Self(3);
                pub const TargetTypeField: Self = Self(4);
                pub const TargetTypeOneof: Self = Self(5);
                pub const TargetTypeEnum: Self = Self(6);
                pub const TargetTypeEnumEntry: Self = Self(7);
                pub const TargetTypeService: Self = Self(8);
                pub const TargetTypeMethod: Self = Self(9);
            }
            impl core::default::Default for OptionTargetType {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for OptionTargetType {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 2]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 2])
                }
                ///Query presence of `ctype`
                #[inline]
                pub const fn r#ctype(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `ctype`
                #[inline]
                pub const fn set_ctype(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `ctype`
                #[inline]
                pub const fn clear_ctype(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `ctype`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_ctype(mut self) -> Self {
                    self.set_ctype();
                    self
                }
                ///Query presence of `packed`
                #[inline]
                pub const fn r#packed(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `packed`
                #[inline]
                pub const fn set_packed(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `packed`
                #[inline]
                pub const fn clear_packed(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `packed`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_packed(mut self) -> Self {
                    self.set_packed();
                    self
                }
                ///Query presence of `jstype`
                #[inline]
                pub const fn r#jstype(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `jstype`
                #[inline]
                pub const fn set_jstype(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `jstype`
                #[inline]
                pub const fn clear_jstype(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `jstype`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_jstype(mut self) -> Self {
                    self.set_jstype();
                    self
                }
                ///Query presence of `lazy`
                #[inline]
                pub const fn r#lazy(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `lazy`
                #[inline]
                pub const fn set_lazy(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `lazy`
                #[inline]
                pub const fn clear_lazy(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `lazy`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_lazy(mut self) -> Self {
                    self.set_lazy();
                    self
                }
                ///Query presence of `unverified_lazy`
                #[inline]
                pub const fn r#unverified_lazy(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `unverified_lazy`
                #[inline]
                pub const fn set_unverified_lazy(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `unverified_lazy`
                #[inline]
                pub const fn clear_unverified_lazy(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `unverified_lazy`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_unverified_lazy(mut self) -> Self {
                    self.set_unverified_lazy();
                    self
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `weak`
                #[inline]
                pub const fn r#weak(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                ///Set presence of `weak`
                #[inline]
                pub const fn set_weak(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 64;
                    self
                }
                ///Clear presence of `weak`
                #[inline]
                pub const fn clear_weak(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !64;
                    self
                }
                ///Builder method that sets the presence of `weak`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_weak(mut self) -> Self {
                    self.set_weak();
                    self
                }
                ///Query presence of `debug_redact`
                #[inline]
                pub const fn r#debug_redact(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                ///Set presence of `debug_redact`
                #[inline]
                pub const fn set_debug_redact(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 128;
                    self
                }
                ///Clear presence of `debug_redact`
                #[inline]
                pub const fn clear_debug_redact(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !128;
                    self
                }
                ///Builder method that sets the presence of `debug_redact`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_debug_redact(mut self) -> Self {
                    self.set_debug_redact();
                    self
                }
                ///Query presence of `retention`
                #[inline]
                pub const fn r#retention(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                ///Set presence of `retention`
                #[inline]
                pub const fn set_retention(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `retention`
                #[inline]
                pub const fn clear_retention(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `retention`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_retention(mut self) -> Self {
                    self.set_retention();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
                ///Query presence of `feature_support`
                #[inline]
                pub const fn r#feature_support(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                ///Set presence of `feature_support`
                #[inline]
                pub const fn set_feature_support(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `feature_support`
                #[inline]
                pub const fn clear_feature_support(&mut self) -> &mut Self {
                    let elem = &mut self.0[1];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `feature_support`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_feature_support(mut self) -> Self {
                    self.set_feature_support();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct FieldOptions {
            pub r#ctype: FieldOptions_::CType,
            pub r#packed: bool,
            pub r#jstype: FieldOptions_::JSType,
            pub r#lazy: bool,
            pub r#unverified_lazy: bool,
            pub r#deprecated: bool,
            pub r#weak: bool,
            pub r#debug_redact: bool,
            pub r#retention: FieldOptions_::OptionRetention,
            pub r#targets: ::std::vec::Vec<FieldOptions_::OptionTargetType>,
            pub r#edition_defaults: ::std::vec::Vec<FieldOptions_::EditionDefault>,
            pub r#features: FeatureSet,
            pub r#feature_support: FieldOptions_::FeatureSupport,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: FieldOptions_::_Hazzer,
        }
        impl ::core::default::Default for FieldOptions {
            fn default() -> Self {
                Self {
                    r#ctype: FieldOptions_::CType::String,
                    r#packed: ::core::default::Default::default(),
                    r#jstype: FieldOptions_::JSType::JsNormal,
                    r#lazy: false as _,
                    r#unverified_lazy: false as _,
                    r#deprecated: false as _,
                    r#weak: false as _,
                    r#debug_redact: false as _,
                    r#retention: ::core::default::Default::default(),
                    r#targets: ::core::default::Default::default(),
                    r#edition_defaults: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#feature_support: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FieldOptions {
            ///Return a reference to `ctype` as an `Option`
            #[inline]
            pub fn r#ctype(&self) -> ::core::option::Option<&FieldOptions_::CType> {
                self._has.r#ctype().then_some(&self.r#ctype)
            }
            ///Set the value and presence of `ctype`
            #[inline]
            pub fn set_ctype(&mut self, value: FieldOptions_::CType) -> &mut Self {
                self._has.set_ctype();
                self.r#ctype = value.into();
                self
            }
            ///Return a mutable reference to `ctype` as an `Option`
            #[inline]
            pub fn mut_ctype(
                &mut self,
            ) -> ::core::option::Option<&mut FieldOptions_::CType> {
                self._has.r#ctype().then_some(&mut self.r#ctype)
            }
            ///Clear the presence of `ctype`
            #[inline]
            pub fn clear_ctype(&mut self) -> &mut Self {
                self._has.clear_ctype();
                self
            }
            ///Take the value of `ctype` and clear its presence
            #[inline]
            pub fn take_ctype(
                &mut self,
            ) -> ::core::option::Option<FieldOptions_::CType> {
                let val = self
                    ._has
                    .r#ctype()
                    .then(|| ::core::mem::take(&mut self.r#ctype));
                self._has.clear_ctype();
                val
            }
            ///Builder method that sets the value of `ctype`. Useful for initializing the message.
            #[inline]
            pub fn init_ctype(mut self, value: FieldOptions_::CType) -> Self {
                self.set_ctype(value);
                self
            }
            ///Return a reference to `packed` as an `Option`
            #[inline]
            pub fn r#packed(&self) -> ::core::option::Option<&bool> {
                self._has.r#packed().then_some(&self.r#packed)
            }
            ///Set the value and presence of `packed`
            #[inline]
            pub fn set_packed(&mut self, value: bool) -> &mut Self {
                self._has.set_packed();
                self.r#packed = value.into();
                self
            }
            ///Return a mutable reference to `packed` as an `Option`
            #[inline]
            pub fn mut_packed(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#packed().then_some(&mut self.r#packed)
            }
            ///Clear the presence of `packed`
            #[inline]
            pub fn clear_packed(&mut self) -> &mut Self {
                self._has.clear_packed();
                self
            }
            ///Take the value of `packed` and clear its presence
            #[inline]
            pub fn take_packed(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#packed()
                    .then(|| ::core::mem::take(&mut self.r#packed));
                self._has.clear_packed();
                val
            }
            ///Builder method that sets the value of `packed`. Useful for initializing the message.
            #[inline]
            pub fn init_packed(mut self, value: bool) -> Self {
                self.set_packed(value);
                self
            }
            ///Return a reference to `jstype` as an `Option`
            #[inline]
            pub fn r#jstype(&self) -> ::core::option::Option<&FieldOptions_::JSType> {
                self._has.r#jstype().then_some(&self.r#jstype)
            }
            ///Set the value and presence of `jstype`
            #[inline]
            pub fn set_jstype(&mut self, value: FieldOptions_::JSType) -> &mut Self {
                self._has.set_jstype();
                self.r#jstype = value.into();
                self
            }
            ///Return a mutable reference to `jstype` as an `Option`
            #[inline]
            pub fn mut_jstype(
                &mut self,
            ) -> ::core::option::Option<&mut FieldOptions_::JSType> {
                self._has.r#jstype().then_some(&mut self.r#jstype)
            }
            ///Clear the presence of `jstype`
            #[inline]
            pub fn clear_jstype(&mut self) -> &mut Self {
                self._has.clear_jstype();
                self
            }
            ///Take the value of `jstype` and clear its presence
            #[inline]
            pub fn take_jstype(
                &mut self,
            ) -> ::core::option::Option<FieldOptions_::JSType> {
                let val = self
                    ._has
                    .r#jstype()
                    .then(|| ::core::mem::take(&mut self.r#jstype));
                self._has.clear_jstype();
                val
            }
            ///Builder method that sets the value of `jstype`. Useful for initializing the message.
            #[inline]
            pub fn init_jstype(mut self, value: FieldOptions_::JSType) -> Self {
                self.set_jstype(value);
                self
            }
            ///Return a reference to `lazy` as an `Option`
            #[inline]
            pub fn r#lazy(&self) -> ::core::option::Option<&bool> {
                self._has.r#lazy().then_some(&self.r#lazy)
            }
            ///Set the value and presence of `lazy`
            #[inline]
            pub fn set_lazy(&mut self, value: bool) -> &mut Self {
                self._has.set_lazy();
                self.r#lazy = value.into();
                self
            }
            ///Return a mutable reference to `lazy` as an `Option`
            #[inline]
            pub fn mut_lazy(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#lazy().then_some(&mut self.r#lazy)
            }
            ///Clear the presence of `lazy`
            #[inline]
            pub fn clear_lazy(&mut self) -> &mut Self {
                self._has.clear_lazy();
                self
            }
            ///Take the value of `lazy` and clear its presence
            #[inline]
            pub fn take_lazy(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#lazy()
                    .then(|| ::core::mem::take(&mut self.r#lazy));
                self._has.clear_lazy();
                val
            }
            ///Builder method that sets the value of `lazy`. Useful for initializing the message.
            #[inline]
            pub fn init_lazy(mut self, value: bool) -> Self {
                self.set_lazy(value);
                self
            }
            ///Return a reference to `unverified_lazy` as an `Option`
            #[inline]
            pub fn r#unverified_lazy(&self) -> ::core::option::Option<&bool> {
                self._has.r#unverified_lazy().then_some(&self.r#unverified_lazy)
            }
            ///Set the value and presence of `unverified_lazy`
            #[inline]
            pub fn set_unverified_lazy(&mut self, value: bool) -> &mut Self {
                self._has.set_unverified_lazy();
                self.r#unverified_lazy = value.into();
                self
            }
            ///Return a mutable reference to `unverified_lazy` as an `Option`
            #[inline]
            pub fn mut_unverified_lazy(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#unverified_lazy().then_some(&mut self.r#unverified_lazy)
            }
            ///Clear the presence of `unverified_lazy`
            #[inline]
            pub fn clear_unverified_lazy(&mut self) -> &mut Self {
                self._has.clear_unverified_lazy();
                self
            }
            ///Take the value of `unverified_lazy` and clear its presence
            #[inline]
            pub fn take_unverified_lazy(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#unverified_lazy()
                    .then(|| ::core::mem::take(&mut self.r#unverified_lazy));
                self._has.clear_unverified_lazy();
                val
            }
            ///Builder method that sets the value of `unverified_lazy`. Useful for initializing the message.
            #[inline]
            pub fn init_unverified_lazy(mut self, value: bool) -> Self {
                self.set_unverified_lazy(value);
                self
            }
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Set the value and presence of `deprecated`
            #[inline]
            pub fn set_deprecated(&mut self, value: bool) -> &mut Self {
                self._has.set_deprecated();
                self.r#deprecated = value.into();
                self
            }
            ///Return a mutable reference to `deprecated` as an `Option`
            #[inline]
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            ///Clear the presence of `deprecated`
            #[inline]
            pub fn clear_deprecated(&mut self) -> &mut Self {
                self._has.clear_deprecated();
                self
            }
            ///Take the value of `deprecated` and clear its presence
            #[inline]
            pub fn take_deprecated(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#deprecated()
                    .then(|| ::core::mem::take(&mut self.r#deprecated));
                self._has.clear_deprecated();
                val
            }
            ///Builder method that sets the value of `deprecated`. Useful for initializing the message.
            #[inline]
            pub fn init_deprecated(mut self, value: bool) -> Self {
                self.set_deprecated(value);
                self
            }
            ///Return a reference to `weak` as an `Option`
            #[inline]
            pub fn r#weak(&self) -> ::core::option::Option<&bool> {
                self._has.r#weak().then_some(&self.r#weak)
            }
            ///Set the value and presence of `weak`
            #[inline]
            pub fn set_weak(&mut self, value: bool) -> &mut Self {
                self._has.set_weak();
                self.r#weak = value.into();
                self
            }
            ///Return a mutable reference to `weak` as an `Option`
            #[inline]
            pub fn mut_weak(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#weak().then_some(&mut self.r#weak)
            }
            ///Clear the presence of `weak`
            #[inline]
            pub fn clear_weak(&mut self) -> &mut Self {
                self._has.clear_weak();
                self
            }
            ///Take the value of `weak` and clear its presence
            #[inline]
            pub fn take_weak(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#weak()
                    .then(|| ::core::mem::take(&mut self.r#weak));
                self._has.clear_weak();
                val
            }
            ///Builder method that sets the value of `weak`. Useful for initializing the message.
            #[inline]
            pub fn init_weak(mut self, value: bool) -> Self {
                self.set_weak(value);
                self
            }
            ///Return a reference to `debug_redact` as an `Option`
            #[inline]
            pub fn r#debug_redact(&self) -> ::core::option::Option<&bool> {
                self._has.r#debug_redact().then_some(&self.r#debug_redact)
            }
            ///Set the value and presence of `debug_redact`
            #[inline]
            pub fn set_debug_redact(&mut self, value: bool) -> &mut Self {
                self._has.set_debug_redact();
                self.r#debug_redact = value.into();
                self
            }
            ///Return a mutable reference to `debug_redact` as an `Option`
            #[inline]
            pub fn mut_debug_redact(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#debug_redact().then_some(&mut self.r#debug_redact)
            }
            ///Clear the presence of `debug_redact`
            #[inline]
            pub fn clear_debug_redact(&mut self) -> &mut Self {
                self._has.clear_debug_redact();
                self
            }
            ///Take the value of `debug_redact` and clear its presence
            #[inline]
            pub fn take_debug_redact(&mut self) -> ::core::option::Option<bool> {
                let val = self
                    ._has
                    .r#debug_redact()
                    .then(|| ::core::mem::take(&mut self.r#debug_redact));
                self._has.clear_debug_redact();
                val
            }
            ///Builder method that sets the value of `debug_redact`. Useful for initializing the message.
            #[inline]
            pub fn init_debug_redact(mut self, value: bool) -> Self {
                self.set_debug_redact(value);
                self
            }
            ///Return a reference to `retention` as an `Option`
            #[inline]
            pub fn r#retention(
                &self,
            ) -> ::core::option::Option<&FieldOptions_::OptionRetention> {
                self._has.r#retention().then_some(&self.r#retention)
            }
            ///Set the value and presence of `retention`
            #[inline]
            pub fn set_retention(
                &mut self,
                value: FieldOptions_::OptionRetention,
            ) -> &mut Self {
                self._has.set_retention();
                self.r#retention = value.into();
                self
            }
            ///Return a mutable reference to `retention` as an `Option`
            #[inline]
            pub fn mut_retention(
                &mut self,
            ) -> ::core::option::Option<&mut FieldOptions_::OptionRetention> {
                self._has.r#retention().then_some(&mut self.r#retention)
            }
            ///Clear the presence of `retention`
            #[inline]
            pub fn clear_retention(&mut self) -> &mut Self {
                self._has.clear_retention();
                self
            }
            ///Take the value of `retention` and clear its presence
            #[inline]
            pub fn take_retention(
                &mut self,
            ) -> ::core::option::Option<FieldOptions_::OptionRetention> {
                let val = self
                    ._has
                    .r#retention()
                    .then(|| ::core::mem::take(&mut self.r#retention));
                self._has.clear_retention();
                val
            }
            ///Builder method that sets the value of `retention`. Useful for initializing the message.
            #[inline]
            pub fn init_retention(
                mut self,
                value: FieldOptions_::OptionRetention,
            ) -> Self {
                self.set_retention(value);
                self
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            ///Set the value and presence of `features`
            #[inline]
            pub fn set_features(&mut self, value: FeatureSet) -> &mut Self {
                self._has.set_features();
                self.r#features = value.into();
                self
            }
            ///Return a mutable reference to `features` as an `Option`
            #[inline]
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            ///Clear the presence of `features`
            #[inline]
            pub fn clear_features(&mut self) -> &mut Self {
                self._has.clear_features();
                self
            }
            ///Take the value of `features` and clear its presence
            #[inline]
            pub fn take_features(&mut self) -> ::core::option::Option<FeatureSet> {
                let val = self
                    ._has
                    .r#features()
                    .then(|| ::core::mem::take(&mut self.r#features));
                self._has.clear_features();
                val
            }
            ///Builder method that sets the value of `features`. Useful for initializing the message.
            #[inline]
            pub fn init_features(mut self, value: FeatureSet) -> Self {
                self.set_features(value);
                self
            }
            ///Return a reference to `feature_support` as an `Option`
            #[inline]
            pub fn r#feature_support(
                &self,
            ) -> ::core::option::Option<&FieldOptions_::FeatureSupport> {
                self._has.r#feature_support().then_some(&self.r#feature_support)
            }
            ///Set the value and presence of `feature_support`
            #[inline]
            pub fn set_feature_support(
                &mut self,
                value: FieldOptions_::FeatureSupport,
            ) -> &mut Self {
                self._has.set_feature_support();
                self.r#feature_support = value.into();
                self
            }
            ///Return a mutable reference to `feature_support` as an `Option`
            #[inline]
            pub fn mut_feature_support(
                &mut self,
            ) -> ::core::option::Option<&mut FieldOptions_::FeatureSupport> {
                self._has.r#feature_support().then_some(&mut self.r#feature_support)
            }
            ///Clear the presence of `feature_support`
            #[inline]
            pub fn clear_feature_support(&mut self) -> &mut Self {
                self._has.clear_feature_support();
                self
            }
            ///Take the value of `feature_support` and clear its presence
            #[inline]
            pub fn take_feature_support(
                &mut self,
            ) -> ::core::option::Option<FieldOptions_::FeatureSupport> {
                let val = self
                    ._has
                    .r#feature_support()
                    .then(|| ::core::mem::take(&mut self.r#feature_support));
                self._has.clear_feature_support();
                val
            }
            ///Builder method that sets the value of `feature_support`. Useful for initializing the message.
            #[inline]
            pub fn init_feature_support(
                mut self,
                value: FieldOptions_::FeatureSupport,
            ) -> Self {
                self.set_feature_support(value);
                self
            }
        }
        impl ::micropb::MessageDecode for FieldOptions {
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
                            let mut_ref = &mut self.r#ctype;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FieldOptions_::CType(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_ctype();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#packed;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_packed();
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#jstype;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FieldOptions_::JSType(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_jstype();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#lazy;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_lazy();
                        }
                        15u32 => {
                            let mut_ref = &mut self.r#unverified_lazy;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_unverified_lazy();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#weak;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_weak();
                        }
                        16u32 => {
                            let mut_ref = &mut self.r#debug_redact;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_debug_redact();
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#retention;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FieldOptions_::OptionRetention(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_retention();
                        }
                        19u32 => {
                            if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                decoder
                                    .decode_packed(
                                        &mut self.r#targets,
                                        |decoder| {
                                            decoder
                                                .decode_int32()
                                                .map(|n| FieldOptions_::OptionTargetType(n as _))
                                                .map(|v| v as _)
                                        },
                                    )?;
                            } else {
                                if let (Err(_), false) = (
                                    self
                                        .r#targets
                                        .pb_push(
                                            decoder
                                                .decode_int32()
                                                .map(|n| FieldOptions_::OptionTargetType(n as _))? as _,
                                        ),
                                    decoder.ignore_repeated_cap_err,
                                ) {
                                    return Err(::micropb::DecodeError::Capacity);
                                }
                            }
                        }
                        20u32 => {
                            let mut val: FieldOptions_::EditionDefault = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#edition_defaults.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        21u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        22u32 => {
                            let mut_ref = &mut self.r#feature_support;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_feature_support();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod OneofOptions_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct OneofOptions {
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: OneofOptions_::_Hazzer,
        }
        impl OneofOptions {
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
        }
        impl ::micropb::MessageDecode for OneofOptions {
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
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod EnumOptions_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `allow_alias`
                #[inline]
                pub const fn r#allow_alias(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `allow_alias`
                #[inline]
                pub const fn set_allow_alias(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `allow_alias`
                #[inline]
                pub const fn clear_allow_alias(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `allow_alias`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_allow_alias(mut self) -> Self {
                    self.set_allow_alias();
                    self
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn r#deprecated_legacy_json_field_conflicts(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn set_deprecated_legacy_json_field_conflicts(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `deprecated_legacy_json_field_conflicts`
                #[inline]
                pub const fn clear_deprecated_legacy_json_field_conflicts(
                    &mut self,
                ) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `deprecated_legacy_json_field_conflicts`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated_legacy_json_field_conflicts(
                    mut self,
                ) -> Self {
                    self.set_deprecated_legacy_json_field_conflicts();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct EnumOptions {
            pub r#allow_alias: bool,
            pub r#deprecated: bool,
            pub r#deprecated_legacy_json_field_conflicts: bool,
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: EnumOptions_::_Hazzer,
        }
        impl ::core::default::Default for EnumOptions {
            fn default() -> Self {
                Self {
                    r#allow_alias: ::core::default::Default::default(),
                    r#deprecated: false as _,
                    r#deprecated_legacy_json_field_conflicts: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl EnumOptions {
            ///Return a reference to `allow_alias` as an `Option`
            #[inline]
            pub fn r#allow_alias(&self) -> ::core::option::Option<&bool> {
                self._has.r#allow_alias().then_some(&self.r#allow_alias)
            }
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Return a reference to `deprecated_legacy_json_field_conflicts` as an `Option`
            #[inline]
            pub fn r#deprecated_legacy_json_field_conflicts(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&self.r#deprecated_legacy_json_field_conflicts)
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
        }
        impl ::micropb::MessageDecode for EnumOptions {
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
                            let mut_ref = &mut self.r#allow_alias;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_allow_alias();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        6u32 => {
                            let mut_ref = &mut self
                                .r#deprecated_legacy_json_field_conflicts;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated_legacy_json_field_conflicts();
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod EnumValueOptions_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
                ///Query presence of `debug_redact`
                #[inline]
                pub const fn r#debug_redact(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `debug_redact`
                #[inline]
                pub const fn set_debug_redact(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `debug_redact`
                #[inline]
                pub const fn clear_debug_redact(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `debug_redact`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_debug_redact(mut self) -> Self {
                    self.set_debug_redact();
                    self
                }
                ///Query presence of `feature_support`
                #[inline]
                pub const fn r#feature_support(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `feature_support`
                #[inline]
                pub const fn set_feature_support(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `feature_support`
                #[inline]
                pub const fn clear_feature_support(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `feature_support`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_feature_support(mut self) -> Self {
                    self.set_feature_support();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct EnumValueOptions {
            pub r#deprecated: bool,
            pub r#features: FeatureSet,
            pub r#debug_redact: bool,
            pub r#feature_support: FieldOptions_::FeatureSupport,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: EnumValueOptions_::_Hazzer,
        }
        impl ::core::default::Default for EnumValueOptions {
            fn default() -> Self {
                Self {
                    r#deprecated: false as _,
                    r#features: ::core::default::Default::default(),
                    r#debug_redact: false as _,
                    r#feature_support: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl EnumValueOptions {
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            ///Return a reference to `debug_redact` as an `Option`
            #[inline]
            pub fn r#debug_redact(&self) -> ::core::option::Option<&bool> {
                self._has.r#debug_redact().then_some(&self.r#debug_redact)
            }
            ///Return a reference to `feature_support` as an `Option`
            #[inline]
            pub fn r#feature_support(
                &self,
            ) -> ::core::option::Option<&FieldOptions_::FeatureSupport> {
                self._has.r#feature_support().then_some(&self.r#feature_support)
            }
        }
        impl ::micropb::MessageDecode for EnumValueOptions {
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
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#debug_redact;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_debug_redact();
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#feature_support;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_feature_support();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod ServiceOptions_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct ServiceOptions {
            pub r#features: FeatureSet,
            pub r#deprecated: bool,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: ServiceOptions_::_Hazzer,
        }
        impl ::core::default::Default for ServiceOptions {
            fn default() -> Self {
                Self {
                    r#features: ::core::default::Default::default(),
                    r#deprecated: false as _,
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl ServiceOptions {
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
        }
        impl ::micropb::MessageDecode for ServiceOptions {
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
                        34u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        33u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod MethodOptions_ {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct IdempotencyLevel(pub i32);
            impl IdempotencyLevel {
                pub const IdempotencyUnknown: Self = Self(0);
                pub const NoSideEffects: Self = Self(1);
                pub const Idempotent: Self = Self(2);
            }
            impl core::default::Default for IdempotencyLevel {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for IdempotencyLevel {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `deprecated`
                #[inline]
                pub const fn r#deprecated(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `deprecated`
                #[inline]
                pub const fn set_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `deprecated`
                #[inline]
                pub const fn clear_deprecated(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `deprecated`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_deprecated(mut self) -> Self {
                    self.set_deprecated();
                    self
                }
                ///Query presence of `idempotency_level`
                #[inline]
                pub const fn r#idempotency_level(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `idempotency_level`
                #[inline]
                pub const fn set_idempotency_level(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `idempotency_level`
                #[inline]
                pub const fn clear_idempotency_level(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `idempotency_level`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_idempotency_level(mut self) -> Self {
                    self.set_idempotency_level();
                    self
                }
                ///Query presence of `features`
                #[inline]
                pub const fn r#features(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `features`
                #[inline]
                pub const fn set_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `features`
                #[inline]
                pub const fn clear_features(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `features`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_features(mut self) -> Self {
                    self.set_features();
                    self
                }
            }
        }
        #[derive(Debug)]
        pub struct MethodOptions {
            pub r#deprecated: bool,
            pub r#idempotency_level: MethodOptions_::IdempotencyLevel,
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: MethodOptions_::_Hazzer,
        }
        impl ::core::default::Default for MethodOptions {
            fn default() -> Self {
                Self {
                    r#deprecated: false as _,
                    r#idempotency_level: MethodOptions_::IdempotencyLevel::IdempotencyUnknown,
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl MethodOptions {
            ///Return a reference to `deprecated` as an `Option`
            #[inline]
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            ///Return a reference to `idempotency_level` as an `Option`
            #[inline]
            pub fn r#idempotency_level(
                &self,
            ) -> ::core::option::Option<&MethodOptions_::IdempotencyLevel> {
                self._has.r#idempotency_level().then_some(&self.r#idempotency_level)
            }
            ///Return a reference to `features` as an `Option`
            #[inline]
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
        }
        impl ::micropb::MessageDecode for MethodOptions {
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
                        33u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated();
                        }
                        34u32 => {
                            let mut_ref = &mut self.r#idempotency_level;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| MethodOptions_::IdempotencyLevel(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_idempotency_level();
                        }
                        35u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features();
                        }
                        999u32 => {
                            let mut val: UninterpretedOption = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#uninterpreted_option.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod UninterpretedOption_ {
            pub mod NamePart_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `name_part`
                    #[inline]
                    pub const fn r#name_part(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `name_part`
                    #[inline]
                    pub const fn set_name_part(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `name_part`
                    #[inline]
                    pub const fn clear_name_part(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `name_part`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_name_part(mut self) -> Self {
                        self.set_name_part();
                        self
                    }
                    ///Query presence of `is_extension`
                    #[inline]
                    pub const fn r#is_extension(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `is_extension`
                    #[inline]
                    pub const fn set_is_extension(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `is_extension`
                    #[inline]
                    pub const fn clear_is_extension(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `is_extension`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_is_extension(mut self) -> Self {
                        self.set_is_extension();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct NamePart {
                pub r#name_part: ::std::string::String,
                pub r#is_extension: bool,
                pub _has: NamePart_::_Hazzer,
            }
            impl NamePart {
                ///Return a reference to `name_part` as an `Option`
                #[inline]
                pub fn r#name_part(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#name_part().then_some(&self.r#name_part)
                }
                ///Return a reference to `is_extension` as an `Option`
                #[inline]
                pub fn r#is_extension(&self) -> ::core::option::Option<&bool> {
                    self._has.r#is_extension().then_some(&self.r#is_extension)
                }
            }
            impl ::micropb::MessageDecode for NamePart {
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
                                let mut_ref = &mut self.r#name_part;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_name_part();
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#is_extension;
                                {
                                    let val = decoder.decode_bool()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_is_extension();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `identifier_value`
                #[inline]
                pub const fn r#identifier_value(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `identifier_value`
                #[inline]
                pub const fn set_identifier_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `identifier_value`
                #[inline]
                pub const fn clear_identifier_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `identifier_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_identifier_value(mut self) -> Self {
                    self.set_identifier_value();
                    self
                }
                ///Query presence of `positive_int_value`
                #[inline]
                pub const fn r#positive_int_value(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `positive_int_value`
                #[inline]
                pub const fn set_positive_int_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `positive_int_value`
                #[inline]
                pub const fn clear_positive_int_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `positive_int_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_positive_int_value(mut self) -> Self {
                    self.set_positive_int_value();
                    self
                }
                ///Query presence of `negative_int_value`
                #[inline]
                pub const fn r#negative_int_value(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `negative_int_value`
                #[inline]
                pub const fn set_negative_int_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `negative_int_value`
                #[inline]
                pub const fn clear_negative_int_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `negative_int_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_negative_int_value(mut self) -> Self {
                    self.set_negative_int_value();
                    self
                }
                ///Query presence of `double_value`
                #[inline]
                pub const fn r#double_value(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `double_value`
                #[inline]
                pub const fn set_double_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `double_value`
                #[inline]
                pub const fn clear_double_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `double_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_double_value(mut self) -> Self {
                    self.set_double_value();
                    self
                }
                ///Query presence of `string_value`
                #[inline]
                pub const fn r#string_value(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `string_value`
                #[inline]
                pub const fn set_string_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `string_value`
                #[inline]
                pub const fn clear_string_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `string_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_string_value(mut self) -> Self {
                    self.set_string_value();
                    self
                }
                ///Query presence of `aggregate_value`
                #[inline]
                pub const fn r#aggregate_value(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `aggregate_value`
                #[inline]
                pub const fn set_aggregate_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `aggregate_value`
                #[inline]
                pub const fn clear_aggregate_value(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `aggregate_value`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_aggregate_value(mut self) -> Self {
                    self.set_aggregate_value();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct UninterpretedOption {
            pub r#name: ::std::vec::Vec<UninterpretedOption_::NamePart>,
            pub r#identifier_value: ::std::string::String,
            pub r#positive_int_value: u64,
            pub r#negative_int_value: i64,
            pub r#double_value: f64,
            pub r#string_value: ::std::vec::Vec<u8>,
            pub r#aggregate_value: ::std::string::String,
            pub _has: UninterpretedOption_::_Hazzer,
        }
        impl UninterpretedOption {
            ///Return a reference to `identifier_value` as an `Option`
            #[inline]
            pub fn r#identifier_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#identifier_value().then_some(&self.r#identifier_value)
            }
            ///Return a reference to `positive_int_value` as an `Option`
            #[inline]
            pub fn r#positive_int_value(&self) -> ::core::option::Option<&u64> {
                self._has.r#positive_int_value().then_some(&self.r#positive_int_value)
            }
            ///Return a reference to `negative_int_value` as an `Option`
            #[inline]
            pub fn r#negative_int_value(&self) -> ::core::option::Option<&i64> {
                self._has.r#negative_int_value().then_some(&self.r#negative_int_value)
            }
            ///Return a reference to `double_value` as an `Option`
            #[inline]
            pub fn r#double_value(&self) -> ::core::option::Option<&f64> {
                self._has.r#double_value().then_some(&self.r#double_value)
            }
            ///Return a reference to `string_value` as an `Option`
            #[inline]
            pub fn r#string_value(
                &self,
            ) -> ::core::option::Option<&::std::vec::Vec<u8>> {
                self._has.r#string_value().then_some(&self.r#string_value)
            }
            ///Return a reference to `aggregate_value` as an `Option`
            #[inline]
            pub fn r#aggregate_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#aggregate_value().then_some(&self.r#aggregate_value)
            }
        }
        impl ::micropb::MessageDecode for UninterpretedOption {
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
                            let mut val: UninterpretedOption_::NamePart = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#name.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#identifier_value;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_identifier_value();
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#positive_int_value;
                            {
                                let val = decoder.decode_varint64()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_positive_int_value();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#negative_int_value;
                            {
                                let val = decoder.decode_int64()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_negative_int_value();
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#double_value;
                            {
                                let val = decoder.decode_double()?;
                                *mut_ref = val as _;
                            };
                            self._has.set_double_value();
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#string_value;
                            {
                                decoder
                                    .decode_bytes(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_string_value();
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#aggregate_value;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_aggregate_value();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FeatureSet_ {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct FieldPresence(pub i32);
            impl FieldPresence {
                pub const Unknown: Self = Self(0);
                pub const Explicit: Self = Self(1);
                pub const Implicit: Self = Self(2);
                pub const LegacyRequired: Self = Self(3);
            }
            impl core::default::Default for FieldPresence {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for FieldPresence {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct EnumType(pub i32);
            impl EnumType {
                pub const Unknown: Self = Self(0);
                pub const Open: Self = Self(1);
                pub const Closed: Self = Self(2);
            }
            impl core::default::Default for EnumType {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for EnumType {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct RepeatedFieldEncoding(pub i32);
            impl RepeatedFieldEncoding {
                pub const Unknown: Self = Self(0);
                pub const Packed: Self = Self(1);
                pub const Expanded: Self = Self(2);
            }
            impl core::default::Default for RepeatedFieldEncoding {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for RepeatedFieldEncoding {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct Utf8Validation(pub i32);
            impl Utf8Validation {
                pub const Unknown: Self = Self(0);
                pub const Verify: Self = Self(2);
                pub const None: Self = Self(3);
            }
            impl core::default::Default for Utf8Validation {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for Utf8Validation {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct MessageEncoding(pub i32);
            impl MessageEncoding {
                pub const Unknown: Self = Self(0);
                pub const LengthPrefixed: Self = Self(1);
                pub const Delimited: Self = Self(2);
            }
            impl core::default::Default for MessageEncoding {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for MessageEncoding {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct JsonFormat(pub i32);
            impl JsonFormat {
                pub const Unknown: Self = Self(0);
                pub const Allow: Self = Self(1);
                pub const LegacyBestEffort: Self = Self(2);
            }
            impl core::default::Default for JsonFormat {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl core::convert::From<i32> for JsonFormat {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `field_presence`
                #[inline]
                pub const fn r#field_presence(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `field_presence`
                #[inline]
                pub const fn set_field_presence(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `field_presence`
                #[inline]
                pub const fn clear_field_presence(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `field_presence`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_field_presence(mut self) -> Self {
                    self.set_field_presence();
                    self
                }
                ///Query presence of `enum_type`
                #[inline]
                pub const fn r#enum_type(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `enum_type`
                #[inline]
                pub const fn set_enum_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `enum_type`
                #[inline]
                pub const fn clear_enum_type(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `enum_type`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_enum_type(mut self) -> Self {
                    self.set_enum_type();
                    self
                }
                ///Query presence of `repeated_field_encoding`
                #[inline]
                pub const fn r#repeated_field_encoding(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                ///Set presence of `repeated_field_encoding`
                #[inline]
                pub const fn set_repeated_field_encoding(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 4;
                    self
                }
                ///Clear presence of `repeated_field_encoding`
                #[inline]
                pub const fn clear_repeated_field_encoding(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !4;
                    self
                }
                ///Builder method that sets the presence of `repeated_field_encoding`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_repeated_field_encoding(mut self) -> Self {
                    self.set_repeated_field_encoding();
                    self
                }
                ///Query presence of `utf8_validation`
                #[inline]
                pub const fn r#utf8_validation(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                ///Set presence of `utf8_validation`
                #[inline]
                pub const fn set_utf8_validation(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 8;
                    self
                }
                ///Clear presence of `utf8_validation`
                #[inline]
                pub const fn clear_utf8_validation(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !8;
                    self
                }
                ///Builder method that sets the presence of `utf8_validation`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_utf8_validation(mut self) -> Self {
                    self.set_utf8_validation();
                    self
                }
                ///Query presence of `message_encoding`
                #[inline]
                pub const fn r#message_encoding(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                ///Set presence of `message_encoding`
                #[inline]
                pub const fn set_message_encoding(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 16;
                    self
                }
                ///Clear presence of `message_encoding`
                #[inline]
                pub const fn clear_message_encoding(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !16;
                    self
                }
                ///Builder method that sets the presence of `message_encoding`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_message_encoding(mut self) -> Self {
                    self.set_message_encoding();
                    self
                }
                ///Query presence of `json_format`
                #[inline]
                pub const fn r#json_format(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                ///Set presence of `json_format`
                #[inline]
                pub const fn set_json_format(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 32;
                    self
                }
                ///Clear presence of `json_format`
                #[inline]
                pub const fn clear_json_format(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !32;
                    self
                }
                ///Builder method that sets the presence of `json_format`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_json_format(mut self) -> Self {
                    self.set_json_format();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct FeatureSet {
            pub r#field_presence: FeatureSet_::FieldPresence,
            pub r#enum_type: FeatureSet_::EnumType,
            pub r#repeated_field_encoding: FeatureSet_::RepeatedFieldEncoding,
            pub r#utf8_validation: FeatureSet_::Utf8Validation,
            pub r#message_encoding: FeatureSet_::MessageEncoding,
            pub r#json_format: FeatureSet_::JsonFormat,
            pub _has: FeatureSet_::_Hazzer,
        }
        impl FeatureSet {
            ///Return a reference to `field_presence` as an `Option`
            #[inline]
            pub fn r#field_presence(
                &self,
            ) -> ::core::option::Option<&FeatureSet_::FieldPresence> {
                self._has.r#field_presence().then_some(&self.r#field_presence)
            }
            ///Return a reference to `enum_type` as an `Option`
            #[inline]
            pub fn r#enum_type(&self) -> ::core::option::Option<&FeatureSet_::EnumType> {
                self._has.r#enum_type().then_some(&self.r#enum_type)
            }
            ///Return a reference to `repeated_field_encoding` as an `Option`
            #[inline]
            pub fn r#repeated_field_encoding(
                &self,
            ) -> ::core::option::Option<&FeatureSet_::RepeatedFieldEncoding> {
                self._has
                    .r#repeated_field_encoding()
                    .then_some(&self.r#repeated_field_encoding)
            }
            ///Return a reference to `utf8_validation` as an `Option`
            #[inline]
            pub fn r#utf8_validation(
                &self,
            ) -> ::core::option::Option<&FeatureSet_::Utf8Validation> {
                self._has.r#utf8_validation().then_some(&self.r#utf8_validation)
            }
            ///Return a reference to `message_encoding` as an `Option`
            #[inline]
            pub fn r#message_encoding(
                &self,
            ) -> ::core::option::Option<&FeatureSet_::MessageEncoding> {
                self._has.r#message_encoding().then_some(&self.r#message_encoding)
            }
            ///Return a reference to `json_format` as an `Option`
            #[inline]
            pub fn r#json_format(
                &self,
            ) -> ::core::option::Option<&FeatureSet_::JsonFormat> {
                self._has.r#json_format().then_some(&self.r#json_format)
            }
        }
        impl ::micropb::MessageDecode for FeatureSet {
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
                            let mut_ref = &mut self.r#field_presence;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::FieldPresence(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_field_presence();
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#enum_type;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::EnumType(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_enum_type();
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#repeated_field_encoding;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::RepeatedFieldEncoding(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_repeated_field_encoding();
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#utf8_validation;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::Utf8Validation(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_utf8_validation();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#message_encoding;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::MessageEncoding(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_message_encoding();
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#json_format;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| FeatureSet_::JsonFormat(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_json_format();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod FeatureSetDefaults_ {
            pub mod FeatureSetEditionDefault_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `edition`
                    #[inline]
                    pub const fn r#edition(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `edition`
                    #[inline]
                    pub const fn set_edition(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `edition`
                    #[inline]
                    pub const fn clear_edition(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `edition`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_edition(mut self) -> Self {
                        self.set_edition();
                        self
                    }
                    ///Query presence of `overridable_features`
                    #[inline]
                    pub const fn r#overridable_features(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `overridable_features`
                    #[inline]
                    pub const fn set_overridable_features(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `overridable_features`
                    #[inline]
                    pub const fn clear_overridable_features(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `overridable_features`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_overridable_features(mut self) -> Self {
                        self.set_overridable_features();
                        self
                    }
                    ///Query presence of `fixed_features`
                    #[inline]
                    pub const fn r#fixed_features(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    ///Set presence of `fixed_features`
                    #[inline]
                    pub const fn set_fixed_features(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 4;
                        self
                    }
                    ///Clear presence of `fixed_features`
                    #[inline]
                    pub const fn clear_fixed_features(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !4;
                        self
                    }
                    ///Builder method that sets the presence of `fixed_features`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_fixed_features(mut self) -> Self {
                        self.set_fixed_features();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct FeatureSetEditionDefault {
                pub r#edition: super::Edition,
                pub r#overridable_features: super::FeatureSet,
                pub r#fixed_features: super::FeatureSet,
                pub _has: FeatureSetEditionDefault_::_Hazzer,
            }
            impl FeatureSetEditionDefault {
                ///Return a reference to `edition` as an `Option`
                #[inline]
                pub fn r#edition(&self) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition().then_some(&self.r#edition)
                }
                ///Return a reference to `overridable_features` as an `Option`
                #[inline]
                pub fn r#overridable_features(
                    &self,
                ) -> ::core::option::Option<&super::FeatureSet> {
                    self._has
                        .r#overridable_features()
                        .then_some(&self.r#overridable_features)
                }
                ///Return a reference to `fixed_features` as an `Option`
                #[inline]
                pub fn r#fixed_features(
                    &self,
                ) -> ::core::option::Option<&super::FeatureSet> {
                    self._has.r#fixed_features().then_some(&self.r#fixed_features)
                }
            }
            impl ::micropb::MessageDecode for FeatureSetEditionDefault {
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
                            3u32 => {
                                let mut_ref = &mut self.r#edition;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition();
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#overridable_features;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_overridable_features();
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#fixed_features;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_fixed_features();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///New hazzer with all fields set to off
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; 1])
                }
                ///Query presence of `minimum_edition`
                #[inline]
                pub const fn r#minimum_edition(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `minimum_edition`
                #[inline]
                pub const fn set_minimum_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                    self
                }
                ///Clear presence of `minimum_edition`
                #[inline]
                pub const fn clear_minimum_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                    self
                }
                ///Builder method that sets the presence of `minimum_edition`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_minimum_edition(mut self) -> Self {
                    self.set_minimum_edition();
                    self
                }
                ///Query presence of `maximum_edition`
                #[inline]
                pub const fn r#maximum_edition(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                ///Set presence of `maximum_edition`
                #[inline]
                pub const fn set_maximum_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem |= 2;
                    self
                }
                ///Clear presence of `maximum_edition`
                #[inline]
                pub const fn clear_maximum_edition(&mut self) -> &mut Self {
                    let elem = &mut self.0[0];
                    *elem &= !2;
                    self
                }
                ///Builder method that sets the presence of `maximum_edition`. Useful for initializing the Hazzer.
                #[inline]
                pub const fn init_maximum_edition(mut self) -> Self {
                    self.set_maximum_edition();
                    self
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct FeatureSetDefaults {
            pub r#defaults: ::std::vec::Vec<
                FeatureSetDefaults_::FeatureSetEditionDefault,
            >,
            pub r#minimum_edition: Edition,
            pub r#maximum_edition: Edition,
            pub _has: FeatureSetDefaults_::_Hazzer,
        }
        impl FeatureSetDefaults {
            ///Return a reference to `minimum_edition` as an `Option`
            #[inline]
            pub fn r#minimum_edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#minimum_edition().then_some(&self.r#minimum_edition)
            }
            ///Return a reference to `maximum_edition` as an `Option`
            #[inline]
            pub fn r#maximum_edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#maximum_edition().then_some(&self.r#maximum_edition)
            }
        }
        impl ::micropb::MessageDecode for FeatureSetDefaults {
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
                            let mut val: FeatureSetDefaults_::FeatureSetEditionDefault = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#defaults.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#minimum_edition;
                            {
                                let val = decoder.decode_int32().map(|n| Edition(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_minimum_edition();
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#maximum_edition;
                            {
                                let val = decoder.decode_int32().map(|n| Edition(n as _))?;
                                *mut_ref = val as _;
                            };
                            self._has.set_maximum_edition();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod SourceCodeInfo_ {
            pub mod Location_ {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `leading_comments`
                    #[inline]
                    pub const fn r#leading_comments(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `leading_comments`
                    #[inline]
                    pub const fn set_leading_comments(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `leading_comments`
                    #[inline]
                    pub const fn clear_leading_comments(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `leading_comments`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_leading_comments(mut self) -> Self {
                        self.set_leading_comments();
                        self
                    }
                    ///Query presence of `trailing_comments`
                    #[inline]
                    pub const fn r#trailing_comments(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `trailing_comments`
                    #[inline]
                    pub const fn set_trailing_comments(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `trailing_comments`
                    #[inline]
                    pub const fn clear_trailing_comments(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `trailing_comments`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_trailing_comments(mut self) -> Self {
                        self.set_trailing_comments();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct Location {
                pub r#path: ::std::vec::Vec<i32>,
                pub r#span: ::std::vec::Vec<i32>,
                pub r#leading_comments: ::std::string::String,
                pub r#trailing_comments: ::std::string::String,
                pub r#leading_detached_comments: ::std::vec::Vec<::std::string::String>,
                pub _has: Location_::_Hazzer,
            }
            impl Location {
                ///Return a reference to `leading_comments` as an `Option`
                #[inline]
                pub fn r#leading_comments(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#leading_comments().then_some(&self.r#leading_comments)
                }
                ///Return a reference to `trailing_comments` as an `Option`
                #[inline]
                pub fn r#trailing_comments(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#trailing_comments().then_some(&self.r#trailing_comments)
                }
            }
            impl ::micropb::MessageDecode for Location {
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
                                if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                    decoder
                                        .decode_packed(
                                            &mut self.r#path,
                                            |decoder| decoder.decode_int32().map(|v| v as _),
                                        )?;
                                } else {
                                    if let (Err(_), false) = (
                                        self.r#path.pb_push(decoder.decode_int32()? as _),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                            }
                            2u32 => {
                                if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                    decoder
                                        .decode_packed(
                                            &mut self.r#span,
                                            |decoder| decoder.decode_int32().map(|v| v as _),
                                        )?;
                                } else {
                                    if let (Err(_), false) = (
                                        self.r#span.pb_push(decoder.decode_int32()? as _),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#leading_comments;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_leading_comments();
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#trailing_comments;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_trailing_comments();
                            }
                            6u32 => {
                                let mut val: ::std::string::String = ::core::default::Default::default();
                                let mut_ref = &mut val;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                if let (Err(_), false) = (
                                    self.r#leading_detached_comments.pb_push(val),
                                    decoder.ignore_repeated_cap_err,
                                ) {
                                    return Err(::micropb::DecodeError::Capacity);
                                }
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct SourceCodeInfo {
            pub r#location: ::std::vec::Vec<SourceCodeInfo_::Location>,
        }
        impl SourceCodeInfo {}
        impl ::micropb::MessageDecode for SourceCodeInfo {
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
                            let mut val: SourceCodeInfo_::Location = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#location.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod GeneratedCodeInfo_ {
            pub mod Annotation_ {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[repr(transparent)]
                pub struct Semantic(pub i32);
                impl Semantic {
                    pub const None: Self = Self(0);
                    pub const Set: Self = Self(1);
                    pub const Alias: Self = Self(2);
                }
                impl core::default::Default for Semantic {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl core::convert::From<i32> for Semantic {
                    fn from(val: i32) -> Self {
                        Self(val)
                    }
                }
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    ///New hazzer with all fields set to off
                    #[inline]
                    pub const fn _new() -> Self {
                        Self([0; 1])
                    }
                    ///Query presence of `source_file`
                    #[inline]
                    pub const fn r#source_file(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    ///Set presence of `source_file`
                    #[inline]
                    pub const fn set_source_file(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 1;
                        self
                    }
                    ///Clear presence of `source_file`
                    #[inline]
                    pub const fn clear_source_file(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !1;
                        self
                    }
                    ///Builder method that sets the presence of `source_file`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_source_file(mut self) -> Self {
                        self.set_source_file();
                        self
                    }
                    ///Query presence of `begin`
                    #[inline]
                    pub const fn r#begin(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    ///Set presence of `begin`
                    #[inline]
                    pub const fn set_begin(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 2;
                        self
                    }
                    ///Clear presence of `begin`
                    #[inline]
                    pub const fn clear_begin(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !2;
                        self
                    }
                    ///Builder method that sets the presence of `begin`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_begin(mut self) -> Self {
                        self.set_begin();
                        self
                    }
                    ///Query presence of `end`
                    #[inline]
                    pub const fn r#end(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    ///Set presence of `end`
                    #[inline]
                    pub const fn set_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 4;
                        self
                    }
                    ///Clear presence of `end`
                    #[inline]
                    pub const fn clear_end(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !4;
                        self
                    }
                    ///Builder method that sets the presence of `end`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_end(mut self) -> Self {
                        self.set_end();
                        self
                    }
                    ///Query presence of `semantic`
                    #[inline]
                    pub const fn r#semantic(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    ///Set presence of `semantic`
                    #[inline]
                    pub const fn set_semantic(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem |= 8;
                        self
                    }
                    ///Clear presence of `semantic`
                    #[inline]
                    pub const fn clear_semantic(&mut self) -> &mut Self {
                        let elem = &mut self.0[0];
                        *elem &= !8;
                        self
                    }
                    ///Builder method that sets the presence of `semantic`. Useful for initializing the Hazzer.
                    #[inline]
                    pub const fn init_semantic(mut self) -> Self {
                        self.set_semantic();
                        self
                    }
                }
            }
            #[derive(Debug, Default)]
            pub struct Annotation {
                pub r#path: ::std::vec::Vec<i32>,
                pub r#source_file: ::std::string::String,
                pub r#begin: i32,
                pub r#end: i32,
                pub r#semantic: Annotation_::Semantic,
                pub _has: Annotation_::_Hazzer,
            }
            impl Annotation {
                ///Return a reference to `source_file` as an `Option`
                #[inline]
                pub fn r#source_file(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#source_file().then_some(&self.r#source_file)
                }
                ///Return a reference to `begin` as an `Option`
                #[inline]
                pub fn r#begin(&self) -> ::core::option::Option<&i32> {
                    self._has.r#begin().then_some(&self.r#begin)
                }
                ///Return a reference to `end` as an `Option`
                #[inline]
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                ///Return a reference to `semantic` as an `Option`
                #[inline]
                pub fn r#semantic(
                    &self,
                ) -> ::core::option::Option<&Annotation_::Semantic> {
                    self._has.r#semantic().then_some(&self.r#semantic)
                }
            }
            impl ::micropb::MessageDecode for Annotation {
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
                                if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                    decoder
                                        .decode_packed(
                                            &mut self.r#path,
                                            |decoder| decoder.decode_int32().map(|v| v as _),
                                        )?;
                                } else {
                                    if let (Err(_), false) = (
                                        self.r#path.pb_push(decoder.decode_int32()? as _),
                                        decoder.ignore_repeated_cap_err,
                                    ) {
                                        return Err(::micropb::DecodeError::Capacity);
                                    }
                                }
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#source_file;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_source_file();
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#begin;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_begin();
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end();
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#semantic;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| Annotation_::Semantic(n as _))?;
                                    *mut_ref = val as _;
                                };
                                self._has.set_semantic();
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
        #[derive(Debug, Default)]
        pub struct GeneratedCodeInfo {
            pub r#annotation: ::std::vec::Vec<GeneratedCodeInfo_::Annotation>,
        }
        impl GeneratedCodeInfo {}
        impl ::micropb::MessageDecode for GeneratedCodeInfo {
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
                            let mut val: GeneratedCodeInfo_::Annotation = ::core::default::Default::default();
                            let mut_ref = &mut val;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            if let (Err(_), false) = (
                                self.r#annotation.pb_push(val),
                                decoder.ignore_repeated_cap_err,
                            ) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct Edition(pub i32);
        impl Edition {
            pub const Unknown: Self = Self(0);
            pub const Legacy: Self = Self(900);
            pub const Proto2: Self = Self(998);
            pub const Proto3: Self = Self(999);
            pub const _2023: Self = Self(1000);
            pub const _2024: Self = Self(1001);
            pub const _1TestOnly: Self = Self(1);
            pub const _2TestOnly: Self = Self(2);
            pub const _99997TestOnly: Self = Self(99997);
            pub const _99998TestOnly: Self = Self(99998);
            pub const _99999TestOnly: Self = Self(99999);
            pub const Max: Self = Self(2147483647);
        }
        impl core::default::Default for Edition {
            fn default() -> Self {
                Self(0)
            }
        }
        impl core::convert::From<i32> for Edition {
            fn from(val: i32) -> Self {
                Self(val)
            }
        }
    }
}
