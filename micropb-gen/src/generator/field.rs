use proc_macro2::{Literal, TokenStream};
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, FieldDescriptorProto, Syntax,
};
use quote::quote;
use syn::Ident;

use crate::config::OptionalRepr;

use super::{type_spec::TypeSpec, CurrentConfig, Generator};

pub(crate) enum CustomField {
    Type(syn::Type),
    Delegate(Ident),
}

pub(crate) enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec,
        val: TypeSpec,
        packed: bool,
        type_path: syn::Path,
        max_len: Option<u32>,
    },
    // Implicit presence
    Single(TypeSpec),
    // Explicit presence
    Optional(TypeSpec, OptionalRepr),
    Repeated {
        typ: TypeSpec,
        packed: bool,
        type_path: syn::Path,
        max_len: Option<u32>,
    },
    Custom(CustomField),
}

pub(crate) struct Field<'a> {
    pub(crate) num: u32,
    pub(crate) ftype: FieldType,
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) default: Option<&'a str>,
    pub(crate) boxed: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> Field<'a> {
    pub(crate) fn explicit_presence(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(..))
    }

    pub(crate) fn is_option(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Option))
    }

    pub(crate) fn is_hazzer(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Hazzer))
    }

    pub(crate) fn delegate(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Delegate(d)) = &self.ftype {
            Some(d)
        } else {
            None
        }
    }

    pub(crate) fn custom_type_field(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Type(_)) = &self.ftype {
            Some(&self.rust_name)
        } else {
            None
        }
    }

    pub(crate) fn from_proto(
        proto: &'a FieldDescriptorProto,
        field_conf: CurrentConfig,
        syntax: Syntax,
        map_msg: Option<&DescriptorProto>,
    ) -> Option<Self> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }
        assert!(proto.oneof_index.is_none());

        let num = proto.number.unwrap() as u32;
        let name = proto.name();
        let rust_name = field_conf.config.rust_field_name(name);
        let boxed = field_conf.config.boxed.unwrap_or(false);

        let ftype = match (
            field_conf.config.custom_field_parsed(),
            map_msg,
            proto.label(),
        ) {
            (Some(t), _, _) => FieldType::Custom(t),

            (None, Some(map_msg), _) => {
                let key = TypeSpec::from_proto(&map_msg.field[0], &field_conf.next_conf("key"));
                let val = TypeSpec::from_proto(&map_msg.field[1], &field_conf.next_conf("value"));
                let type_name = field_conf.config.vec_type_parsed().unwrap();
                FieldType::Map {
                    key,
                    val,
                    type_path: type_name,
                    max_len: field_conf.config.max_len,
                    packed: proto
                        .options
                        .as_ref()
                        .and_then(|opt| opt.packed)
                        .unwrap_or(false),
                }
            }

            (None, None, Label::Repeated) => FieldType::Repeated {
                typ: TypeSpec::from_proto(proto, &field_conf.next_conf("elem")),
                type_path: field_conf.config.vec_type_parsed().unwrap(),
                max_len: field_conf.config.max_len,
                packed: proto
                    .options
                    .as_ref()
                    .and_then(|opt| opt.packed)
                    .unwrap_or(false),
            },

            (None, None, Label::Required | Label::Optional)
                if syntax == Syntax::Proto2
                    || proto.proto3_optional()
                    || proto.r#type() == Type::Message =>
            {
                let repr = field_conf.config.optional_repr.unwrap_or(if boxed {
                    OptionalRepr::Option
                } else {
                    OptionalRepr::Hazzer
                });
                FieldType::Optional(TypeSpec::from_proto(proto, &field_conf), repr)
            }

            (None, None, _) => FieldType::Single(TypeSpec::from_proto(proto, &field_conf)),
        };
        let attrs = field_conf.config.field_attr_parsed();

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            default: proto.default_value.as_deref(),
            boxed,
            attrs,
        })
    }

    pub(crate) fn generate_rust_type(&self, gen: &Generator) -> TokenStream {
        let typ = match &self.ftype {
            FieldType::Map {
                key,
                val,
                type_path: type_name,
                max_len,
                ..
            } => {
                let k = key.generate_rust_type(gen);
                let v = val.generate_rust_type(gen);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_name <#k, #v #(, #max_len)* > }
            }

            FieldType::Single(t) | FieldType::Optional(t, _) => t.generate_rust_type(gen),

            FieldType::Repeated {
                typ,
                type_path,
                max_len,
                ..
            } => {
                let t = typ.generate_rust_type(gen);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path <#t #(, #max_len)* > }
            }

            FieldType::Custom(CustomField::Type(t)) => return quote! {#t},
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have a type")
            }
        };
        gen.wrapped_type(typ, self.boxed, self.is_option())
    }

    pub(crate) fn generate_field(&self, gen: &Generator) -> TokenStream {
        if let FieldType::Custom(CustomField::Delegate(_)) = self.ftype {
            return quote! {};
        }
        let typ = self.generate_rust_type(gen);
        let name = &self.rust_name;
        let attrs = &self.attrs;
        quote! { #(#attrs)* pub #name : #typ, }
    }

    pub(crate) fn generate_default(&self, gen: &Generator) -> TokenStream {
        match self.ftype {
            FieldType::Single(ref t) | FieldType::Optional(ref t, OptionalRepr::Hazzer) => {
                if let Some(default) = self.default {
                    let value = t.generate_default(default, gen);
                    return gen.wrapped_value(value, self.boxed, false);
                }
            }
            // Options don't use custom defaults, they should just default to None
            FieldType::Optional(_, OptionalRepr::Option) => {
                return quote! { ::core::option::Option::None }
            }
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have default")
            }
            _ => {}
        }
        quote! { ::core::default::Default::default() }
    }
}

#[cfg(test)]
pub(crate) fn make_test_field(num: u32, name: &str, boxed: bool, ftype: FieldType) -> Field {
    Field {
        num,
        ftype,
        name,
        rust_name: Ident::new(name, proc_macro2::Span::call_site()),
        default: None,
        boxed,
        attrs: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::IntType, generator::type_spec::PbInt};

    use super::*;

    #[test]
    fn field_rust_type() {
        let gen = Generator::default();
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
            )
            .generate_rust_type(&gen)
            .to_string(),
            "bool"
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Repeated {
                    typ: TypeSpec::Message(".Message".to_owned()),
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_len: None,
                    packed: true
                }
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { Vec<Message> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Repeated {
                    typ: TypeSpec::String {
                        type_path: syn::parse_str("String").unwrap(),
                        max_bytes: Some(4)
                    },
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_len: Some(10),
                    packed: true
                }
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { Vec<String<4>, 10> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Map {
                    key: TypeSpec::Float,
                    val: TypeSpec::Int(PbInt::Uint64, IntType::U32),
                    type_path: syn::parse_str("std::HashMap").unwrap(),
                    max_len: None,
                    packed: true
                }
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { std::HashMap<f32, u32> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Map {
                    key: TypeSpec::Int(PbInt::Uint64, IntType::U32),
                    val: TypeSpec::Float,
                    type_path: syn::parse_str("std::HashMap").unwrap(),
                    max_len: Some(8),
                    packed: true
                }
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { std::HashMap<u32, f32, 8> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Custom(CustomField::Type(
                    syn::parse_str("custom::Type<true>").unwrap()
                ))
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { custom::Type<true> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Optional(
                    TypeSpec::Message(".Config".to_owned()),
                    OptionalRepr::Option
                )
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { ::core::option::Option<::alloc::boxed::Box<Config> > }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Single(TypeSpec::Message(".Config".to_owned()))
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { ::alloc::boxed::Box<Config> }.to_string()
        );
    }

    #[test]
    fn field_default() {
        let gen = Generator::default();
        // no special default
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
            )
            .generate_default(&gen)
            .to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            false,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { false as _ }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::alloc::boxed::Box::new(false as _) }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::option::Option::None }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Custom(CustomField::Type(syn::parse_str("Map").unwrap())),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );

        let field = make_test_field(
            0,
            "field",
            true,
            FieldType::Repeated {
                typ: TypeSpec::Double,
                packed: false,
                type_path: syn::parse_str("Vec").unwrap(),
                max_len: None,
            },
        );
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );
    }
}
