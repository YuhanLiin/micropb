use convert_case::{Case, Casing};
use micropb::size::{
    sizeof_len_record, sizeof_sint32, sizeof_sint64, sizeof_varint32, sizeof_varint64,
};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Ident, Lifetime};

use crate::{
    config::IntSize,
    descriptor::{FieldDescriptorProto, FieldDescriptorProto_::Type},
    generator::sanitized_ident,
    utils::{path_suffix, unescape_c_escape_string},
};

use super::{CurrentConfig, Generator};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
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

/// Find the first lifetime embedded in a type
pub(crate) fn find_lifetime_from_type(ty: &syn::Type) -> Option<&Lifetime> {
    match ty {
        syn::Type::Array(tarr) => find_lifetime_from_type(&tarr.elem),
        syn::Type::Group(t) => find_lifetime_from_type(&t.elem),
        syn::Type::Paren(t) => find_lifetime_from_type(&t.elem),
        syn::Type::Reference(tref) => tref.lifetime.as_ref(),
        syn::Type::Path(tpath) => find_lifetime_from_path(&tpath.path),
        _ => None,
    }
}

/// Find the first lifetime embedded in a type path
pub(crate) fn find_lifetime_from_path(tpath: &syn::Path) -> Option<&Lifetime> {
    if let syn::PathArguments::AngleBracketed(args) =
        &tpath.segments.last().expect("empty type path").arguments
    {
        for arg in &args.args {
            match arg {
                syn::GenericArgument::Lifetime(lt) => return Some(lt),
                syn::GenericArgument::Type(ty) => return find_lifetime_from_type(ty),
                _ => (),
            }
        }
    }
    None
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum TypeSpec {
    Message(String, Option<syn::Lifetime>),
    Enum(String),
    Float,
    Double,
    Bool,
    Int(PbInt, IntSize),
    String {
        type_path: syn::Type,
        max_bytes: Option<u32>,
    },
    Bytes {
        type_path: syn::Type,
        max_bytes: Option<u32>,
    },
}

impl TypeSpec {
    pub(crate) fn find_lifetime(&self) -> Option<&Lifetime> {
        match self {
            TypeSpec::Message(_, lifetime) => lifetime.as_ref(),
            TypeSpec::Bytes { type_path, .. } | TypeSpec::String { type_path, .. } => {
                find_lifetime_from_type(type_path)
            }
            _ => None,
        }
    }

    fn max_size(&self) -> Option<usize> {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => Some(4),
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => Some(8),
            TypeSpec::Bool => Some(1),

            // negative VARINT values will always take up 10 bytes
            TypeSpec::Int(PbInt::Int32 | PbInt::Int64, _) | TypeSpec::Enum(_) => Some(10),

            // positive VARINT size depends on the max size of the represented int type
            TypeSpec::Int(PbInt::Uint32, intsize) => Some(sizeof_varint32(
                intsize.max_value().try_into().unwrap_or(u32::MAX),
            )),
            TypeSpec::Int(PbInt::Uint64, intsize) => Some(sizeof_varint64(intsize.max_value())),
            TypeSpec::Int(PbInt::Sint32, intsize) => Some(sizeof_sint32(
                intsize.min_value().try_into().unwrap_or(i32::MAX),
            )),
            TypeSpec::Int(PbInt::Sint64, intsize) => Some(sizeof_sint64(intsize.min_value())),

            TypeSpec::Bytes { max_bytes, .. } | TypeSpec::String { max_bytes, .. } => {
                max_bytes.map(|max| sizeof_len_record(max as usize))
            }

            TypeSpec::Message(..) => None,
        }
    }

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
        let res = match proto.r#type {
            Type::Group => return Err("Group fields are unsupported".to_owned()),
            Type::Double => TypeSpec::Double,
            Type::Float => TypeSpec::Float,
            Type::Bool => TypeSpec::Bool,
            Type::String => TypeSpec::String {
                type_path: conf
                    .string_type_parsed(conf.max_bytes)?
                    .ok_or_else(|| "string_type not configured for string field".to_owned())?,
                max_bytes: conf.max_bytes,
            },
            Type::Bytes => TypeSpec::Bytes {
                type_path: conf
                    .bytes_type_parsed(conf.max_bytes)?
                    .ok_or_else(|| "bytes_type not configured for bytes field".to_owned())?,
                max_bytes: conf.max_bytes,
            },
            Type::Message => {
                TypeSpec::Message(proto.type_name.clone(), conf.field_lifetime_parsed()?)
            }
            Type::Enum => TypeSpec::Enum(proto.type_name.clone()),
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
            t => return Err(format!("Unknown type specifier {}", t.0)),
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
            TypeSpec::String { type_path, .. } => quote! { #type_path },
            TypeSpec::Bytes { type_path, .. } => quote! { #type_path },

            TypeSpec::Message(tname, lifetime) => {
                let rust_type = gen.resolve_type_name(tname);
                quote! { #rust_type<#lifetime> }
            }
            TypeSpec::Enum(tname) => {
                let rust_type = gen.resolve_type_name(tname);
                quote! { #rust_type }
            }
        }
    }

    pub(crate) fn generate_max_size(&self, gen: &Generator) -> TokenStream {
        if let TypeSpec::Message(tname, _) = self {
            let rust_type = gen.resolve_type_name(tname);
            return quote! { ::micropb::const_map!(<#rust_type as ::micropb::MessageEncode>::MAX_SIZE, |size| ::micropb::size::sizeof_len_record(size)) };
        }

        self.max_size()
            .map(Literal::usize_suffixed)
            .map(|lit| quote! {::core::option::Option::Some(#lit)})
            .unwrap_or(quote! {::core::option::Option::<usize>::None})
    }

    pub(crate) fn generate_default(
        &self,
        default: &str,
        gen: &Generator,
    ) -> Result<TokenStream, String> {
        let out = match self {
            TypeSpec::String { max_bytes, .. } => {
                match *max_bytes {
                    Some(max_bytes) if default.len() > max_bytes as usize =>
                        return Err(format!("String field is limited to {max_bytes} bytes, but its default value is {} bytes", default.len())),
                    _ => quote! { ::core::convert::TryFrom::try_from(#default).unwrap_or_default() }
                }
            }

            TypeSpec::Bytes { max_bytes, .. } => {
                let bytes = unescape_c_escape_string(default);
                let default_bytes = Literal::byte_string(&bytes);
                match *max_bytes {
                    Some(max_bytes) if bytes.len() > max_bytes as usize =>
                        return Err(format!("Bytes field is limited to {max_bytes} bytes, but its default value is {} bytes", bytes.len())),
                    _ => quote! { ::core::convert::TryFrom::try_from(#default_bytes.as_slice()).unwrap_or_default() }
                }
            }

            TypeSpec::Message(..) => {
                unreachable!("message fields shouldn't have custom defaults")
            }

            TypeSpec::Enum(tpath) => {
                let enum_path = gen.resolve_type_name(tpath);
                let enum_name =
                    sanitized_ident(&path_suffix(tpath).to_case(Case::Pascal));
                let variant = gen.enum_variant_name(default, &enum_name);
                quote! { #enum_path::#variant }
            }

            _ => {
                let default: TokenStream =
                    syn::parse_str(default).expect("default value tokenization error");
                quote! { #default as _ }
            }
        };
        Ok(out)
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
            TypeSpec::Message(..) | TypeSpec::String { .. } | TypeSpec::Bytes { .. } => {
                micropb::WIRE_TYPE_LEN
            }
        }
    }

    pub(crate) fn generate_implicit_presence_check(&self, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(..) => quote! {},
            TypeSpec::Enum(_) => quote! { if #val_ref.0 != 0 },
            TypeSpec::Float | TypeSpec::Double => quote! { if *#val_ref != 0.0 },
            TypeSpec::Bool => quote! { if *#val_ref },
            TypeSpec::Int(_, _) => quote! { if *#val_ref != 0 },
            TypeSpec::String { .. } => quote! { if !#val_ref.is_empty() },
            TypeSpec::Bytes { .. } => quote! { if !#val_ref.is_empty() },
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
            TypeSpec::Message(..) => quote! { #mut_ref.decode_len_delimited(#decoder)?; },
            TypeSpec::Enum(_)
            | TypeSpec::Float
            | TypeSpec::Double
            | TypeSpec::Bool
            | TypeSpec::Int(..) => {
                let val_expr = self
                    .generate_decode_val(gen, decoder)
                    .expect("ints should be packable");
                let setter = if implicit_presence {
                    let val_ref = Ident::new("val_ref", Span::call_site());
                    let presence_check = self.generate_implicit_presence_check(&val_ref);
                    quote! {
                        let #val_ref = &val;
                        #presence_check {
                            *#mut_ref = val as _;
                        }
                    }
                } else {
                    quote! { *#mut_ref = val as _; }
                };
                quote! {
                    let val = #val_expr?;
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

    pub(crate) fn generate_sizeof(&self, _gen: &Generator, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(..) => {
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
        _gen: &Generator,
        encoder: &Ident,
        val_ref: &Ident,
    ) -> TokenStream {
        match self {
            TypeSpec::Message(..) => quote! { #val_ref.encode_len_delimited(#encoder) },
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

    #[test]
    fn find_lifetime() {
        let ty: syn::Type = syn::parse_str("Vec").unwrap();
        assert!(find_lifetime_from_type(&ty).is_none());
        let ty: syn::Type = syn::parse_str("Vec<u8>").unwrap();
        assert!(find_lifetime_from_type(&ty).is_none());
        let ty: syn::Type = syn::parse_str("std::Vec<'a>").unwrap();
        assert!(find_lifetime_from_type(&ty).is_some());
        let ty: syn::Type = syn::parse_str("&'a [u8]").unwrap();
        assert!(find_lifetime_from_type(&ty).is_some());
        let ty: syn::Type = syn::parse_str("[&'a u8; 10]").unwrap();
        assert!(find_lifetime_from_type(&ty).is_some());
        let ty: syn::Type = syn::parse_str("([&'a u8; 10])").unwrap();
        assert!(find_lifetime_from_type(&ty).is_some());
        let ty: syn::Type = syn::parse_str("std::Option<std::Vec<'a>>").unwrap();
        assert!(find_lifetime_from_type(&ty).is_some());
    }

    #[test]
    fn max_size() {
        assert_eq!(TypeSpec::Float.max_size(), Some(4));
        assert_eq!(TypeSpec::Double.max_size(), Some(8));
        assert_eq!(TypeSpec::Bool.max_size(), Some(1));
        assert_eq!(TypeSpec::Enum("test".to_string()).max_size(), Some(10));
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntSize::S8).max_size(),
            Some(10)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Int64, IntSize::S8).max_size(),
            Some(10)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Fixed32, IntSize::S8).max_size(),
            Some(4)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Fixed64, IntSize::S8).max_size(),
            Some(8)
        );

        // uint types
        assert_eq!(
            TypeSpec::Int(PbInt::Uint32, IntSize::S8).max_size(),
            Some(2)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Uint32, IntSize::S32).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Uint32, IntSize::S64).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Uint64, IntSize::S16).max_size(),
            Some(3)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Uint64, IntSize::S32).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Uint64, IntSize::S64).max_size(),
            Some(10)
        );

        // sint types
        assert_eq!(
            TypeSpec::Int(PbInt::Sint32, IntSize::S16).max_size(),
            Some(3)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Sint32, IntSize::S32).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Sint32, IntSize::S64).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Sint64, IntSize::S16).max_size(),
            Some(3)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Sint64, IntSize::S32).max_size(),
            Some(5)
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Sint64, IntSize::S64).max_size(),
            Some(10)
        );

        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("test").unwrap(),
                max_bytes: Some(12)
            }
            .max_size(),
            Some(13)
        );
        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("test").unwrap(),
                max_bytes: None
            }
            .max_size(),
            None
        );

        assert_eq!(
            TypeSpec::Bytes {
                type_path: syn::parse_str("test").unwrap(),
                max_bytes: Some(12)
            }
            .max_size(),
            Some(13)
        );
        assert_eq!(
            TypeSpec::Bytes {
                type_path: syn::parse_str("test").unwrap(),
                max_bytes: None
            }
            .max_size(),
            None
        );
    }

    fn field_proto(typ: Type, type_name: &str) -> FieldDescriptorProto {
        let mut f = FieldDescriptorProto::default();
        f.set_name("name".to_owned());
        f.set_number(0);
        f.set_type(typ);
        f.set_type_name(type_name.to_owned());
        f
    }

    #[test]
    fn from_proto() {
        let mut config = Box::new(
            Config::new()
                .string_type("string::String<$N>")
                .bytes_type("vec::Vec<u8, $N>")
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
                type_path: syn::parse_str("string::String<10>").unwrap(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bytes, ""), &type_conf).unwrap(),
            TypeSpec::Bytes {
                type_path: syn::parse_str("vec::Vec<u8, 10>").unwrap(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Message, ".msg.Message"), &type_conf).unwrap(),
            TypeSpec::Message(".msg.Message".to_owned(), None)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Enum, ".Enum"), &type_conf).unwrap(),
            TypeSpec::Enum(".Enum".to_owned())
        );

        config.string_type = Some("string::String".to_owned());
        config.bytes_type = Some("Bytes".to_owned());
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
                type_path: syn::parse_str("Bytes").unwrap(),
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
    fn tspec_default() {
        let gen = Generator::new();
        assert_eq!(
            TypeSpec::Bool
                .generate_default("true", &gen)
                .unwrap()
                .to_string(),
            quote! { true as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Bool
                .generate_default("false", &gen)
                .unwrap()
                .to_string(),
            quote! { false as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Float
                .generate_default("0.1", &gen)
                .unwrap()
                .to_string(),
            quote! { 0.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Double
                .generate_default("-4.1", &gen)
                .unwrap()
                .to_string(),
            quote! { -4.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntSize::S8)
                .generate_default("-99", &gen)
                .unwrap()
                .to_string(),
            quote! { -99 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::String {
                type_path: syn::parse_str("Vec").unwrap(),
                max_bytes: None
            }
            .generate_default("abc\n\tddd", &gen)
            .unwrap()
            .to_string(),
            quote! { ::core::convert::TryFrom::try_from("abc\n\tddd").unwrap_or_default() }
                .to_string()
        );
        assert_eq!(
            TypeSpec::Bytes {
                type_path: syn::parse_str("Vec").unwrap(),
                max_bytes: None
            }
            .generate_default("abc\\n\\t\\a\\xA0ddd", &gen)
            .unwrap()
            .to_string(),
            quote! { ::core::convert::TryFrom::try_from(b"abc\n\t\x07\xA0ddd".as_slice()).unwrap_or_default() }
                .to_string()
        );
    }
}
