pub mod r#google {
    pub mod r#protobuf {
        pub mod mod_FileDescriptorSet {}
        #[derive(Debug)]
        pub struct FileDescriptorSet {
            pub r#file: ::std::vec::Vec<FileDescriptorProto>,
        }
        impl ::core::default::Default for FileDescriptorSet {
            fn default() -> Self {
                Self {
                    r#file: ::core::default::Default::default(),
                }
            }
        }
        impl FileDescriptorSet {}
        impl ::micropb::MessageDecode for FileDescriptorSet {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
        pub mod mod_FileDescriptorProto {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#package(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_package(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#source_code_info(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_source_code_info(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#syntax(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_syntax(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#edition(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_edition(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
            }
        }
        #[derive(Debug)]
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
            pub _has: mod_FileDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for FileDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#package: ::core::default::Default::default(),
                    r#dependency: ::core::default::Default::default(),
                    r#public_dependency: ::core::default::Default::default(),
                    r#weak_dependency: ::core::default::Default::default(),
                    r#message_type: ::core::default::Default::default(),
                    r#enum_type: ::core::default::Default::default(),
                    r#service: ::core::default::Default::default(),
                    r#extension: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    r#source_code_info: ::core::default::Default::default(),
                    r#syntax: ::core::default::Default::default(),
                    r#edition: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FileDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#package(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#package().then_some(&self.r#package)
            }
            pub fn mut_package(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#package().then_some(&mut self.r#package)
            }
            pub fn set_package(&mut self, value: ::std::string::String) {
                self._has.set_package(true);
                self.r#package = value.into();
            }
            pub fn clear_package(&mut self) {
                self._has.set_package(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&FileOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut FileOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: FileOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
            pub fn r#source_code_info(&self) -> ::core::option::Option<&SourceCodeInfo> {
                self._has.r#source_code_info().then_some(&self.r#source_code_info)
            }
            pub fn mut_source_code_info(
                &mut self,
            ) -> ::core::option::Option<&mut SourceCodeInfo> {
                self._has.r#source_code_info().then_some(&mut self.r#source_code_info)
            }
            pub fn set_source_code_info(&mut self, value: SourceCodeInfo) {
                self._has.set_source_code_info(true);
                self.r#source_code_info = value.into();
            }
            pub fn clear_source_code_info(&mut self) {
                self._has.set_source_code_info(false);
            }
            pub fn r#syntax(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#syntax().then_some(&self.r#syntax)
            }
            pub fn mut_syntax(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#syntax().then_some(&mut self.r#syntax)
            }
            pub fn set_syntax(&mut self, value: ::std::string::String) {
                self._has.set_syntax(true);
                self.r#syntax = value.into();
            }
            pub fn clear_syntax(&mut self) {
                self._has.set_syntax(false);
            }
            pub fn r#edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#edition().then_some(&self.r#edition)
            }
            pub fn mut_edition(&mut self) -> ::core::option::Option<&mut Edition> {
                self._has.r#edition().then_some(&mut self.r#edition)
            }
            pub fn set_edition(&mut self, value: Edition) {
                self._has.set_edition(true);
                self.r#edition = value.into();
            }
            pub fn clear_edition(&mut self) {
                self._has.set_edition(false);
            }
        }
        impl ::micropb::MessageDecode for FileDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_package(true);
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
                            self._has.set_options(true);
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#source_code_info;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_source_code_info(true);
                        }
                        12u32 => {
                            let mut_ref = &mut self.r#syntax;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_syntax(true);
                        }
                        14u32 => {
                            let mut_ref = &mut self.r#edition;
                            {
                                let val = decoder.decode_int32().map(|n| Edition(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_edition(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_DescriptorProto {
            pub mod mod_ExtensionRange {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_start(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_end(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                    #[inline]
                    pub fn r#options(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    #[inline]
                    pub fn set_options(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 4;
                        } else {
                            *elem &= !4;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct ExtensionRange {
                pub r#start: i32,
                pub r#end: i32,
                pub r#options: super::ExtensionRangeOptions,
                pub _has: mod_ExtensionRange::_Hazzer,
            }
            impl ::core::default::Default for ExtensionRange {
                fn default() -> Self {
                    Self {
                        r#start: ::core::default::Default::default(),
                        r#end: ::core::default::Default::default(),
                        r#options: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl ExtensionRange {
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                pub fn mut_start(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#start().then_some(&mut self.r#start)
                }
                pub fn set_start(&mut self, value: i32) {
                    self._has.set_start(true);
                    self.r#start = value.into();
                }
                pub fn clear_start(&mut self) {
                    self._has.set_start(false);
                }
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                pub fn set_end(&mut self, value: i32) {
                    self._has.set_end(true);
                    self.r#end = value.into();
                }
                pub fn clear_end(&mut self) {
                    self._has.set_end(false);
                }
                pub fn r#options(
                    &self,
                ) -> ::core::option::Option<&super::ExtensionRangeOptions> {
                    self._has.r#options().then_some(&self.r#options)
                }
                pub fn mut_options(
                    &mut self,
                ) -> ::core::option::Option<&mut super::ExtensionRangeOptions> {
                    self._has.r#options().then_some(&mut self.r#options)
                }
                pub fn set_options(&mut self, value: super::ExtensionRangeOptions) {
                    self._has.set_options(true);
                    self.r#options = value.into();
                }
                pub fn clear_options(&mut self) {
                    self._has.set_options(false);
                }
            }
            impl ::micropb::MessageDecode for ExtensionRange {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                    let before = decoder.bytes_read();
                    while decoder.bytes_read() - before < len {
                        let tag = decoder.decode_tag()?;
                        match tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            1u32 => {
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end(true);
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#options;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_options(true);
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            pub mod mod_ReservedRange {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_start(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_end(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct ReservedRange {
                pub r#start: i32,
                pub r#end: i32,
                pub _has: mod_ReservedRange::_Hazzer,
            }
            impl ::core::default::Default for ReservedRange {
                fn default() -> Self {
                    Self {
                        r#start: ::core::default::Default::default(),
                        r#end: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl ReservedRange {
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                pub fn mut_start(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#start().then_some(&mut self.r#start)
                }
                pub fn set_start(&mut self, value: i32) {
                    self._has.set_start(true);
                    self.r#start = value.into();
                }
                pub fn clear_start(&mut self) {
                    self._has.set_start(false);
                }
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                pub fn set_end(&mut self, value: i32) {
                    self._has.set_end(true);
                    self.r#end = value.into();
                }
                pub fn clear_end(&mut self) {
                    self._has.set_end(false);
                }
            }
            impl ::micropb::MessageDecode for ReservedRange {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                    let before = decoder.bytes_read();
                    while decoder.bytes_read() - before < len {
                        let tag = decoder.decode_tag()?;
                        match tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            1u32 => {
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end(true);
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
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct DescriptorProto {
            pub r#name: ::std::string::String,
            pub r#field: ::std::vec::Vec<FieldDescriptorProto>,
            pub r#extension: ::std::vec::Vec<FieldDescriptorProto>,
            pub r#nested_type: ::std::vec::Vec<DescriptorProto>,
            pub r#enum_type: ::std::vec::Vec<EnumDescriptorProto>,
            pub r#extension_range: ::std::vec::Vec<mod_DescriptorProto::ExtensionRange>,
            pub r#oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,
            pub r#options: MessageOptions,
            pub r#reserved_range: ::std::vec::Vec<mod_DescriptorProto::ReservedRange>,
            pub r#reserved_name: ::std::vec::Vec<::std::string::String>,
            pub _has: mod_DescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for DescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#field: ::core::default::Default::default(),
                    r#extension: ::core::default::Default::default(),
                    r#nested_type: ::core::default::Default::default(),
                    r#enum_type: ::core::default::Default::default(),
                    r#extension_range: ::core::default::Default::default(),
                    r#oneof_decl: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    r#reserved_range: ::core::default::Default::default(),
                    r#reserved_name: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl DescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&MessageOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(
                &mut self,
            ) -> ::core::option::Option<&mut MessageOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: MessageOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
        }
        impl ::micropb::MessageDecode for DescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
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
                            let mut val: mod_DescriptorProto::ExtensionRange = ::core::default::Default::default();
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
                            self._has.set_options(true);
                        }
                        9u32 => {
                            let mut val: mod_DescriptorProto::ReservedRange = ::core::default::Default::default();
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
        pub mod mod_ExtensionRangeOptions {
            pub mod mod_Declaration {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#number(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_number(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#full_name(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_full_name(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                    #[inline]
                    pub fn r#type(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    #[inline]
                    pub fn set_type(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 4;
                        } else {
                            *elem &= !4;
                        }
                    }
                    #[inline]
                    pub fn r#reserved(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    #[inline]
                    pub fn set_reserved(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 8;
                        } else {
                            *elem &= !8;
                        }
                    }
                    #[inline]
                    pub fn r#repeated(&self) -> bool {
                        (self.0[0] & 16) != 0
                    }
                    #[inline]
                    pub fn set_repeated(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 16;
                        } else {
                            *elem &= !16;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct Declaration {
                pub r#number: i32,
                pub r#full_name: ::std::string::String,
                pub r#type: ::std::string::String,
                pub r#reserved: bool,
                pub r#repeated: bool,
                pub _has: mod_Declaration::_Hazzer,
            }
            impl ::core::default::Default for Declaration {
                fn default() -> Self {
                    Self {
                        r#number: ::core::default::Default::default(),
                        r#full_name: ::core::default::Default::default(),
                        r#type: ::core::default::Default::default(),
                        r#reserved: ::core::default::Default::default(),
                        r#repeated: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl Declaration {
                pub fn r#number(&self) -> ::core::option::Option<&i32> {
                    self._has.r#number().then_some(&self.r#number)
                }
                pub fn mut_number(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#number().then_some(&mut self.r#number)
                }
                pub fn set_number(&mut self, value: i32) {
                    self._has.set_number(true);
                    self.r#number = value.into();
                }
                pub fn clear_number(&mut self) {
                    self._has.set_number(false);
                }
                pub fn r#full_name(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#full_name().then_some(&self.r#full_name)
                }
                pub fn mut_full_name(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#full_name().then_some(&mut self.r#full_name)
                }
                pub fn set_full_name(&mut self, value: ::std::string::String) {
                    self._has.set_full_name(true);
                    self.r#full_name = value.into();
                }
                pub fn clear_full_name(&mut self) {
                    self._has.set_full_name(false);
                }
                pub fn r#type(&self) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#type().then_some(&self.r#type)
                }
                pub fn mut_type(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#type().then_some(&mut self.r#type)
                }
                pub fn set_type(&mut self, value: ::std::string::String) {
                    self._has.set_type(true);
                    self.r#type = value.into();
                }
                pub fn clear_type(&mut self) {
                    self._has.set_type(false);
                }
                pub fn r#reserved(&self) -> ::core::option::Option<&bool> {
                    self._has.r#reserved().then_some(&self.r#reserved)
                }
                pub fn mut_reserved(&mut self) -> ::core::option::Option<&mut bool> {
                    self._has.r#reserved().then_some(&mut self.r#reserved)
                }
                pub fn set_reserved(&mut self, value: bool) {
                    self._has.set_reserved(true);
                    self.r#reserved = value.into();
                }
                pub fn clear_reserved(&mut self) {
                    self._has.set_reserved(false);
                }
                pub fn r#repeated(&self) -> ::core::option::Option<&bool> {
                    self._has.r#repeated().then_some(&self.r#repeated)
                }
                pub fn mut_repeated(&mut self) -> ::core::option::Option<&mut bool> {
                    self._has.r#repeated().then_some(&mut self.r#repeated)
                }
                pub fn set_repeated(&mut self, value: bool) {
                    self._has.set_repeated(true);
                    self.r#repeated = value.into();
                }
                pub fn clear_repeated(&mut self) {
                    self._has.set_repeated(false);
                }
            }
            impl ::micropb::MessageDecode for Declaration {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                    let before = decoder.bytes_read();
                    while decoder.bytes_read() - before < len {
                        let tag = decoder.decode_tag()?;
                        match tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            1u32 => {
                                let mut_ref = &mut self.r#number;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_number(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#full_name;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_full_name(true);
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#type;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_type(true);
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#reserved;
                                {
                                    let val = decoder.decode_bool()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_reserved(true);
                            }
                            6u32 => {
                                let mut_ref = &mut self.r#repeated;
                                {
                                    let val = decoder.decode_bool()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_repeated(true);
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
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#verification(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_verification(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct ExtensionRangeOptions {
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub r#declaration: ::std::vec::Vec<mod_ExtensionRangeOptions::Declaration>,
            pub r#features: FeatureSet,
            pub r#verification: mod_ExtensionRangeOptions::VerificationState,
            pub _has: mod_ExtensionRangeOptions::_Hazzer,
        }
        impl ::core::default::Default for ExtensionRangeOptions {
            fn default() -> Self {
                Self {
                    r#uninterpreted_option: ::core::default::Default::default(),
                    r#declaration: ::core::default::Default::default(),
                    r#features: ::core::default::Default::default(),
                    r#verification: mod_ExtensionRangeOptions::VerificationState::Unverified,
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl ExtensionRangeOptions {
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
            pub fn r#verification(
                &self,
            ) -> ::core::option::Option<&mod_ExtensionRangeOptions::VerificationState> {
                self._has.r#verification().then_some(&self.r#verification)
            }
            pub fn mut_verification(
                &mut self,
            ) -> ::core::option::Option<
                &mut mod_ExtensionRangeOptions::VerificationState,
            > {
                self._has.r#verification().then_some(&mut self.r#verification)
            }
            pub fn set_verification(
                &mut self,
                value: mod_ExtensionRangeOptions::VerificationState,
            ) {
                self._has.set_verification(true);
                self.r#verification = value.into();
            }
            pub fn clear_verification(&mut self) {
                self._has.set_verification(false);
            }
        }
        impl ::micropb::MessageDecode for ExtensionRangeOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            let mut val: mod_ExtensionRangeOptions::Declaration = ::core::default::Default::default();
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
                            self._has.set_features(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#verification;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_ExtensionRangeOptions::VerificationState(
                                        n as _,
                                    ))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_verification(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_FieldDescriptorProto {
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
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#number(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_number(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#label(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_label(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#type(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_type(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#type_name(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_type_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#extendee(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_extendee(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
                #[inline]
                pub fn r#default_value(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                #[inline]
                pub fn set_default_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 64;
                    } else {
                        *elem &= !64;
                    }
                }
                #[inline]
                pub fn r#oneof_index(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                #[inline]
                pub fn set_oneof_index(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 128;
                    } else {
                        *elem &= !128;
                    }
                }
                #[inline]
                pub fn r#json_name(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                #[inline]
                pub fn set_json_name(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#proto3_optional(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                #[inline]
                pub fn set_proto3_optional(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct FieldDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#number: i32,
            pub r#label: mod_FieldDescriptorProto::Label,
            pub r#type: mod_FieldDescriptorProto::Type,
            pub r#type_name: ::std::string::String,
            pub r#extendee: ::std::string::String,
            pub r#default_value: ::std::string::String,
            pub r#oneof_index: i32,
            pub r#json_name: ::std::string::String,
            pub r#options: FieldOptions,
            pub r#proto3_optional: bool,
            pub _has: mod_FieldDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for FieldDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#number: ::core::default::Default::default(),
                    r#label: ::core::default::Default::default(),
                    r#type: ::core::default::Default::default(),
                    r#type_name: ::core::default::Default::default(),
                    r#extendee: ::core::default::Default::default(),
                    r#default_value: ::core::default::Default::default(),
                    r#oneof_index: ::core::default::Default::default(),
                    r#json_name: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    r#proto3_optional: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FieldDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#number(&self) -> ::core::option::Option<&i32> {
                self._has.r#number().then_some(&self.r#number)
            }
            pub fn mut_number(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#number().then_some(&mut self.r#number)
            }
            pub fn set_number(&mut self, value: i32) {
                self._has.set_number(true);
                self.r#number = value.into();
            }
            pub fn clear_number(&mut self) {
                self._has.set_number(false);
            }
            pub fn r#label(
                &self,
            ) -> ::core::option::Option<&mod_FieldDescriptorProto::Label> {
                self._has.r#label().then_some(&self.r#label)
            }
            pub fn mut_label(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldDescriptorProto::Label> {
                self._has.r#label().then_some(&mut self.r#label)
            }
            pub fn set_label(&mut self, value: mod_FieldDescriptorProto::Label) {
                self._has.set_label(true);
                self.r#label = value.into();
            }
            pub fn clear_label(&mut self) {
                self._has.set_label(false);
            }
            pub fn r#type(
                &self,
            ) -> ::core::option::Option<&mod_FieldDescriptorProto::Type> {
                self._has.r#type().then_some(&self.r#type)
            }
            pub fn mut_type(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldDescriptorProto::Type> {
                self._has.r#type().then_some(&mut self.r#type)
            }
            pub fn set_type(&mut self, value: mod_FieldDescriptorProto::Type) {
                self._has.set_type(true);
                self.r#type = value.into();
            }
            pub fn clear_type(&mut self) {
                self._has.set_type(false);
            }
            pub fn r#type_name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#type_name().then_some(&self.r#type_name)
            }
            pub fn mut_type_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#type_name().then_some(&mut self.r#type_name)
            }
            pub fn set_type_name(&mut self, value: ::std::string::String) {
                self._has.set_type_name(true);
                self.r#type_name = value.into();
            }
            pub fn clear_type_name(&mut self) {
                self._has.set_type_name(false);
            }
            pub fn r#extendee(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#extendee().then_some(&self.r#extendee)
            }
            pub fn mut_extendee(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#extendee().then_some(&mut self.r#extendee)
            }
            pub fn set_extendee(&mut self, value: ::std::string::String) {
                self._has.set_extendee(true);
                self.r#extendee = value.into();
            }
            pub fn clear_extendee(&mut self) {
                self._has.set_extendee(false);
            }
            pub fn r#default_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#default_value().then_some(&self.r#default_value)
            }
            pub fn mut_default_value(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#default_value().then_some(&mut self.r#default_value)
            }
            pub fn set_default_value(&mut self, value: ::std::string::String) {
                self._has.set_default_value(true);
                self.r#default_value = value.into();
            }
            pub fn clear_default_value(&mut self) {
                self._has.set_default_value(false);
            }
            pub fn r#oneof_index(&self) -> ::core::option::Option<&i32> {
                self._has.r#oneof_index().then_some(&self.r#oneof_index)
            }
            pub fn mut_oneof_index(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#oneof_index().then_some(&mut self.r#oneof_index)
            }
            pub fn set_oneof_index(&mut self, value: i32) {
                self._has.set_oneof_index(true);
                self.r#oneof_index = value.into();
            }
            pub fn clear_oneof_index(&mut self) {
                self._has.set_oneof_index(false);
            }
            pub fn r#json_name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#json_name().then_some(&self.r#json_name)
            }
            pub fn mut_json_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#json_name().then_some(&mut self.r#json_name)
            }
            pub fn set_json_name(&mut self, value: ::std::string::String) {
                self._has.set_json_name(true);
                self.r#json_name = value.into();
            }
            pub fn clear_json_name(&mut self) {
                self._has.set_json_name(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&FieldOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut FieldOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: FieldOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
            pub fn r#proto3_optional(&self) -> ::core::option::Option<&bool> {
                self._has.r#proto3_optional().then_some(&self.r#proto3_optional)
            }
            pub fn mut_proto3_optional(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#proto3_optional().then_some(&mut self.r#proto3_optional)
            }
            pub fn set_proto3_optional(&mut self, value: bool) {
                self._has.set_proto3_optional(true);
                self.r#proto3_optional = value.into();
            }
            pub fn clear_proto3_optional(&mut self) {
                self._has.set_proto3_optional(false);
            }
        }
        impl ::micropb::MessageDecode for FieldDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#number;
                            {
                                let val = decoder.decode_int32()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_number(true);
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#label;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FieldDescriptorProto::Label(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_label(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#type;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FieldDescriptorProto::Type(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_type(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#type_name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_type_name(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#extendee;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_extendee(true);
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#default_value;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_default_value(true);
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#oneof_index;
                            {
                                let val = decoder.decode_int32()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_oneof_index(true);
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#json_name;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_json_name(true);
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options(true);
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#proto3_optional;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_proto3_optional(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_OneofDescriptorProto {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct OneofDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#options: OneofOptions,
            pub _has: mod_OneofDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for OneofDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl OneofDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&OneofOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut OneofOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: OneofOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
        }
        impl ::micropb::MessageDecode for OneofDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_EnumDescriptorProto {
            pub mod mod_EnumReservedRange {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#start(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_start(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#end(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_end(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct EnumReservedRange {
                pub r#start: i32,
                pub r#end: i32,
                pub _has: mod_EnumReservedRange::_Hazzer,
            }
            impl ::core::default::Default for EnumReservedRange {
                fn default() -> Self {
                    Self {
                        r#start: ::core::default::Default::default(),
                        r#end: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl EnumReservedRange {
                pub fn r#start(&self) -> ::core::option::Option<&i32> {
                    self._has.r#start().then_some(&self.r#start)
                }
                pub fn mut_start(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#start().then_some(&mut self.r#start)
                }
                pub fn set_start(&mut self, value: i32) {
                    self._has.set_start(true);
                    self.r#start = value.into();
                }
                pub fn clear_start(&mut self) {
                    self._has.set_start(false);
                }
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                pub fn set_end(&mut self, value: i32) {
                    self._has.set_end(true);
                    self.r#end = value.into();
                }
                pub fn clear_end(&mut self) {
                    self._has.set_end(false);
                }
            }
            impl ::micropb::MessageDecode for EnumReservedRange {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                    let before = decoder.bytes_read();
                    while decoder.bytes_read() - before < len {
                        let tag = decoder.decode_tag()?;
                        match tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            1u32 => {
                                let mut_ref = &mut self.r#start;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_start(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end(true);
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
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct EnumDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#value: ::std::vec::Vec<EnumValueDescriptorProto>,
            pub r#options: EnumOptions,
            pub r#reserved_range: ::std::vec::Vec<
                mod_EnumDescriptorProto::EnumReservedRange,
            >,
            pub r#reserved_name: ::std::vec::Vec<::std::string::String>,
            pub _has: mod_EnumDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for EnumDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#value: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    r#reserved_range: ::core::default::Default::default(),
                    r#reserved_name: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl EnumDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&EnumOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut EnumOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: EnumOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
        }
        impl ::micropb::MessageDecode for EnumDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
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
                            self._has.set_options(true);
                        }
                        4u32 => {
                            let mut val: mod_EnumDescriptorProto::EnumReservedRange = ::core::default::Default::default();
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
        pub mod mod_EnumValueDescriptorProto {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#number(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_number(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct EnumValueDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#number: i32,
            pub r#options: EnumValueOptions,
            pub _has: mod_EnumValueDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for EnumValueDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#number: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl EnumValueDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#number(&self) -> ::core::option::Option<&i32> {
                self._has.r#number().then_some(&self.r#number)
            }
            pub fn mut_number(&mut self) -> ::core::option::Option<&mut i32> {
                self._has.r#number().then_some(&mut self.r#number)
            }
            pub fn set_number(&mut self, value: i32) {
                self._has.set_number(true);
                self.r#number = value.into();
            }
            pub fn clear_number(&mut self) {
                self._has.set_number(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&EnumValueOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(
                &mut self,
            ) -> ::core::option::Option<&mut EnumValueOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: EnumValueOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
        }
        impl ::micropb::MessageDecode for EnumValueDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#number;
                            {
                                let val = decoder.decode_int32()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_number(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_ServiceDescriptorProto {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct ServiceDescriptorProto {
            pub r#name: ::std::string::String,
            pub r#method: ::std::vec::Vec<MethodDescriptorProto>,
            pub r#options: ServiceOptions,
            pub _has: mod_ServiceDescriptorProto::_Hazzer,
        }
        impl ::core::default::Default for ServiceDescriptorProto {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#method: ::core::default::Default::default(),
                    r#options: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl ServiceDescriptorProto {
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&ServiceOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(
                &mut self,
            ) -> ::core::option::Option<&mut ServiceOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: ServiceOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
        }
        impl ::micropb::MessageDecode for ServiceDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
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
                            self._has.set_options(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_MethodDescriptorProto {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#name(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_name(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#input_type(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_input_type(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#output_type(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_output_type(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#options(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_options(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#client_streaming(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_client_streaming(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#server_streaming(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_server_streaming(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
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
            pub _has: mod_MethodDescriptorProto::_Hazzer,
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
            pub fn r#name(&self) -> ::core::option::Option<&::std::string::String> {
                self._has.r#name().then_some(&self.r#name)
            }
            pub fn mut_name(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#name().then_some(&mut self.r#name)
            }
            pub fn set_name(&mut self, value: ::std::string::String) {
                self._has.set_name(true);
                self.r#name = value.into();
            }
            pub fn clear_name(&mut self) {
                self._has.set_name(false);
            }
            pub fn r#input_type(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#input_type().then_some(&self.r#input_type)
            }
            pub fn mut_input_type(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#input_type().then_some(&mut self.r#input_type)
            }
            pub fn set_input_type(&mut self, value: ::std::string::String) {
                self._has.set_input_type(true);
                self.r#input_type = value.into();
            }
            pub fn clear_input_type(&mut self) {
                self._has.set_input_type(false);
            }
            pub fn r#output_type(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#output_type().then_some(&self.r#output_type)
            }
            pub fn mut_output_type(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#output_type().then_some(&mut self.r#output_type)
            }
            pub fn set_output_type(&mut self, value: ::std::string::String) {
                self._has.set_output_type(true);
                self.r#output_type = value.into();
            }
            pub fn clear_output_type(&mut self) {
                self._has.set_output_type(false);
            }
            pub fn r#options(&self) -> ::core::option::Option<&MethodOptions> {
                self._has.r#options().then_some(&self.r#options)
            }
            pub fn mut_options(&mut self) -> ::core::option::Option<&mut MethodOptions> {
                self._has.r#options().then_some(&mut self.r#options)
            }
            pub fn set_options(&mut self, value: MethodOptions) {
                self._has.set_options(true);
                self.r#options = value.into();
            }
            pub fn clear_options(&mut self) {
                self._has.set_options(false);
            }
            pub fn r#client_streaming(&self) -> ::core::option::Option<&bool> {
                self._has.r#client_streaming().then_some(&self.r#client_streaming)
            }
            pub fn mut_client_streaming(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#client_streaming().then_some(&mut self.r#client_streaming)
            }
            pub fn set_client_streaming(&mut self, value: bool) {
                self._has.set_client_streaming(true);
                self.r#client_streaming = value.into();
            }
            pub fn clear_client_streaming(&mut self) {
                self._has.set_client_streaming(false);
            }
            pub fn r#server_streaming(&self) -> ::core::option::Option<&bool> {
                self._has.r#server_streaming().then_some(&self.r#server_streaming)
            }
            pub fn mut_server_streaming(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#server_streaming().then_some(&mut self.r#server_streaming)
            }
            pub fn set_server_streaming(&mut self, value: bool) {
                self._has.set_server_streaming(true);
                self.r#server_streaming = value.into();
            }
            pub fn clear_server_streaming(&mut self) {
                self._has.set_server_streaming(false);
            }
        }
        impl ::micropb::MessageDecode for MethodDescriptorProto {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_name(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#input_type;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_input_type(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#output_type;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_output_type(true);
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#options;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_options(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#client_streaming;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_client_streaming(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#server_streaming;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_server_streaming(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_FileOptions {
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
                #[inline]
                pub fn r#java_package(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_java_package(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#java_outer_classname(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_java_outer_classname(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#java_multiple_files(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_java_multiple_files(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#java_generate_equals_and_hash(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_java_generate_equals_and_hash(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#java_string_check_utf8(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_java_string_check_utf8(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#optimize_for(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_optimize_for(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
                #[inline]
                pub fn r#go_package(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                #[inline]
                pub fn set_go_package(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 64;
                    } else {
                        *elem &= !64;
                    }
                }
                #[inline]
                pub fn r#cc_generic_services(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                #[inline]
                pub fn set_cc_generic_services(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 128;
                    } else {
                        *elem &= !128;
                    }
                }
                #[inline]
                pub fn r#java_generic_services(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                #[inline]
                pub fn set_java_generic_services(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#py_generic_services(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                #[inline]
                pub fn set_py_generic_services(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#cc_enable_arenas(&self) -> bool {
                    (self.0[1] & 8) != 0
                }
                #[inline]
                pub fn set_cc_enable_arenas(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#objc_class_prefix(&self) -> bool {
                    (self.0[1] & 16) != 0
                }
                #[inline]
                pub fn set_objc_class_prefix(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#csharp_namespace(&self) -> bool {
                    (self.0[1] & 32) != 0
                }
                #[inline]
                pub fn set_csharp_namespace(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
                #[inline]
                pub fn r#swift_prefix(&self) -> bool {
                    (self.0[1] & 64) != 0
                }
                #[inline]
                pub fn set_swift_prefix(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 64;
                    } else {
                        *elem &= !64;
                    }
                }
                #[inline]
                pub fn r#php_class_prefix(&self) -> bool {
                    (self.0[1] & 128) != 0
                }
                #[inline]
                pub fn set_php_class_prefix(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 128;
                    } else {
                        *elem &= !128;
                    }
                }
                #[inline]
                pub fn r#php_namespace(&self) -> bool {
                    (self.0[2] & 1) != 0
                }
                #[inline]
                pub fn set_php_namespace(&mut self, val: bool) {
                    let elem = &mut self.0[2];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#php_metadata_namespace(&self) -> bool {
                    (self.0[2] & 2) != 0
                }
                #[inline]
                pub fn set_php_metadata_namespace(&mut self, val: bool) {
                    let elem = &mut self.0[2];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#ruby_package(&self) -> bool {
                    (self.0[2] & 4) != 0
                }
                #[inline]
                pub fn set_ruby_package(&mut self, val: bool) {
                    let elem = &mut self.0[2];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[2] & 8) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[2];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
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
            pub r#optimize_for: mod_FileOptions::OptimizeMode,
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
            pub _has: mod_FileOptions::_Hazzer,
        }
        impl ::core::default::Default for FileOptions {
            fn default() -> Self {
                Self {
                    r#java_package: ::core::default::Default::default(),
                    r#java_outer_classname: ::core::default::Default::default(),
                    r#java_multiple_files: false as _,
                    r#java_generate_equals_and_hash: ::core::default::Default::default(),
                    r#java_string_check_utf8: false as _,
                    r#optimize_for: mod_FileOptions::OptimizeMode::Speed,
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
            pub fn r#java_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#java_package().then_some(&self.r#java_package)
            }
            pub fn mut_java_package(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#java_package().then_some(&mut self.r#java_package)
            }
            pub fn set_java_package(&mut self, value: ::std::string::String) {
                self._has.set_java_package(true);
                self.r#java_package = value.into();
            }
            pub fn clear_java_package(&mut self) {
                self._has.set_java_package(false);
            }
            pub fn r#java_outer_classname(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has
                    .r#java_outer_classname()
                    .then_some(&self.r#java_outer_classname)
            }
            pub fn mut_java_outer_classname(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has
                    .r#java_outer_classname()
                    .then_some(&mut self.r#java_outer_classname)
            }
            pub fn set_java_outer_classname(&mut self, value: ::std::string::String) {
                self._has.set_java_outer_classname(true);
                self.r#java_outer_classname = value.into();
            }
            pub fn clear_java_outer_classname(&mut self) {
                self._has.set_java_outer_classname(false);
            }
            pub fn r#java_multiple_files(&self) -> ::core::option::Option<&bool> {
                self._has.r#java_multiple_files().then_some(&self.r#java_multiple_files)
            }
            pub fn mut_java_multiple_files(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#java_multiple_files()
                    .then_some(&mut self.r#java_multiple_files)
            }
            pub fn set_java_multiple_files(&mut self, value: bool) {
                self._has.set_java_multiple_files(true);
                self.r#java_multiple_files = value.into();
            }
            pub fn clear_java_multiple_files(&mut self) {
                self._has.set_java_multiple_files(false);
            }
            pub fn r#java_generate_equals_and_hash(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_generate_equals_and_hash()
                    .then_some(&self.r#java_generate_equals_and_hash)
            }
            pub fn mut_java_generate_equals_and_hash(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#java_generate_equals_and_hash()
                    .then_some(&mut self.r#java_generate_equals_and_hash)
            }
            pub fn set_java_generate_equals_and_hash(&mut self, value: bool) {
                self._has.set_java_generate_equals_and_hash(true);
                self.r#java_generate_equals_and_hash = value.into();
            }
            pub fn clear_java_generate_equals_and_hash(&mut self) {
                self._has.set_java_generate_equals_and_hash(false);
            }
            pub fn r#java_string_check_utf8(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_string_check_utf8()
                    .then_some(&self.r#java_string_check_utf8)
            }
            pub fn mut_java_string_check_utf8(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#java_string_check_utf8()
                    .then_some(&mut self.r#java_string_check_utf8)
            }
            pub fn set_java_string_check_utf8(&mut self, value: bool) {
                self._has.set_java_string_check_utf8(true);
                self.r#java_string_check_utf8 = value.into();
            }
            pub fn clear_java_string_check_utf8(&mut self) {
                self._has.set_java_string_check_utf8(false);
            }
            pub fn r#optimize_for(
                &self,
            ) -> ::core::option::Option<&mod_FileOptions::OptimizeMode> {
                self._has.r#optimize_for().then_some(&self.r#optimize_for)
            }
            pub fn mut_optimize_for(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FileOptions::OptimizeMode> {
                self._has.r#optimize_for().then_some(&mut self.r#optimize_for)
            }
            pub fn set_optimize_for(&mut self, value: mod_FileOptions::OptimizeMode) {
                self._has.set_optimize_for(true);
                self.r#optimize_for = value.into();
            }
            pub fn clear_optimize_for(&mut self) {
                self._has.set_optimize_for(false);
            }
            pub fn r#go_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#go_package().then_some(&self.r#go_package)
            }
            pub fn mut_go_package(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#go_package().then_some(&mut self.r#go_package)
            }
            pub fn set_go_package(&mut self, value: ::std::string::String) {
                self._has.set_go_package(true);
                self.r#go_package = value.into();
            }
            pub fn clear_go_package(&mut self) {
                self._has.set_go_package(false);
            }
            pub fn r#cc_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has.r#cc_generic_services().then_some(&self.r#cc_generic_services)
            }
            pub fn mut_cc_generic_services(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#cc_generic_services()
                    .then_some(&mut self.r#cc_generic_services)
            }
            pub fn set_cc_generic_services(&mut self, value: bool) {
                self._has.set_cc_generic_services(true);
                self.r#cc_generic_services = value.into();
            }
            pub fn clear_cc_generic_services(&mut self) {
                self._has.set_cc_generic_services(false);
            }
            pub fn r#java_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#java_generic_services()
                    .then_some(&self.r#java_generic_services)
            }
            pub fn mut_java_generic_services(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#java_generic_services()
                    .then_some(&mut self.r#java_generic_services)
            }
            pub fn set_java_generic_services(&mut self, value: bool) {
                self._has.set_java_generic_services(true);
                self.r#java_generic_services = value.into();
            }
            pub fn clear_java_generic_services(&mut self) {
                self._has.set_java_generic_services(false);
            }
            pub fn r#py_generic_services(&self) -> ::core::option::Option<&bool> {
                self._has.r#py_generic_services().then_some(&self.r#py_generic_services)
            }
            pub fn mut_py_generic_services(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#py_generic_services()
                    .then_some(&mut self.r#py_generic_services)
            }
            pub fn set_py_generic_services(&mut self, value: bool) {
                self._has.set_py_generic_services(true);
                self.r#py_generic_services = value.into();
            }
            pub fn clear_py_generic_services(&mut self) {
                self._has.set_py_generic_services(false);
            }
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#cc_enable_arenas(&self) -> ::core::option::Option<&bool> {
                self._has.r#cc_enable_arenas().then_some(&self.r#cc_enable_arenas)
            }
            pub fn mut_cc_enable_arenas(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#cc_enable_arenas().then_some(&mut self.r#cc_enable_arenas)
            }
            pub fn set_cc_enable_arenas(&mut self, value: bool) {
                self._has.set_cc_enable_arenas(true);
                self.r#cc_enable_arenas = value.into();
            }
            pub fn clear_cc_enable_arenas(&mut self) {
                self._has.set_cc_enable_arenas(false);
            }
            pub fn r#objc_class_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#objc_class_prefix().then_some(&self.r#objc_class_prefix)
            }
            pub fn mut_objc_class_prefix(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#objc_class_prefix().then_some(&mut self.r#objc_class_prefix)
            }
            pub fn set_objc_class_prefix(&mut self, value: ::std::string::String) {
                self._has.set_objc_class_prefix(true);
                self.r#objc_class_prefix = value.into();
            }
            pub fn clear_objc_class_prefix(&mut self) {
                self._has.set_objc_class_prefix(false);
            }
            pub fn r#csharp_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#csharp_namespace().then_some(&self.r#csharp_namespace)
            }
            pub fn mut_csharp_namespace(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#csharp_namespace().then_some(&mut self.r#csharp_namespace)
            }
            pub fn set_csharp_namespace(&mut self, value: ::std::string::String) {
                self._has.set_csharp_namespace(true);
                self.r#csharp_namespace = value.into();
            }
            pub fn clear_csharp_namespace(&mut self) {
                self._has.set_csharp_namespace(false);
            }
            pub fn r#swift_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#swift_prefix().then_some(&self.r#swift_prefix)
            }
            pub fn mut_swift_prefix(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#swift_prefix().then_some(&mut self.r#swift_prefix)
            }
            pub fn set_swift_prefix(&mut self, value: ::std::string::String) {
                self._has.set_swift_prefix(true);
                self.r#swift_prefix = value.into();
            }
            pub fn clear_swift_prefix(&mut self) {
                self._has.set_swift_prefix(false);
            }
            pub fn r#php_class_prefix(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#php_class_prefix().then_some(&self.r#php_class_prefix)
            }
            pub fn mut_php_class_prefix(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#php_class_prefix().then_some(&mut self.r#php_class_prefix)
            }
            pub fn set_php_class_prefix(&mut self, value: ::std::string::String) {
                self._has.set_php_class_prefix(true);
                self.r#php_class_prefix = value.into();
            }
            pub fn clear_php_class_prefix(&mut self) {
                self._has.set_php_class_prefix(false);
            }
            pub fn r#php_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#php_namespace().then_some(&self.r#php_namespace)
            }
            pub fn mut_php_namespace(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#php_namespace().then_some(&mut self.r#php_namespace)
            }
            pub fn set_php_namespace(&mut self, value: ::std::string::String) {
                self._has.set_php_namespace(true);
                self.r#php_namespace = value.into();
            }
            pub fn clear_php_namespace(&mut self) {
                self._has.set_php_namespace(false);
            }
            pub fn r#php_metadata_namespace(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has
                    .r#php_metadata_namespace()
                    .then_some(&self.r#php_metadata_namespace)
            }
            pub fn mut_php_metadata_namespace(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has
                    .r#php_metadata_namespace()
                    .then_some(&mut self.r#php_metadata_namespace)
            }
            pub fn set_php_metadata_namespace(&mut self, value: ::std::string::String) {
                self._has.set_php_metadata_namespace(true);
                self.r#php_metadata_namespace = value.into();
            }
            pub fn clear_php_metadata_namespace(&mut self) {
                self._has.set_php_metadata_namespace(false);
            }
            pub fn r#ruby_package(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#ruby_package().then_some(&self.r#ruby_package)
            }
            pub fn mut_ruby_package(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#ruby_package().then_some(&mut self.r#ruby_package)
            }
            pub fn set_ruby_package(&mut self, value: ::std::string::String) {
                self._has.set_ruby_package(true);
                self.r#ruby_package = value.into();
            }
            pub fn clear_ruby_package(&mut self) {
                self._has.set_ruby_package(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
        }
        impl ::micropb::MessageDecode for FileOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_java_package(true);
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#java_outer_classname;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_java_outer_classname(true);
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#java_multiple_files;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_multiple_files(true);
                        }
                        20u32 => {
                            let mut_ref = &mut self.r#java_generate_equals_and_hash;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_generate_equals_and_hash(true);
                        }
                        27u32 => {
                            let mut_ref = &mut self.r#java_string_check_utf8;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_string_check_utf8(true);
                        }
                        9u32 => {
                            let mut_ref = &mut self.r#optimize_for;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FileOptions::OptimizeMode(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_optimize_for(true);
                        }
                        11u32 => {
                            let mut_ref = &mut self.r#go_package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_go_package(true);
                        }
                        16u32 => {
                            let mut_ref = &mut self.r#cc_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_cc_generic_services(true);
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#java_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_java_generic_services(true);
                        }
                        18u32 => {
                            let mut_ref = &mut self.r#py_generic_services;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_py_generic_services(true);
                        }
                        23u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        31u32 => {
                            let mut_ref = &mut self.r#cc_enable_arenas;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_cc_enable_arenas(true);
                        }
                        36u32 => {
                            let mut_ref = &mut self.r#objc_class_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_objc_class_prefix(true);
                        }
                        37u32 => {
                            let mut_ref = &mut self.r#csharp_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_csharp_namespace(true);
                        }
                        39u32 => {
                            let mut_ref = &mut self.r#swift_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_swift_prefix(true);
                        }
                        40u32 => {
                            let mut_ref = &mut self.r#php_class_prefix;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_class_prefix(true);
                        }
                        41u32 => {
                            let mut_ref = &mut self.r#php_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_namespace(true);
                        }
                        44u32 => {
                            let mut_ref = &mut self.r#php_metadata_namespace;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_php_metadata_namespace(true);
                        }
                        45u32 => {
                            let mut_ref = &mut self.r#ruby_package;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_ruby_package(true);
                        }
                        50u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features(true);
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
        pub mod mod_MessageOptions {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#message_set_wire_format(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_message_set_wire_format(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#no_standard_descriptor_accessor(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_no_standard_descriptor_accessor(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#map_entry(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_map_entry(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#deprecated_legacy_json_field_conflicts(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_deprecated_legacy_json_field_conflicts(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
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
            pub _has: mod_MessageOptions::_Hazzer,
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
            pub fn r#message_set_wire_format(&self) -> ::core::option::Option<&bool> {
                self._has
                    .r#message_set_wire_format()
                    .then_some(&self.r#message_set_wire_format)
            }
            pub fn mut_message_set_wire_format(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#message_set_wire_format()
                    .then_some(&mut self.r#message_set_wire_format)
            }
            pub fn set_message_set_wire_format(&mut self, value: bool) {
                self._has.set_message_set_wire_format(true);
                self.r#message_set_wire_format = value.into();
            }
            pub fn clear_message_set_wire_format(&mut self) {
                self._has.set_message_set_wire_format(false);
            }
            pub fn r#no_standard_descriptor_accessor(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#no_standard_descriptor_accessor()
                    .then_some(&self.r#no_standard_descriptor_accessor)
            }
            pub fn mut_no_standard_descriptor_accessor(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#no_standard_descriptor_accessor()
                    .then_some(&mut self.r#no_standard_descriptor_accessor)
            }
            pub fn set_no_standard_descriptor_accessor(&mut self, value: bool) {
                self._has.set_no_standard_descriptor_accessor(true);
                self.r#no_standard_descriptor_accessor = value.into();
            }
            pub fn clear_no_standard_descriptor_accessor(&mut self) {
                self._has.set_no_standard_descriptor_accessor(false);
            }
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#map_entry(&self) -> ::core::option::Option<&bool> {
                self._has.r#map_entry().then_some(&self.r#map_entry)
            }
            pub fn mut_map_entry(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#map_entry().then_some(&mut self.r#map_entry)
            }
            pub fn set_map_entry(&mut self, value: bool) {
                self._has.set_map_entry(true);
                self.r#map_entry = value.into();
            }
            pub fn clear_map_entry(&mut self) {
                self._has.set_map_entry(false);
            }
            pub fn r#deprecated_legacy_json_field_conflicts(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&self.r#deprecated_legacy_json_field_conflicts)
            }
            pub fn mut_deprecated_legacy_json_field_conflicts(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&mut self.r#deprecated_legacy_json_field_conflicts)
            }
            pub fn set_deprecated_legacy_json_field_conflicts(&mut self, value: bool) {
                self._has.set_deprecated_legacy_json_field_conflicts(true);
                self.r#deprecated_legacy_json_field_conflicts = value.into();
            }
            pub fn clear_deprecated_legacy_json_field_conflicts(&mut self) {
                self._has.set_deprecated_legacy_json_field_conflicts(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
        }
        impl ::micropb::MessageDecode for MessageOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut_ref = &mut self.r#message_set_wire_format;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_message_set_wire_format(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#no_standard_descriptor_accessor;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_no_standard_descriptor_accessor(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#map_entry;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_map_entry(true);
                        }
                        11u32 => {
                            let mut_ref = &mut self
                                .r#deprecated_legacy_json_field_conflicts;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated_legacy_json_field_conflicts(true);
                        }
                        12u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features(true);
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
        pub mod mod_FieldOptions {
            pub mod mod_EditionDefault {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#edition(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_edition(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#value(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_value(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct EditionDefault {
                pub r#edition: super::Edition,
                pub r#value: ::std::string::String,
                pub _has: mod_EditionDefault::_Hazzer,
            }
            impl ::core::default::Default for EditionDefault {
                fn default() -> Self {
                    Self {
                        r#edition: ::core::default::Default::default(),
                        r#value: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl EditionDefault {
                pub fn r#edition(&self) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition().then_some(&self.r#edition)
                }
                pub fn mut_edition(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has.r#edition().then_some(&mut self.r#edition)
                }
                pub fn set_edition(&mut self, value: super::Edition) {
                    self._has.set_edition(true);
                    self.r#edition = value.into();
                }
                pub fn clear_edition(&mut self) {
                    self._has.set_edition(false);
                }
                pub fn r#value(&self) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#value().then_some(&self.r#value)
                }
                pub fn mut_value(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#value().then_some(&mut self.r#value)
                }
                pub fn set_value(&mut self, value: ::std::string::String) {
                    self._has.set_value(true);
                    self.r#value = value.into();
                }
                pub fn clear_value(&mut self) {
                    self._has.set_value(false);
                }
            }
            impl ::micropb::MessageDecode for EditionDefault {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#value;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_value(true);
                            }
                            _ => {
                                decoder.skip_wire_value(tag.wire_type())?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            pub mod mod_FeatureSupport {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#edition_introduced(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_edition_introduced(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#edition_deprecated(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_edition_deprecated(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                    #[inline]
                    pub fn r#deprecation_warning(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    #[inline]
                    pub fn set_deprecation_warning(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 4;
                        } else {
                            *elem &= !4;
                        }
                    }
                    #[inline]
                    pub fn r#edition_removed(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    #[inline]
                    pub fn set_edition_removed(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 8;
                        } else {
                            *elem &= !8;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct FeatureSupport {
                pub r#edition_introduced: super::Edition,
                pub r#edition_deprecated: super::Edition,
                pub r#deprecation_warning: ::std::string::String,
                pub r#edition_removed: super::Edition,
                pub _has: mod_FeatureSupport::_Hazzer,
            }
            impl ::core::default::Default for FeatureSupport {
                fn default() -> Self {
                    Self {
                        r#edition_introduced: ::core::default::Default::default(),
                        r#edition_deprecated: ::core::default::Default::default(),
                        r#deprecation_warning: ::core::default::Default::default(),
                        r#edition_removed: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl FeatureSupport {
                pub fn r#edition_introduced(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has
                        .r#edition_introduced()
                        .then_some(&self.r#edition_introduced)
                }
                pub fn mut_edition_introduced(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has
                        .r#edition_introduced()
                        .then_some(&mut self.r#edition_introduced)
                }
                pub fn set_edition_introduced(&mut self, value: super::Edition) {
                    self._has.set_edition_introduced(true);
                    self.r#edition_introduced = value.into();
                }
                pub fn clear_edition_introduced(&mut self) {
                    self._has.set_edition_introduced(false);
                }
                pub fn r#edition_deprecated(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has
                        .r#edition_deprecated()
                        .then_some(&self.r#edition_deprecated)
                }
                pub fn mut_edition_deprecated(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has
                        .r#edition_deprecated()
                        .then_some(&mut self.r#edition_deprecated)
                }
                pub fn set_edition_deprecated(&mut self, value: super::Edition) {
                    self._has.set_edition_deprecated(true);
                    self.r#edition_deprecated = value.into();
                }
                pub fn clear_edition_deprecated(&mut self) {
                    self._has.set_edition_deprecated(false);
                }
                pub fn r#deprecation_warning(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has
                        .r#deprecation_warning()
                        .then_some(&self.r#deprecation_warning)
                }
                pub fn mut_deprecation_warning(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has
                        .r#deprecation_warning()
                        .then_some(&mut self.r#deprecation_warning)
                }
                pub fn set_deprecation_warning(&mut self, value: ::std::string::String) {
                    self._has.set_deprecation_warning(true);
                    self.r#deprecation_warning = value.into();
                }
                pub fn clear_deprecation_warning(&mut self) {
                    self._has.set_deprecation_warning(false);
                }
                pub fn r#edition_removed(
                    &self,
                ) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition_removed().then_some(&self.r#edition_removed)
                }
                pub fn mut_edition_removed(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has.r#edition_removed().then_some(&mut self.r#edition_removed)
                }
                pub fn set_edition_removed(&mut self, value: super::Edition) {
                    self._has.set_edition_removed(true);
                    self.r#edition_removed = value.into();
                }
                pub fn clear_edition_removed(&mut self) {
                    self._has.set_edition_removed(false);
                }
            }
            impl ::micropb::MessageDecode for FeatureSupport {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_introduced(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#edition_deprecated;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_deprecated(true);
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#deprecation_warning;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_deprecation_warning(true);
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#edition_removed;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| super::Edition(n as _))?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition_removed(true);
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
                #[inline]
                pub fn r#ctype(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_ctype(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#packed(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_packed(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#jstype(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_jstype(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#lazy(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_lazy(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#unverified_lazy(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_unverified_lazy(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
                #[inline]
                pub fn r#weak(&self) -> bool {
                    (self.0[0] & 64) != 0
                }
                #[inline]
                pub fn set_weak(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 64;
                    } else {
                        *elem &= !64;
                    }
                }
                #[inline]
                pub fn r#debug_redact(&self) -> bool {
                    (self.0[0] & 128) != 0
                }
                #[inline]
                pub fn set_debug_redact(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 128;
                    } else {
                        *elem &= !128;
                    }
                }
                #[inline]
                pub fn r#retention(&self) -> bool {
                    (self.0[1] & 1) != 0
                }
                #[inline]
                pub fn set_retention(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[1] & 2) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#feature_support(&self) -> bool {
                    (self.0[1] & 4) != 0
                }
                #[inline]
                pub fn set_feature_support(&mut self, val: bool) {
                    let elem = &mut self.0[1];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct FieldOptions {
            pub r#ctype: mod_FieldOptions::CType,
            pub r#packed: bool,
            pub r#jstype: mod_FieldOptions::JSType,
            pub r#lazy: bool,
            pub r#unverified_lazy: bool,
            pub r#deprecated: bool,
            pub r#weak: bool,
            pub r#debug_redact: bool,
            pub r#retention: mod_FieldOptions::OptionRetention,
            pub r#targets: ::std::vec::Vec<mod_FieldOptions::OptionTargetType>,
            pub r#edition_defaults: ::std::vec::Vec<mod_FieldOptions::EditionDefault>,
            pub r#features: FeatureSet,
            pub r#feature_support: mod_FieldOptions::FeatureSupport,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: mod_FieldOptions::_Hazzer,
        }
        impl ::core::default::Default for FieldOptions {
            fn default() -> Self {
                Self {
                    r#ctype: mod_FieldOptions::CType::String,
                    r#packed: ::core::default::Default::default(),
                    r#jstype: mod_FieldOptions::JSType::JsNormal,
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
            pub fn r#ctype(&self) -> ::core::option::Option<&mod_FieldOptions::CType> {
                self._has.r#ctype().then_some(&self.r#ctype)
            }
            pub fn mut_ctype(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldOptions::CType> {
                self._has.r#ctype().then_some(&mut self.r#ctype)
            }
            pub fn set_ctype(&mut self, value: mod_FieldOptions::CType) {
                self._has.set_ctype(true);
                self.r#ctype = value.into();
            }
            pub fn clear_ctype(&mut self) {
                self._has.set_ctype(false);
            }
            pub fn r#packed(&self) -> ::core::option::Option<&bool> {
                self._has.r#packed().then_some(&self.r#packed)
            }
            pub fn mut_packed(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#packed().then_some(&mut self.r#packed)
            }
            pub fn set_packed(&mut self, value: bool) {
                self._has.set_packed(true);
                self.r#packed = value.into();
            }
            pub fn clear_packed(&mut self) {
                self._has.set_packed(false);
            }
            pub fn r#jstype(&self) -> ::core::option::Option<&mod_FieldOptions::JSType> {
                self._has.r#jstype().then_some(&self.r#jstype)
            }
            pub fn mut_jstype(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldOptions::JSType> {
                self._has.r#jstype().then_some(&mut self.r#jstype)
            }
            pub fn set_jstype(&mut self, value: mod_FieldOptions::JSType) {
                self._has.set_jstype(true);
                self.r#jstype = value.into();
            }
            pub fn clear_jstype(&mut self) {
                self._has.set_jstype(false);
            }
            pub fn r#lazy(&self) -> ::core::option::Option<&bool> {
                self._has.r#lazy().then_some(&self.r#lazy)
            }
            pub fn mut_lazy(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#lazy().then_some(&mut self.r#lazy)
            }
            pub fn set_lazy(&mut self, value: bool) {
                self._has.set_lazy(true);
                self.r#lazy = value.into();
            }
            pub fn clear_lazy(&mut self) {
                self._has.set_lazy(false);
            }
            pub fn r#unverified_lazy(&self) -> ::core::option::Option<&bool> {
                self._has.r#unverified_lazy().then_some(&self.r#unverified_lazy)
            }
            pub fn mut_unverified_lazy(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#unverified_lazy().then_some(&mut self.r#unverified_lazy)
            }
            pub fn set_unverified_lazy(&mut self, value: bool) {
                self._has.set_unverified_lazy(true);
                self.r#unverified_lazy = value.into();
            }
            pub fn clear_unverified_lazy(&mut self) {
                self._has.set_unverified_lazy(false);
            }
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#weak(&self) -> ::core::option::Option<&bool> {
                self._has.r#weak().then_some(&self.r#weak)
            }
            pub fn mut_weak(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#weak().then_some(&mut self.r#weak)
            }
            pub fn set_weak(&mut self, value: bool) {
                self._has.set_weak(true);
                self.r#weak = value.into();
            }
            pub fn clear_weak(&mut self) {
                self._has.set_weak(false);
            }
            pub fn r#debug_redact(&self) -> ::core::option::Option<&bool> {
                self._has.r#debug_redact().then_some(&self.r#debug_redact)
            }
            pub fn mut_debug_redact(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#debug_redact().then_some(&mut self.r#debug_redact)
            }
            pub fn set_debug_redact(&mut self, value: bool) {
                self._has.set_debug_redact(true);
                self.r#debug_redact = value.into();
            }
            pub fn clear_debug_redact(&mut self) {
                self._has.set_debug_redact(false);
            }
            pub fn r#retention(
                &self,
            ) -> ::core::option::Option<&mod_FieldOptions::OptionRetention> {
                self._has.r#retention().then_some(&self.r#retention)
            }
            pub fn mut_retention(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldOptions::OptionRetention> {
                self._has.r#retention().then_some(&mut self.r#retention)
            }
            pub fn set_retention(&mut self, value: mod_FieldOptions::OptionRetention) {
                self._has.set_retention(true);
                self.r#retention = value.into();
            }
            pub fn clear_retention(&mut self) {
                self._has.set_retention(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
            pub fn r#feature_support(
                &self,
            ) -> ::core::option::Option<&mod_FieldOptions::FeatureSupport> {
                self._has.r#feature_support().then_some(&self.r#feature_support)
            }
            pub fn mut_feature_support(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldOptions::FeatureSupport> {
                self._has.r#feature_support().then_some(&mut self.r#feature_support)
            }
            pub fn set_feature_support(
                &mut self,
                value: mod_FieldOptions::FeatureSupport,
            ) {
                self._has.set_feature_support(true);
                self.r#feature_support = value.into();
            }
            pub fn clear_feature_support(&mut self) {
                self._has.set_feature_support(false);
            }
        }
        impl ::micropb::MessageDecode for FieldOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                    .map(|n| mod_FieldOptions::CType(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_ctype(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#packed;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_packed(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#jstype;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FieldOptions::JSType(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_jstype(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#lazy;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_lazy(true);
                        }
                        15u32 => {
                            let mut_ref = &mut self.r#unverified_lazy;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_unverified_lazy(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        10u32 => {
                            let mut_ref = &mut self.r#weak;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_weak(true);
                        }
                        16u32 => {
                            let mut_ref = &mut self.r#debug_redact;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_debug_redact(true);
                        }
                        17u32 => {
                            let mut_ref = &mut self.r#retention;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FieldOptions::OptionRetention(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_retention(true);
                        }
                        19u32 => {
                            if tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                                decoder
                                    .decode_packed(
                                        &mut self.r#targets,
                                        |decoder| {
                                            decoder
                                                .decode_int32()
                                                .map(|n| mod_FieldOptions::OptionTargetType(n as _))
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
                                                .map(|n| mod_FieldOptions::OptionTargetType(n as _))? as _,
                                        ),
                                    decoder.ignore_repeated_cap_err,
                                ) {
                                    return Err(::micropb::DecodeError::Capacity);
                                }
                            }
                        }
                        20u32 => {
                            let mut val: mod_FieldOptions::EditionDefault = ::core::default::Default::default();
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
                            self._has.set_features(true);
                        }
                        22u32 => {
                            let mut_ref = &mut self.r#feature_support;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_feature_support(true);
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
        pub mod mod_OneofOptions {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct OneofOptions {
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: mod_OneofOptions::_Hazzer,
        }
        impl ::core::default::Default for OneofOptions {
            fn default() -> Self {
                Self {
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl OneofOptions {
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
        }
        impl ::micropb::MessageDecode for OneofOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_features(true);
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
        pub mod mod_EnumOptions {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#allow_alias(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_allow_alias(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#deprecated_legacy_json_field_conflicts(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_deprecated_legacy_json_field_conflicts(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
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
            pub _has: mod_EnumOptions::_Hazzer,
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
            pub fn r#allow_alias(&self) -> ::core::option::Option<&bool> {
                self._has.r#allow_alias().then_some(&self.r#allow_alias)
            }
            pub fn mut_allow_alias(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#allow_alias().then_some(&mut self.r#allow_alias)
            }
            pub fn set_allow_alias(&mut self, value: bool) {
                self._has.set_allow_alias(true);
                self.r#allow_alias = value.into();
            }
            pub fn clear_allow_alias(&mut self) {
                self._has.set_allow_alias(false);
            }
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#deprecated_legacy_json_field_conflicts(
                &self,
            ) -> ::core::option::Option<&bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&self.r#deprecated_legacy_json_field_conflicts)
            }
            pub fn mut_deprecated_legacy_json_field_conflicts(
                &mut self,
            ) -> ::core::option::Option<&mut bool> {
                self._has
                    .r#deprecated_legacy_json_field_conflicts()
                    .then_some(&mut self.r#deprecated_legacy_json_field_conflicts)
            }
            pub fn set_deprecated_legacy_json_field_conflicts(&mut self, value: bool) {
                self._has.set_deprecated_legacy_json_field_conflicts(true);
                self.r#deprecated_legacy_json_field_conflicts = value.into();
            }
            pub fn clear_deprecated_legacy_json_field_conflicts(&mut self) {
                self._has.set_deprecated_legacy_json_field_conflicts(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
        }
        impl ::micropb::MessageDecode for EnumOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        2u32 => {
                            let mut_ref = &mut self.r#allow_alias;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_allow_alias(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self
                                .r#deprecated_legacy_json_field_conflicts;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated_legacy_json_field_conflicts(true);
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features(true);
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
        pub mod mod_EnumValueOptions {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#debug_redact(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_debug_redact(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#feature_support(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_feature_support(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct EnumValueOptions {
            pub r#deprecated: bool,
            pub r#features: FeatureSet,
            pub r#debug_redact: bool,
            pub r#feature_support: mod_FieldOptions::FeatureSupport,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: mod_EnumValueOptions::_Hazzer,
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
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
            pub fn r#debug_redact(&self) -> ::core::option::Option<&bool> {
                self._has.r#debug_redact().then_some(&self.r#debug_redact)
            }
            pub fn mut_debug_redact(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#debug_redact().then_some(&mut self.r#debug_redact)
            }
            pub fn set_debug_redact(&mut self, value: bool) {
                self._has.set_debug_redact(true);
                self.r#debug_redact = value.into();
            }
            pub fn clear_debug_redact(&mut self) {
                self._has.set_debug_redact(false);
            }
            pub fn r#feature_support(
                &self,
            ) -> ::core::option::Option<&mod_FieldOptions::FeatureSupport> {
                self._has.r#feature_support().then_some(&self.r#feature_support)
            }
            pub fn mut_feature_support(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FieldOptions::FeatureSupport> {
                self._has.r#feature_support().then_some(&mut self.r#feature_support)
            }
            pub fn set_feature_support(
                &mut self,
                value: mod_FieldOptions::FeatureSupport,
            ) {
                self._has.set_feature_support(true);
                self.r#feature_support = value.into();
            }
            pub fn clear_feature_support(&mut self) {
                self._has.set_feature_support(false);
            }
        }
        impl ::micropb::MessageDecode for EnumValueOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#debug_redact;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_debug_redact(true);
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#feature_support;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_feature_support(true);
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
        pub mod mod_ServiceOptions {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct ServiceOptions {
            pub r#features: FeatureSet,
            pub r#deprecated: bool,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: mod_ServiceOptions::_Hazzer,
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
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
        }
        impl ::micropb::MessageDecode for ServiceOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                            self._has.set_features(true);
                        }
                        33u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
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
        pub mod mod_MethodOptions {
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
                #[inline]
                pub fn r#deprecated(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_deprecated(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#idempotency_level(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_idempotency_level(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#features(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_features(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct MethodOptions {
            pub r#deprecated: bool,
            pub r#idempotency_level: mod_MethodOptions::IdempotencyLevel,
            pub r#features: FeatureSet,
            pub r#uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            pub _has: mod_MethodOptions::_Hazzer,
        }
        impl ::core::default::Default for MethodOptions {
            fn default() -> Self {
                Self {
                    r#deprecated: false as _,
                    r#idempotency_level: mod_MethodOptions::IdempotencyLevel::IdempotencyUnknown,
                    r#features: ::core::default::Default::default(),
                    r#uninterpreted_option: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl MethodOptions {
            pub fn r#deprecated(&self) -> ::core::option::Option<&bool> {
                self._has.r#deprecated().then_some(&self.r#deprecated)
            }
            pub fn mut_deprecated(&mut self) -> ::core::option::Option<&mut bool> {
                self._has.r#deprecated().then_some(&mut self.r#deprecated)
            }
            pub fn set_deprecated(&mut self, value: bool) {
                self._has.set_deprecated(true);
                self.r#deprecated = value.into();
            }
            pub fn clear_deprecated(&mut self) {
                self._has.set_deprecated(false);
            }
            pub fn r#idempotency_level(
                &self,
            ) -> ::core::option::Option<&mod_MethodOptions::IdempotencyLevel> {
                self._has.r#idempotency_level().then_some(&self.r#idempotency_level)
            }
            pub fn mut_idempotency_level(
                &mut self,
            ) -> ::core::option::Option<&mut mod_MethodOptions::IdempotencyLevel> {
                self._has.r#idempotency_level().then_some(&mut self.r#idempotency_level)
            }
            pub fn set_idempotency_level(
                &mut self,
                value: mod_MethodOptions::IdempotencyLevel,
            ) {
                self._has.set_idempotency_level(true);
                self.r#idempotency_level = value.into();
            }
            pub fn clear_idempotency_level(&mut self) {
                self._has.set_idempotency_level(false);
            }
            pub fn r#features(&self) -> ::core::option::Option<&FeatureSet> {
                self._has.r#features().then_some(&self.r#features)
            }
            pub fn mut_features(&mut self) -> ::core::option::Option<&mut FeatureSet> {
                self._has.r#features().then_some(&mut self.r#features)
            }
            pub fn set_features(&mut self, value: FeatureSet) {
                self._has.set_features(true);
                self.r#features = value.into();
            }
            pub fn clear_features(&mut self) {
                self._has.set_features(false);
            }
        }
        impl ::micropb::MessageDecode for MethodOptions {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        33u32 => {
                            let mut_ref = &mut self.r#deprecated;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_deprecated(true);
                        }
                        34u32 => {
                            let mut_ref = &mut self.r#idempotency_level;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_MethodOptions::IdempotencyLevel(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_idempotency_level(true);
                        }
                        35u32 => {
                            let mut_ref = &mut self.r#features;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_features(true);
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
        pub mod mod_UninterpretedOption {
            pub mod mod_NamePart {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#name_part(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_name_part(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#is_extension(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_is_extension(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct NamePart {
                pub r#name_part: ::std::string::String,
                pub r#is_extension: bool,
                pub _has: mod_NamePart::_Hazzer,
            }
            impl ::core::default::Default for NamePart {
                fn default() -> Self {
                    Self {
                        r#name_part: ::core::default::Default::default(),
                        r#is_extension: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl NamePart {
                pub fn r#name_part(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#name_part().then_some(&self.r#name_part)
                }
                pub fn mut_name_part(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#name_part().then_some(&mut self.r#name_part)
                }
                pub fn set_name_part(&mut self, value: ::std::string::String) {
                    self._has.set_name_part(true);
                    self.r#name_part = value.into();
                }
                pub fn clear_name_part(&mut self) {
                    self._has.set_name_part(false);
                }
                pub fn r#is_extension(&self) -> ::core::option::Option<&bool> {
                    self._has.r#is_extension().then_some(&self.r#is_extension)
                }
                pub fn mut_is_extension(&mut self) -> ::core::option::Option<&mut bool> {
                    self._has.r#is_extension().then_some(&mut self.r#is_extension)
                }
                pub fn set_is_extension(&mut self, value: bool) {
                    self._has.set_is_extension(true);
                    self.r#is_extension = value.into();
                }
                pub fn clear_is_extension(&mut self) {
                    self._has.set_is_extension(false);
                }
            }
            impl ::micropb::MessageDecode for NamePart {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                self._has.set_name_part(true);
                            }
                            2u32 => {
                                let mut_ref = &mut self.r#is_extension;
                                {
                                    let val = decoder.decode_bool()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_is_extension(true);
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
                #[inline]
                pub fn r#identifier_value(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_identifier_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#positive_int_value(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_positive_int_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#negative_int_value(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_negative_int_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#double_value(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_double_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#string_value(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_string_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#aggregate_value(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_aggregate_value(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct UninterpretedOption {
            pub r#name: ::std::vec::Vec<mod_UninterpretedOption::NamePart>,
            pub r#identifier_value: ::std::string::String,
            pub r#positive_int_value: u64,
            pub r#negative_int_value: i64,
            pub r#double_value: f64,
            pub r#string_value: ::std::vec::Vec<u8>,
            pub r#aggregate_value: ::std::string::String,
            pub _has: mod_UninterpretedOption::_Hazzer,
        }
        impl ::core::default::Default for UninterpretedOption {
            fn default() -> Self {
                Self {
                    r#name: ::core::default::Default::default(),
                    r#identifier_value: ::core::default::Default::default(),
                    r#positive_int_value: ::core::default::Default::default(),
                    r#negative_int_value: ::core::default::Default::default(),
                    r#double_value: ::core::default::Default::default(),
                    r#string_value: ::core::default::Default::default(),
                    r#aggregate_value: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl UninterpretedOption {
            pub fn r#identifier_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#identifier_value().then_some(&self.r#identifier_value)
            }
            pub fn mut_identifier_value(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#identifier_value().then_some(&mut self.r#identifier_value)
            }
            pub fn set_identifier_value(&mut self, value: ::std::string::String) {
                self._has.set_identifier_value(true);
                self.r#identifier_value = value.into();
            }
            pub fn clear_identifier_value(&mut self) {
                self._has.set_identifier_value(false);
            }
            pub fn r#positive_int_value(&self) -> ::core::option::Option<&u64> {
                self._has.r#positive_int_value().then_some(&self.r#positive_int_value)
            }
            pub fn mut_positive_int_value(
                &mut self,
            ) -> ::core::option::Option<&mut u64> {
                self._has
                    .r#positive_int_value()
                    .then_some(&mut self.r#positive_int_value)
            }
            pub fn set_positive_int_value(&mut self, value: u64) {
                self._has.set_positive_int_value(true);
                self.r#positive_int_value = value.into();
            }
            pub fn clear_positive_int_value(&mut self) {
                self._has.set_positive_int_value(false);
            }
            pub fn r#negative_int_value(&self) -> ::core::option::Option<&i64> {
                self._has.r#negative_int_value().then_some(&self.r#negative_int_value)
            }
            pub fn mut_negative_int_value(
                &mut self,
            ) -> ::core::option::Option<&mut i64> {
                self._has
                    .r#negative_int_value()
                    .then_some(&mut self.r#negative_int_value)
            }
            pub fn set_negative_int_value(&mut self, value: i64) {
                self._has.set_negative_int_value(true);
                self.r#negative_int_value = value.into();
            }
            pub fn clear_negative_int_value(&mut self) {
                self._has.set_negative_int_value(false);
            }
            pub fn r#double_value(&self) -> ::core::option::Option<&f64> {
                self._has.r#double_value().then_some(&self.r#double_value)
            }
            pub fn mut_double_value(&mut self) -> ::core::option::Option<&mut f64> {
                self._has.r#double_value().then_some(&mut self.r#double_value)
            }
            pub fn set_double_value(&mut self, value: f64) {
                self._has.set_double_value(true);
                self.r#double_value = value.into();
            }
            pub fn clear_double_value(&mut self) {
                self._has.set_double_value(false);
            }
            pub fn r#string_value(
                &self,
            ) -> ::core::option::Option<&::std::vec::Vec<u8>> {
                self._has.r#string_value().then_some(&self.r#string_value)
            }
            pub fn mut_string_value(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::vec::Vec<u8>> {
                self._has.r#string_value().then_some(&mut self.r#string_value)
            }
            pub fn set_string_value(&mut self, value: ::std::vec::Vec<u8>) {
                self._has.set_string_value(true);
                self.r#string_value = value.into();
            }
            pub fn clear_string_value(&mut self) {
                self._has.set_string_value(false);
            }
            pub fn r#aggregate_value(
                &self,
            ) -> ::core::option::Option<&::std::string::String> {
                self._has.r#aggregate_value().then_some(&self.r#aggregate_value)
            }
            pub fn mut_aggregate_value(
                &mut self,
            ) -> ::core::option::Option<&mut ::std::string::String> {
                self._has.r#aggregate_value().then_some(&mut self.r#aggregate_value)
            }
            pub fn set_aggregate_value(&mut self, value: ::std::string::String) {
                self._has.set_aggregate_value(true);
                self.r#aggregate_value = value.into();
            }
            pub fn clear_aggregate_value(&mut self) {
                self._has.set_aggregate_value(false);
            }
        }
        impl ::micropb::MessageDecode for UninterpretedOption {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        2u32 => {
                            let mut val: mod_UninterpretedOption::NamePart = ::core::default::Default::default();
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
                            self._has.set_identifier_value(true);
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#positive_int_value;
                            {
                                let val = decoder.decode_varint64()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_positive_int_value(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#negative_int_value;
                            {
                                let val = decoder.decode_int64()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_negative_int_value(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#double_value;
                            {
                                let val = decoder.decode_double()?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_double_value(true);
                        }
                        7u32 => {
                            let mut_ref = &mut self.r#string_value;
                            {
                                decoder
                                    .decode_bytes(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_string_value(true);
                        }
                        8u32 => {
                            let mut_ref = &mut self.r#aggregate_value;
                            {
                                decoder
                                    .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                            };
                            self._has.set_aggregate_value(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_FeatureSet {
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
                #[inline]
                pub fn r#field_presence(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_field_presence(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#enum_type(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_enum_type(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
                #[inline]
                pub fn r#repeated_field_encoding(&self) -> bool {
                    (self.0[0] & 4) != 0
                }
                #[inline]
                pub fn set_repeated_field_encoding(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 4;
                    } else {
                        *elem &= !4;
                    }
                }
                #[inline]
                pub fn r#utf8_validation(&self) -> bool {
                    (self.0[0] & 8) != 0
                }
                #[inline]
                pub fn set_utf8_validation(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 8;
                    } else {
                        *elem &= !8;
                    }
                }
                #[inline]
                pub fn r#message_encoding(&self) -> bool {
                    (self.0[0] & 16) != 0
                }
                #[inline]
                pub fn set_message_encoding(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 16;
                    } else {
                        *elem &= !16;
                    }
                }
                #[inline]
                pub fn r#json_format(&self) -> bool {
                    (self.0[0] & 32) != 0
                }
                #[inline]
                pub fn set_json_format(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 32;
                    } else {
                        *elem &= !32;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct FeatureSet {
            pub r#field_presence: mod_FeatureSet::FieldPresence,
            pub r#enum_type: mod_FeatureSet::EnumType,
            pub r#repeated_field_encoding: mod_FeatureSet::RepeatedFieldEncoding,
            pub r#utf8_validation: mod_FeatureSet::Utf8Validation,
            pub r#message_encoding: mod_FeatureSet::MessageEncoding,
            pub r#json_format: mod_FeatureSet::JsonFormat,
            pub _has: mod_FeatureSet::_Hazzer,
        }
        impl ::core::default::Default for FeatureSet {
            fn default() -> Self {
                Self {
                    r#field_presence: ::core::default::Default::default(),
                    r#enum_type: ::core::default::Default::default(),
                    r#repeated_field_encoding: ::core::default::Default::default(),
                    r#utf8_validation: ::core::default::Default::default(),
                    r#message_encoding: ::core::default::Default::default(),
                    r#json_format: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FeatureSet {
            pub fn r#field_presence(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::FieldPresence> {
                self._has.r#field_presence().then_some(&self.r#field_presence)
            }
            pub fn mut_field_presence(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::FieldPresence> {
                self._has.r#field_presence().then_some(&mut self.r#field_presence)
            }
            pub fn set_field_presence(&mut self, value: mod_FeatureSet::FieldPresence) {
                self._has.set_field_presence(true);
                self.r#field_presence = value.into();
            }
            pub fn clear_field_presence(&mut self) {
                self._has.set_field_presence(false);
            }
            pub fn r#enum_type(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::EnumType> {
                self._has.r#enum_type().then_some(&self.r#enum_type)
            }
            pub fn mut_enum_type(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::EnumType> {
                self._has.r#enum_type().then_some(&mut self.r#enum_type)
            }
            pub fn set_enum_type(&mut self, value: mod_FeatureSet::EnumType) {
                self._has.set_enum_type(true);
                self.r#enum_type = value.into();
            }
            pub fn clear_enum_type(&mut self) {
                self._has.set_enum_type(false);
            }
            pub fn r#repeated_field_encoding(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::RepeatedFieldEncoding> {
                self._has
                    .r#repeated_field_encoding()
                    .then_some(&self.r#repeated_field_encoding)
            }
            pub fn mut_repeated_field_encoding(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::RepeatedFieldEncoding> {
                self._has
                    .r#repeated_field_encoding()
                    .then_some(&mut self.r#repeated_field_encoding)
            }
            pub fn set_repeated_field_encoding(
                &mut self,
                value: mod_FeatureSet::RepeatedFieldEncoding,
            ) {
                self._has.set_repeated_field_encoding(true);
                self.r#repeated_field_encoding = value.into();
            }
            pub fn clear_repeated_field_encoding(&mut self) {
                self._has.set_repeated_field_encoding(false);
            }
            pub fn r#utf8_validation(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::Utf8Validation> {
                self._has.r#utf8_validation().then_some(&self.r#utf8_validation)
            }
            pub fn mut_utf8_validation(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::Utf8Validation> {
                self._has.r#utf8_validation().then_some(&mut self.r#utf8_validation)
            }
            pub fn set_utf8_validation(
                &mut self,
                value: mod_FeatureSet::Utf8Validation,
            ) {
                self._has.set_utf8_validation(true);
                self.r#utf8_validation = value.into();
            }
            pub fn clear_utf8_validation(&mut self) {
                self._has.set_utf8_validation(false);
            }
            pub fn r#message_encoding(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::MessageEncoding> {
                self._has.r#message_encoding().then_some(&self.r#message_encoding)
            }
            pub fn mut_message_encoding(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::MessageEncoding> {
                self._has.r#message_encoding().then_some(&mut self.r#message_encoding)
            }
            pub fn set_message_encoding(
                &mut self,
                value: mod_FeatureSet::MessageEncoding,
            ) {
                self._has.set_message_encoding(true);
                self.r#message_encoding = value.into();
            }
            pub fn clear_message_encoding(&mut self) {
                self._has.set_message_encoding(false);
            }
            pub fn r#json_format(
                &self,
            ) -> ::core::option::Option<&mod_FeatureSet::JsonFormat> {
                self._has.r#json_format().then_some(&self.r#json_format)
            }
            pub fn mut_json_format(
                &mut self,
            ) -> ::core::option::Option<&mut mod_FeatureSet::JsonFormat> {
                self._has.r#json_format().then_some(&mut self.r#json_format)
            }
            pub fn set_json_format(&mut self, value: mod_FeatureSet::JsonFormat) {
                self._has.set_json_format(true);
                self.r#json_format = value.into();
            }
            pub fn clear_json_format(&mut self) {
                self._has.set_json_format(false);
            }
        }
        impl ::micropb::MessageDecode for FeatureSet {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                    .map(|n| mod_FeatureSet::FieldPresence(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_field_presence(true);
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#enum_type;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FeatureSet::EnumType(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_enum_type(true);
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#repeated_field_encoding;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FeatureSet::RepeatedFieldEncoding(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_repeated_field_encoding(true);
                        }
                        4u32 => {
                            let mut_ref = &mut self.r#utf8_validation;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FeatureSet::Utf8Validation(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_utf8_validation(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#message_encoding;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FeatureSet::MessageEncoding(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_message_encoding(true);
                        }
                        6u32 => {
                            let mut_ref = &mut self.r#json_format;
                            {
                                let val = decoder
                                    .decode_int32()
                                    .map(|n| mod_FeatureSet::JsonFormat(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_json_format(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_FeatureSetDefaults {
            pub mod mod_FeatureSetEditionDefault {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#edition(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_edition(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#overridable_features(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_overridable_features(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                    #[inline]
                    pub fn r#fixed_features(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    #[inline]
                    pub fn set_fixed_features(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 4;
                        } else {
                            *elem &= !4;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct FeatureSetEditionDefault {
                pub r#edition: super::Edition,
                pub r#overridable_features: super::FeatureSet,
                pub r#fixed_features: super::FeatureSet,
                pub _has: mod_FeatureSetEditionDefault::_Hazzer,
            }
            impl ::core::default::Default for FeatureSetEditionDefault {
                fn default() -> Self {
                    Self {
                        r#edition: ::core::default::Default::default(),
                        r#overridable_features: ::core::default::Default::default(),
                        r#fixed_features: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl FeatureSetEditionDefault {
                pub fn r#edition(&self) -> ::core::option::Option<&super::Edition> {
                    self._has.r#edition().then_some(&self.r#edition)
                }
                pub fn mut_edition(
                    &mut self,
                ) -> ::core::option::Option<&mut super::Edition> {
                    self._has.r#edition().then_some(&mut self.r#edition)
                }
                pub fn set_edition(&mut self, value: super::Edition) {
                    self._has.set_edition(true);
                    self.r#edition = value.into();
                }
                pub fn clear_edition(&mut self) {
                    self._has.set_edition(false);
                }
                pub fn r#overridable_features(
                    &self,
                ) -> ::core::option::Option<&super::FeatureSet> {
                    self._has
                        .r#overridable_features()
                        .then_some(&self.r#overridable_features)
                }
                pub fn mut_overridable_features(
                    &mut self,
                ) -> ::core::option::Option<&mut super::FeatureSet> {
                    self._has
                        .r#overridable_features()
                        .then_some(&mut self.r#overridable_features)
                }
                pub fn set_overridable_features(&mut self, value: super::FeatureSet) {
                    self._has.set_overridable_features(true);
                    self.r#overridable_features = value.into();
                }
                pub fn clear_overridable_features(&mut self) {
                    self._has.set_overridable_features(false);
                }
                pub fn r#fixed_features(
                    &self,
                ) -> ::core::option::Option<&super::FeatureSet> {
                    self._has.r#fixed_features().then_some(&self.r#fixed_features)
                }
                pub fn mut_fixed_features(
                    &mut self,
                ) -> ::core::option::Option<&mut super::FeatureSet> {
                    self._has.r#fixed_features().then_some(&mut self.r#fixed_features)
                }
                pub fn set_fixed_features(&mut self, value: super::FeatureSet) {
                    self._has.set_fixed_features(true);
                    self.r#fixed_features = value.into();
                }
                pub fn clear_fixed_features(&mut self) {
                    self._has.set_fixed_features(false);
                }
            }
            impl ::micropb::MessageDecode for FeatureSetEditionDefault {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_edition(true);
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#overridable_features;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_overridable_features(true);
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#fixed_features;
                                {
                                    mut_ref.decode_len_delimited(decoder)?;
                                };
                                self._has.set_fixed_features(true);
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
                #[inline]
                pub fn r#minimum_edition(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                #[inline]
                pub fn set_minimum_edition(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 1;
                    } else {
                        *elem &= !1;
                    }
                }
                #[inline]
                pub fn r#maximum_edition(&self) -> bool {
                    (self.0[0] & 2) != 0
                }
                #[inline]
                pub fn set_maximum_edition(&mut self, val: bool) {
                    let elem = &mut self.0[0];
                    if val {
                        *elem |= 2;
                    } else {
                        *elem &= !2;
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct FeatureSetDefaults {
            pub r#defaults: ::std::vec::Vec<
                mod_FeatureSetDefaults::FeatureSetEditionDefault,
            >,
            pub r#minimum_edition: Edition,
            pub r#maximum_edition: Edition,
            pub _has: mod_FeatureSetDefaults::_Hazzer,
        }
        impl ::core::default::Default for FeatureSetDefaults {
            fn default() -> Self {
                Self {
                    r#defaults: ::core::default::Default::default(),
                    r#minimum_edition: ::core::default::Default::default(),
                    r#maximum_edition: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl FeatureSetDefaults {
            pub fn r#minimum_edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#minimum_edition().then_some(&self.r#minimum_edition)
            }
            pub fn mut_minimum_edition(
                &mut self,
            ) -> ::core::option::Option<&mut Edition> {
                self._has.r#minimum_edition().then_some(&mut self.r#minimum_edition)
            }
            pub fn set_minimum_edition(&mut self, value: Edition) {
                self._has.set_minimum_edition(true);
                self.r#minimum_edition = value.into();
            }
            pub fn clear_minimum_edition(&mut self) {
                self._has.set_minimum_edition(false);
            }
            pub fn r#maximum_edition(&self) -> ::core::option::Option<&Edition> {
                self._has.r#maximum_edition().then_some(&self.r#maximum_edition)
            }
            pub fn mut_maximum_edition(
                &mut self,
            ) -> ::core::option::Option<&mut Edition> {
                self._has.r#maximum_edition().then_some(&mut self.r#maximum_edition)
            }
            pub fn set_maximum_edition(&mut self, value: Edition) {
                self._has.set_maximum_edition(true);
                self.r#maximum_edition = value.into();
            }
            pub fn clear_maximum_edition(&mut self) {
                self._has.set_maximum_edition(false);
            }
        }
        impl ::micropb::MessageDecode for FeatureSetDefaults {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut val: mod_FeatureSetDefaults::FeatureSetEditionDefault = ::core::default::Default::default();
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
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_minimum_edition(true);
                        }
                        5u32 => {
                            let mut_ref = &mut self.r#maximum_edition;
                            {
                                let val = decoder.decode_int32().map(|n| Edition(n as _))?;
                                let val_ref = &val;
                                *mut_ref = val as _;
                            };
                            self._has.set_maximum_edition(true);
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        pub mod mod_SourceCodeInfo {
            pub mod mod_Location {
                #[derive(Debug, Default, PartialEq, Clone)]
                pub struct _Hazzer([u8; 1]);
                impl _Hazzer {
                    #[inline]
                    pub fn r#leading_comments(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_leading_comments(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#trailing_comments(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_trailing_comments(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct Location {
                pub r#path: ::std::vec::Vec<i32>,
                pub r#span: ::std::vec::Vec<i32>,
                pub r#leading_comments: ::std::string::String,
                pub r#trailing_comments: ::std::string::String,
                pub r#leading_detached_comments: ::std::vec::Vec<::std::string::String>,
                pub _has: mod_Location::_Hazzer,
            }
            impl ::core::default::Default for Location {
                fn default() -> Self {
                    Self {
                        r#path: ::core::default::Default::default(),
                        r#span: ::core::default::Default::default(),
                        r#leading_comments: ::core::default::Default::default(),
                        r#trailing_comments: ::core::default::Default::default(),
                        r#leading_detached_comments: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl Location {
                pub fn r#leading_comments(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#leading_comments().then_some(&self.r#leading_comments)
                }
                pub fn mut_leading_comments(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has
                        .r#leading_comments()
                        .then_some(&mut self.r#leading_comments)
                }
                pub fn set_leading_comments(&mut self, value: ::std::string::String) {
                    self._has.set_leading_comments(true);
                    self.r#leading_comments = value.into();
                }
                pub fn clear_leading_comments(&mut self) {
                    self._has.set_leading_comments(false);
                }
                pub fn r#trailing_comments(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#trailing_comments().then_some(&self.r#trailing_comments)
                }
                pub fn mut_trailing_comments(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has
                        .r#trailing_comments()
                        .then_some(&mut self.r#trailing_comments)
                }
                pub fn set_trailing_comments(&mut self, value: ::std::string::String) {
                    self._has.set_trailing_comments(true);
                    self.r#trailing_comments = value.into();
                }
                pub fn clear_trailing_comments(&mut self) {
                    self._has.set_trailing_comments(false);
                }
            }
            impl ::micropb::MessageDecode for Location {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                self._has.set_leading_comments(true);
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#trailing_comments;
                                {
                                    decoder
                                        .decode_string(mut_ref, ::micropb::Presence::Explicit)?;
                                };
                                self._has.set_trailing_comments(true);
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
        #[derive(Debug)]
        pub struct SourceCodeInfo {
            pub r#location: ::std::vec::Vec<mod_SourceCodeInfo::Location>,
        }
        impl ::core::default::Default for SourceCodeInfo {
            fn default() -> Self {
                Self {
                    r#location: ::core::default::Default::default(),
                }
            }
        }
        impl SourceCodeInfo {}
        impl ::micropb::MessageDecode for SourceCodeInfo {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut val: mod_SourceCodeInfo::Location = ::core::default::Default::default();
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
        pub mod mod_GeneratedCodeInfo {
            pub mod mod_Annotation {
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
                    #[inline]
                    pub fn r#source_file(&self) -> bool {
                        (self.0[0] & 1) != 0
                    }
                    #[inline]
                    pub fn set_source_file(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 1;
                        } else {
                            *elem &= !1;
                        }
                    }
                    #[inline]
                    pub fn r#begin(&self) -> bool {
                        (self.0[0] & 2) != 0
                    }
                    #[inline]
                    pub fn set_begin(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 2;
                        } else {
                            *elem &= !2;
                        }
                    }
                    #[inline]
                    pub fn r#end(&self) -> bool {
                        (self.0[0] & 4) != 0
                    }
                    #[inline]
                    pub fn set_end(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 4;
                        } else {
                            *elem &= !4;
                        }
                    }
                    #[inline]
                    pub fn r#semantic(&self) -> bool {
                        (self.0[0] & 8) != 0
                    }
                    #[inline]
                    pub fn set_semantic(&mut self, val: bool) {
                        let elem = &mut self.0[0];
                        if val {
                            *elem |= 8;
                        } else {
                            *elem &= !8;
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct Annotation {
                pub r#path: ::std::vec::Vec<i32>,
                pub r#source_file: ::std::string::String,
                pub r#begin: i32,
                pub r#end: i32,
                pub r#semantic: mod_Annotation::Semantic,
                pub _has: mod_Annotation::_Hazzer,
            }
            impl ::core::default::Default for Annotation {
                fn default() -> Self {
                    Self {
                        r#path: ::core::default::Default::default(),
                        r#source_file: ::core::default::Default::default(),
                        r#begin: ::core::default::Default::default(),
                        r#end: ::core::default::Default::default(),
                        r#semantic: ::core::default::Default::default(),
                        _has: ::core::default::Default::default(),
                    }
                }
            }
            impl Annotation {
                pub fn r#source_file(
                    &self,
                ) -> ::core::option::Option<&::std::string::String> {
                    self._has.r#source_file().then_some(&self.r#source_file)
                }
                pub fn mut_source_file(
                    &mut self,
                ) -> ::core::option::Option<&mut ::std::string::String> {
                    self._has.r#source_file().then_some(&mut self.r#source_file)
                }
                pub fn set_source_file(&mut self, value: ::std::string::String) {
                    self._has.set_source_file(true);
                    self.r#source_file = value.into();
                }
                pub fn clear_source_file(&mut self) {
                    self._has.set_source_file(false);
                }
                pub fn r#begin(&self) -> ::core::option::Option<&i32> {
                    self._has.r#begin().then_some(&self.r#begin)
                }
                pub fn mut_begin(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#begin().then_some(&mut self.r#begin)
                }
                pub fn set_begin(&mut self, value: i32) {
                    self._has.set_begin(true);
                    self.r#begin = value.into();
                }
                pub fn clear_begin(&mut self) {
                    self._has.set_begin(false);
                }
                pub fn r#end(&self) -> ::core::option::Option<&i32> {
                    self._has.r#end().then_some(&self.r#end)
                }
                pub fn mut_end(&mut self) -> ::core::option::Option<&mut i32> {
                    self._has.r#end().then_some(&mut self.r#end)
                }
                pub fn set_end(&mut self, value: i32) {
                    self._has.set_end(true);
                    self.r#end = value.into();
                }
                pub fn clear_end(&mut self) {
                    self._has.set_end(false);
                }
                pub fn r#semantic(
                    &self,
                ) -> ::core::option::Option<&mod_Annotation::Semantic> {
                    self._has.r#semantic().then_some(&self.r#semantic)
                }
                pub fn mut_semantic(
                    &mut self,
                ) -> ::core::option::Option<&mut mod_Annotation::Semantic> {
                    self._has.r#semantic().then_some(&mut self.r#semantic)
                }
                pub fn set_semantic(&mut self, value: mod_Annotation::Semantic) {
                    self._has.set_semantic(true);
                    self.r#semantic = value.into();
                }
                pub fn clear_semantic(&mut self) {
                    self._has.set_semantic(false);
                }
            }
            impl ::micropb::MessageDecode for Annotation {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
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
                                self._has.set_source_file(true);
                            }
                            3u32 => {
                                let mut_ref = &mut self.r#begin;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_begin(true);
                            }
                            4u32 => {
                                let mut_ref = &mut self.r#end;
                                {
                                    let val = decoder.decode_int32()?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_end(true);
                            }
                            5u32 => {
                                let mut_ref = &mut self.r#semantic;
                                {
                                    let val = decoder
                                        .decode_int32()
                                        .map(|n| mod_Annotation::Semantic(n as _))?;
                                    let val_ref = &val;
                                    *mut_ref = val as _;
                                };
                                self._has.set_semantic(true);
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
        #[derive(Debug)]
        pub struct GeneratedCodeInfo {
            pub r#annotation: ::std::vec::Vec<mod_GeneratedCodeInfo::Annotation>,
        }
        impl ::core::default::Default for GeneratedCodeInfo {
            fn default() -> Self {
                Self {
                    r#annotation: ::core::default::Default::default(),
                }
            }
        }
        impl GeneratedCodeInfo {}
        impl ::micropb::MessageDecode for GeneratedCodeInfo {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut val: mod_GeneratedCodeInfo::Annotation = ::core::default::Default::default();
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
