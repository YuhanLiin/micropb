use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{field_descriptor_proto::Type, FieldDescriptorProto};
use quote::quote;
use syn::Ident;

use crate::{
    config::IntType,
    utils::{path_suffix, unescape_c_escape_string},
};

use super::{CurrentConfig, Generator};

pub(crate) enum PbInt {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sfixed32,
    Sint64,
    Sfixed64,
    Fixed32,
    Fixed64,
}

pub(crate) enum TypeSpec {
    Message(String),
    Enum(String),
    Float,
    Double,
    Bool,
    Int(PbInt, IntType),
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
    pub(crate) fn from_proto(proto: &FieldDescriptorProto, type_conf: &CurrentConfig) -> Self {
        let conf = &type_conf.config;
        match proto.r#type() {
            Type::Group => panic!("Groups are unsupported"),
            Type::Double => TypeSpec::Double,
            Type::Float => TypeSpec::Float,
            Type::Bool => TypeSpec::Bool,
            Type::String => TypeSpec::String {
                type_path: conf.string_type_parsed().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Bytes => TypeSpec::Bytes {
                type_path: conf.vec_type_parsed().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Message => TypeSpec::Message(proto.type_name().to_owned()),
            Type::Enum => TypeSpec::Enum(proto.type_name().to_owned()),
            Type::Uint32 => TypeSpec::Int(PbInt::Uint32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Int64 => TypeSpec::Int(PbInt::Int64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Uint64 => TypeSpec::Int(PbInt::Uint64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Int32 => TypeSpec::Int(PbInt::Int32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Fixed64 => TypeSpec::Int(PbInt::Fixed64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Fixed32 => TypeSpec::Int(PbInt::Fixed32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sfixed32 => TypeSpec::Int(PbInt::Sfixed32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sfixed64 => TypeSpec::Int(PbInt::Sfixed64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sint32 => TypeSpec::Int(PbInt::Sint32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sint64 => TypeSpec::Int(PbInt::Sint64, conf.int_type.unwrap_or(IntType::I32)),
        }
    }

    pub(crate) fn generate_rust_type(&self, gen: &Generator) -> TokenStream {
        match self {
            TypeSpec::Int(_, itype) => {
                let typ = itype.type_name();
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
                quote! { #default }
            }
            TypeSpec::Bytes { .. } => {
                let bytes = Literal::byte_string(&unescape_c_escape_string(default));
                quote! { #bytes }
            }
            TypeSpec::Message(_) => {
                unreachable!("message fields shouldn't have custom defaults")
            }
            TypeSpec::Enum(tname) => {
                let enum_name =
                    Ident::new(&path_suffix(tname).to_case(Case::Pascal), Span::call_site());
                let variant = gen.enum_variant_name(default, &enum_name);
                quote! { #enum_name::#variant }
            }
            _ => {
                let default: TokenStream =
                    syn::parse_str(default).expect("default value tokenization error");
                quote! { #default as _ }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tspec_rust_type() {
        let gen = Generator::default();
        assert_eq!(TypeSpec::Bool.generate_rust_type(&gen).to_string(), "bool");
        assert_eq!(TypeSpec::Float.generate_rust_type(&gen).to_string(), "f32");
        assert_eq!(TypeSpec::Double.generate_rust_type(&gen).to_string(), "f64");
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntType::I32)
                .generate_rust_type(&gen)
                .to_string(),
            "i32"
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Fixed32, IntType::I32)
                .generate_rust_type(&gen)
                .to_string(),
            "i32"
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Fixed32, IntType::Usize)
                .generate_rust_type(&gen)
                .to_string(),
            "usize"
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
            TypeSpec::Int(PbInt::Int32, IntType::I8)
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
            quote! { "abc\n\tddd" }.to_string()
        );
        assert_eq!(
            TypeSpec::Bytes {
                type_path: syn::parse_str("Vec").unwrap(),
                max_bytes: None
            }
            .generate_default("abc\\n\\t\\a\\xA0ddd", &gen)
            .to_string(),
            quote! { b"abc\n\t\x07\xA0ddd" }.to_string()
        );
    }
}
