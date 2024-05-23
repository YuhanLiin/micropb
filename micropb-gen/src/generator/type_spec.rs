use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{field_descriptor_proto::Type, FieldDescriptorProto};
use quote::quote;
use syn::Ident;

use crate::{
    config::IntSize,
    utils::{path_suffix, unescape_c_escape_string},
};

use super::{CurrentConfig, Generator};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum PbInt {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Sfixed32,
    Sfixed64,
    Fixed32,
    Fixed64,
}

impl PbInt {
    fn is_signed(&self) -> bool {
        matches!(
            self,
            PbInt::Int32
                | PbInt::Int64
                | PbInt::Sint32
                | PbInt::Sint64
                | PbInt::Sfixed32
                | PbInt::Sfixed64
        )
    }

    fn generate_decode_func(&self, int_size: &IntSize) -> Ident {
        let func = match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => "decode_int64",
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => "decode_varint64",
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => "decode_sint64",
            PbInt::Fixed64 if matches!(int_size, IntSize::S64) => "decode_fixed64",
            PbInt::Sfixed64 if matches!(int_size, IntSize::S64) => "decode_sfixed64",

            PbInt::Int32 | PbInt::Int64 => "decode_int32",
            PbInt::Uint32 | PbInt::Uint64 => "decode_varint32",
            PbInt::Sint32 | PbInt::Sint64 => "decode_sint32",
            PbInt::Sfixed32 => "decode_sfixed32",
            PbInt::Fixed32 => "decode_fixed32",
            PbInt::Sfixed64 => "decode_sfixed64_as_32",
            PbInt::Fixed64 => "decode_fixed64_as_32",
        };
        Ident::new(func, Span::call_site())
    }

    fn generate_sizeof(&self, int_size: &IntSize, val_ref: &Ident) -> TokenStream {
        match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_int64(* #val_ref as _) }
            }
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_varint64(* #val_ref as _) }
            }
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_sint64(* #val_ref as _) }
            }

            PbInt::Int32 | PbInt::Int64 => {
                quote! { ::micropb::size::sizeof_int32(* #val_ref as _) }
            }
            PbInt::Uint32 | PbInt::Uint64 => {
                quote! { ::micropb::size::sizeof_varint32(* #val_ref as _) }
            }
            PbInt::Sint32 | PbInt::Sint64 => {
                quote! { ::micropb::size::sizeof_sint32(* #val_ref as _) }
            }

            PbInt::Sfixed32 => quote! { 4 },
            PbInt::Fixed32 => quote! { 4 },
            PbInt::Sfixed64 => quote! { 8 },
            PbInt::Fixed64 => quote! { 8 },
        }
    }

    fn generate_encode_func(&self, int_size: &IntSize) -> Ident {
        let func = match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => "encode_int64",
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => "encode_varint64",
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => "encode_sint64",
            PbInt::Sfixed64 if matches!(int_size, IntSize::S64) => "encode_sfixed64",
            PbInt::Fixed64 if matches!(int_size, IntSize::S64) => "encode_fixed64",

            PbInt::Int32 | PbInt::Int64 => "encode_int32",
            PbInt::Uint32 | PbInt::Uint64 => "encode_varint32",
            PbInt::Sint32 | PbInt::Sint64 => "encode_sint32",
            PbInt::Sfixed32 => "encode_sfixed32",
            PbInt::Fixed32 => "encode_fixed32",
            PbInt::Sfixed64 => "encode_sfixed64_as_32",
            PbInt::Fixed64 => "encode_fixed64_as_32",
        };
        Ident::new(func, Span::call_site())
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum TypeSpec {
    Message(String),
    Enum(String),
    Float,
    Double,
    Bool,
    Int(PbInt, IntSize),
    String {
        type_path: syn::Path,
        max_bytes: Option<u32>,
    },
    Bytes {
        type_path: syn::Path,
        max_bytes: Option<u32>,
    },
}

impl TypeSpec {
    pub(crate) fn fixed_size(&self) -> Option<usize> {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => Some(4),
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => Some(8),
            TypeSpec::Bool => Some(1),
            _ => None,
        }
    }

    pub(crate) fn from_proto(
        proto: &FieldDescriptorProto,
        type_conf: &CurrentConfig,
    ) -> Result<Self, String> {
        let conf = &type_conf.config;
        let res = match proto.r#type() {
            Type::Group => panic!("Groups are unsupported"),
            Type::Double => TypeSpec::Double,
            Type::Float => TypeSpec::Float,
            Type::Bool => TypeSpec::Bool,
            Type::String => TypeSpec::String {
                type_path: conf.string_type_parsed()?.ok_or_else(|| {
                    "Field is of type `string`, but string_type was not configured for it"
                        .to_owned()
                })?,
                max_bytes: conf.max_bytes,
            },
            Type::Bytes => TypeSpec::Bytes {
                type_path: conf.vec_type_parsed()?.ok_or_else(|| {
                    "Field is of type `bytes`, but vec_type was not configured for it".to_owned()
                })?,
                max_bytes: conf.max_bytes,
            },
            Type::Message => TypeSpec::Message(proto.type_name().to_owned()),
            Type::Enum => TypeSpec::Enum(proto.type_name().to_owned()),
            Type::Uint32 => TypeSpec::Int(PbInt::Uint32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Int64 => TypeSpec::Int(PbInt::Int64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Uint64 => TypeSpec::Int(PbInt::Uint64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Int32 => TypeSpec::Int(PbInt::Int32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Fixed64 => TypeSpec::Int(PbInt::Fixed64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Fixed32 => TypeSpec::Int(PbInt::Fixed32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sfixed32 => TypeSpec::Int(PbInt::Sfixed32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sfixed64 => TypeSpec::Int(PbInt::Sfixed64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Sint32 => TypeSpec::Int(PbInt::Sint32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sint64 => TypeSpec::Int(PbInt::Sint64, conf.int_size.unwrap_or(IntSize::S64)),
        };
        Ok(res)
    }

    pub(crate) fn generate_rust_type(&self, gen: &Generator) -> TokenStream {
        match self {
            TypeSpec::Int(pbint, itype) => {
                let typ = itype.type_name(pbint.is_signed());
                quote! { #typ }
            }
            TypeSpec::Float => quote! {f32},
            TypeSpec::Double => quote! {f64},
            TypeSpec::Bool => quote! {bool},
            TypeSpec::String {
                type_path,
                max_bytes,
            } => {
                let max_bytes = max_bytes.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path #(<#max_bytes>)* }
            }
            TypeSpec::Bytes {
                type_path,
                max_bytes,
            } => {
                let max_bytes = max_bytes.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path <u8 #(, #max_bytes)* > }
            }
            TypeSpec::Message(tname) | TypeSpec::Enum(tname) => {
                let rust_type = gen.resolve_type_name(tname);
                quote! { #rust_type }
            }
        }
    }

    pub(crate) fn generate_default(&self, default: &str, gen: &Generator) -> TokenStream {
        match self {
            TypeSpec::String { .. } => {
                quote! { ::micropb::PbString::pb_from_str(#default).unwrap_or_default() }
            }
            TypeSpec::Bytes { .. } => {
                let bytes = Literal::byte_string(&unescape_c_escape_string(default));
                quote! { ::micropb::PbVec::pb_from_slice(#bytes).unwrap_or_default() }
            }
            TypeSpec::Message(_) => {
                unreachable!("message fields shouldn't have custom defaults")
            }
            TypeSpec::Enum(tpath) => {
                let enum_path = gen.resolve_type_name(tpath);
                let enum_name =
                    Ident::new(&path_suffix(tpath).to_case(Case::Pascal), Span::call_site());
                let variant = gen.enum_variant_name(default, &enum_name);
                quote! { #enum_path::#variant }
            }
            _ => {
                let default: TokenStream =
                    syn::parse_str(default).expect("default value tokenization error");
                quote! { #default as _ }
            }
        }
    }

    pub(crate) fn wire_type(&self) -> u8 {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => {
                micropb::WIRE_TYPE_I32
            }
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => {
                micropb::WIRE_TYPE_I64
            }
            TypeSpec::Enum(_)
            | TypeSpec::Bool
            | TypeSpec::Int(
                PbInt::Int32
                | PbInt::Int64
                | PbInt::Uint32
                | PbInt::Uint64
                | PbInt::Sint32
                | PbInt::Sint64,
                _,
            ) => micropb::WIRE_TYPE_VARINT,
            TypeSpec::Message(_) | TypeSpec::String { .. } | TypeSpec::Bytes { .. } => {
                micropb::WIRE_TYPE_LEN
            }
        }
    }

    pub(crate) fn generate_implicit_presence_check(&self, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(_) => quote! { true },
            TypeSpec::Enum(_) => quote! { #val_ref.0 != 0 },
            TypeSpec::Float | TypeSpec::Double => quote! { *#val_ref != 0.0 },
            TypeSpec::Bool => quote! { *#val_ref },
            TypeSpec::Int(_, _) => quote! { *#val_ref != 0 },
            TypeSpec::String { .. } => quote! { !#val_ref.is_empty() },
            TypeSpec::Bytes { .. } => quote! { !#val_ref.is_empty() },
        }
    }

    /// Generate decode value expressions (Result<T, DecodeError>) for "packable" types
    pub(crate) fn generate_decode_val(
        &self,
        gen: &Generator,
        decoder: &Ident,
    ) -> Option<TokenStream> {
        match self {
            TypeSpec::Float => Some(quote! { #decoder.decode_float() }),
            TypeSpec::Double => Some(quote! { #decoder.decode_double() }),
            TypeSpec::Bool => Some(quote! { #decoder.decode_bool() }),
            TypeSpec::Int(pbint, int_size) => {
                let func = pbint.generate_decode_func(int_size);
                Some(quote! { #decoder.#func() })
            }
            // Enum is actually packable due to https://github.com/protocolbuffers/protobuf/issues/15480
            TypeSpec::Enum(tpath) => {
                let enum_path = gen.resolve_type_name(tpath);
                Some(quote! { #decoder.decode_int32().map(|n| #enum_path(n as _)) })
            }
            _ => None,
        }
    }

    pub(crate) fn generate_decode_mut(
        &self,
        gen: &Generator,
        implicit_presence: bool,
        decoder: &Ident,
        mut_ref: &Ident,
    ) -> TokenStream {
        let presence = if implicit_presence {
            "Implicit"
        } else {
            "Explicit"
        };
        let presence_ident = Ident::new(presence, Span::call_site());

        match self {
            TypeSpec::Message(_) => quote! { #mut_ref.decode_len_delimited(#decoder)?; },
            TypeSpec::Enum(_)
            | TypeSpec::Float
            | TypeSpec::Double
            | TypeSpec::Bool
            | TypeSpec::Int(..) => {
                let val_expr = self
                    .generate_decode_val(gen, decoder)
                    .expect("ints should be packable");
                let val_ref = Ident::new("val_ref", Span::call_site());
                let setter = if implicit_presence {
                    let presence_check = self.generate_implicit_presence_check(&val_ref);
                    quote! {
                        if #presence_check {
                            *#mut_ref = val as _;
                        }
                    }
                } else {
                    quote! { *#mut_ref = val as _; }
                };
                quote! {
                    let val = #val_expr?;
                    let #val_ref = &val;
                    #setter
                }
            }
            TypeSpec::String { .. } => {
                quote! { #decoder.decode_string(#mut_ref, ::micropb::Presence::#presence_ident)?; }
            }
            TypeSpec::Bytes { .. } => {
                quote! { #decoder.decode_bytes(#mut_ref, ::micropb::Presence::#presence_ident)?; }
            }
        }
    }

    pub(crate) fn generate_sizeof(&self, gen: &Generator, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(_) => {
                quote! { ::micropb::size::sizeof_len_record(#val_ref.compute_size()) }
            }
            TypeSpec::Enum(_) => quote! { ::micropb::size::sizeof_int32(#val_ref.0 as _) },
            TypeSpec::Float => quote! { 4 },
            TypeSpec::Double => quote! { 8 },
            TypeSpec::Bool => quote! { 1 },
            TypeSpec::Int(pbint, int_size) => pbint.generate_sizeof(int_size, val_ref),
            TypeSpec::String { .. } => {
                quote! { ::micropb::size::sizeof_len_record(#val_ref.len()) }
            }
            TypeSpec::Bytes { .. } => quote! { ::micropb::size::sizeof_len_record(#val_ref.len()) },
        }
    }

    pub(crate) fn generate_encode_expr(
        &self,
        gen: &Generator,
        encoder: &Ident,
        val_ref: &Ident,
    ) -> TokenStream {
        match self {
            TypeSpec::Message(_) => quote! { #val_ref.encode_len_delimited(#encoder) },
            TypeSpec::Enum(_) => quote! { #encoder.encode_int32(#val_ref.0 as _) },
            TypeSpec::Float => quote! { #encoder.encode_float(* #val_ref) },
            TypeSpec::Double => quote! { #encoder.encode_double(* #val_ref) },
            TypeSpec::Bool => quote! { #encoder.encode_bool(* #val_ref) },
            TypeSpec::Int(pbint, int_size) => {
                let func = pbint.generate_encode_func(int_size);
                quote! { #encoder.#func(* #val_ref as _) }
            }
            TypeSpec::String { .. } => quote! { #encoder.encode_string(#val_ref) },
            TypeSpec::Bytes { .. } => quote! { #encoder.encode_bytes(#val_ref) },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::config::Config;

    use super::*;

    fn field_proto(typ: Type, type_name: &str) -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: Some("name".to_owned()),
            number: Some(0),
            r#type: Some(typ.into()),
            type_name: Some(type_name.to_owned()),
            ..Default::default()
        }
    }

    #[test]
    fn from_proto() {
        let mut config = Box::new(
            Config::new()
                .string_type("string::String")
                .vec_type("vec::Vec")
                .max_bytes(10),
        );

        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Float, ""), &type_conf).unwrap(),
            TypeSpec::Float
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Double, ""), &type_conf).unwrap(),
            TypeSpec::Double
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bool, ""), &type_conf).unwrap(),
            TypeSpec::Bool
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::String, ""), &type_conf).unwrap(),
            TypeSpec::String {
                type_path: syn::parse_str("string::String").unwrap(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bytes, ""), &type_conf).unwrap(),
            TypeSpec::Bytes {
                type_path: syn::parse_str("vec::Vec").unwrap(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Message, ".msg.Message"), &type_conf).unwrap(),
            TypeSpec::Message(".msg.Message".to_owned())
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Enum, ".Enum"), &type_conf).unwrap(),
            TypeSpec::Enum(".Enum".to_owned())
        );

        config.max_bytes = None;
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::String, ""), &type_conf).unwrap(),
            TypeSpec::String {
                type_path: syn::parse_str("string::String").unwrap(),
                max_bytes: None
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bytes, ""), &type_conf).unwrap(),
            TypeSpec::Bytes {
                type_path: syn::parse_str("vec::Vec").unwrap(),
                max_bytes: None
            }
        );
    }

    #[test]
    fn from_proto_num() {
        let mut config = Box::new(Config::new());
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Sint32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Sint32, IntSize::S32)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Int64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Int64, IntSize::S64)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Fixed32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Fixed32, IntSize::S32)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Uint64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Uint64, IntSize::S64)
        );

        config.int_size = Some(IntSize::S8);
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Sint32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Sint32, IntSize::S8)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Uint64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Uint64, IntSize::S8)
        );
    }

    #[test]
    fn tspec_rust_type() {
        let mut gen = Generator::default();
        assert_eq!(TypeSpec::Bool.generate_rust_type(&gen).to_string(), "bool");
        assert_eq!(TypeSpec::Float.generate_rust_type(&gen).to_string(), "f32");
        assert_eq!(TypeSpec::Double.generate_rust_type(&gen).to_string(), "f64");
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntSize::S32)
                .generate_rust_type(&gen)
                .to_string(),
            "i32"
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Fixed32, IntSize::S32)
                .generate_rust_type(&gen)
                .to_string(),
            "u32"
        );
        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("heapless::String").unwrap(),
                max_bytes: Some(10)
            }
            .generate_rust_type(&gen)
            .to_string(),
            quote! { heapless::String<10> }.to_string()
        );
        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("heapless::String").unwrap(),
                max_bytes: None
            }
            .generate_rust_type(&gen)
            .to_string(),
            quote! { heapless::String }.to_string()
        );
        assert_eq!(
            TypeSpec::Enum(".package.Enum".to_owned())
                .generate_rust_type(&gen)
                .to_string(),
            quote! { package::Enum }.to_string()
        );
        assert_eq!(
            TypeSpec::Message(".package.Msg".to_owned())
                .generate_rust_type(&gen)
                .to_string(),
            quote! { package::Msg }.to_string()
        );

        gen.pkg_path.push("package".to_owned());
        assert_eq!(
            TypeSpec::Enum(".package.Enum".to_owned())
                .generate_rust_type(&gen)
                .to_string(),
            quote! { Enum }.to_string()
        );
        assert_eq!(
            TypeSpec::Message(".package.Msg".to_owned())
                .generate_rust_type(&gen)
                .to_string(),
            quote! { Msg }.to_string()
        );
    }

    #[test]
    fn tspec_default() {
        let gen = Generator::default();
        assert_eq!(
            TypeSpec::Bool.generate_default("true", &gen).to_string(),
            quote! { true as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Bool.generate_default("false", &gen).to_string(),
            quote! { false as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Float.generate_default("0.1", &gen).to_string(),
            quote! { 0.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Double.generate_default("-4.1", &gen).to_string(),
            quote! { -4.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntSize::S8)
                .generate_default("-99", &gen)
                .to_string(),
            quote! { -99 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("Vec").unwrap(),
                max_bytes: None
            }
            .generate_default("abc\n\tddd", &gen)
            .to_string(),
            quote! { ::micropb::PbString::pb_from_str("abc\n\tddd").unwrap_or_default() }
                .to_string()
        );
        assert_eq!(
            TypeSpec::Bytes {
                type_path: syn::parse_str("Vec").unwrap(),
                max_bytes: None
            }
            .generate_default("abc\\n\\t\\a\\xA0ddd", &gen)
            .to_string(),
            quote! { ::micropb::PbVec::pb_from_slice(b"abc\n\t\x07\xA0ddd").unwrap_or_default() }
                .to_string()
        );
    }
}
