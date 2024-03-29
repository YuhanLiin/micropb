use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use prost_types::{FieldDescriptorProto, OneofDescriptorProto};
use quote::quote;
use syn::Ident;

use super::{derive_msg_attr, field::CustomField, type_spec::TypeSpec, CurrentConfig, Generator};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct OneofField<'a> {
    pub(crate) num: u32,
    pub(crate) tspec: TypeSpec,
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) boxed: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> OneofField<'a> {
    pub(crate) fn from_proto(
        proto: &'a FieldDescriptorProto,
        field_conf: &CurrentConfig,
    ) -> Option<Self> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name();
        // Oneof fields have camelcased variant names
        let rust_name = Ident::new(
            &field_conf
                .config
                .rust_field_name(&name)
                .to_string()
                .to_case(Case::Pascal),
            Span::call_site(),
        );
        let num = proto.number.unwrap() as u32;
        let tspec = TypeSpec::from_proto(proto, field_conf);
        let attrs = field_conf.config.field_attr_parsed();

        Some(OneofField {
            num,
            tspec,
            name,
            rust_name,
            boxed: field_conf.config.boxed.unwrap_or(false),
            attrs,
        })
    }

    fn generate_field(&self, gen: &Generator) -> TokenStream {
        let typ = gen.wrapped_type(self.tspec.generate_rust_type(gen), self.boxed, false);
        let name = &self.rust_name;
        let attrs = &self.attrs;
        quote! { #(#attrs)* #name(#typ), }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum OneofType<'a> {
    Enum {
        type_name: Ident,
        fields: Vec<OneofField<'a>>,
    },
    Custom(CustomField),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Oneof<'a> {
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) otype: OneofType<'a>,
    pub(crate) boxed: bool,
    pub(crate) field_attrs: Vec<syn::Attribute>,
    pub(crate) type_attrs: Vec<syn::Attribute>,
    pub(crate) derive_dbg: bool,
    pub(crate) idx: usize,
}

impl<'a> Oneof<'a> {
    pub(crate) fn delegate(&self) -> Option<&Ident> {
        if let OneofType::Custom(CustomField::Delegate(d)) = &self.otype {
            Some(d)
        } else {
            None
        }
    }

    pub(crate) fn custom_type_field(&self) -> Option<&Ident> {
        if let OneofType::Custom(CustomField::Type(_)) = &self.otype {
            Some(&self.rust_name)
        } else {
            None
        }
    }

    pub(crate) fn from_proto(
        proto: &'a OneofDescriptorProto,
        oneof_conf: CurrentConfig,
        idx: usize,
    ) -> Option<Self> {
        if oneof_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name();
        let rust_name = oneof_conf.config.rust_field_name(name);
        let otype = match oneof_conf.config.custom_field_parsed() {
            Some(custom) => OneofType::Custom(custom),
            None => OneofType::Enum {
                type_name: Ident::new(
                    &rust_name.to_string().to_case(Case::Pascal),
                    Span::call_site(),
                ),
                fields: vec![],
            },
        };
        let field_attrs = oneof_conf.config.field_attr_parsed();
        let type_attrs = oneof_conf.config.type_attr_parsed();

        Some(Oneof {
            name,
            rust_name,
            idx,
            otype,
            derive_dbg: oneof_conf.derive_dbg(),
            boxed: oneof_conf.config.boxed.unwrap_or(false),
            field_attrs,
            type_attrs,
        })
    }

    pub(crate) fn generate_decl(&self, gen: &Generator) -> TokenStream {
        if let OneofType::Enum { type_name, fields } = &self.otype {
            assert!(!fields.is_empty(), "empty enums should have been filtered");
            let fields = fields.iter().map(|f| f.generate_field(gen));
            let derive_msg = derive_msg_attr(self.derive_dbg, false);
            let attrs = &self.type_attrs;

            quote! {
                #derive_msg
                #(#attrs)*
                pub enum #type_name {
                    #(#fields)*
                }
            }
        } else {
            quote! {}
        }
    }

    pub(crate) fn generate_field(&self, gen: &Generator, msg_mod_name: &Ident) -> TokenStream {
        let name = &self.rust_name;
        let oneof_type = match &self.otype {
            OneofType::Enum { type_name, .. } => {
                gen.wrapped_type(quote! { #msg_mod_name::#type_name }, self.boxed, true)
            }
            OneofType::Custom(CustomField::Type(type_path)) => quote! { #type_path },
            OneofType::Custom(CustomField::Delegate(_)) => return quote! {},
        };
        let attrs = &self.field_attrs;
        quote! { #(#attrs)* pub #name: #oneof_type, }
    }
}

#[cfg(test)]
pub(crate) fn make_test_oneof_field(
    num: u32,
    name: &str,
    boxed: bool,
    tspec: TypeSpec,
) -> OneofField {
    OneofField {
        num,
        name,
        tspec,
        rust_name: Ident::new(&name.to_case(Case::Pascal), Span::call_site()),
        boxed,
        attrs: vec![],
    }
}

#[cfg(test)]
pub(crate) fn make_test_oneof<'a>(name: &'a str, boxed: bool, otype: OneofType<'a>) -> Oneof<'a> {
    Oneof {
        name,
        rust_name: Ident::new(name, Span::call_site()),
        otype,
        boxed,
        field_attrs: vec![],
        type_attrs: vec![],
        derive_dbg: true,
        idx: 0,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use prost_types::field_descriptor_proto::Type;

    use crate::config::{parse_attributes, Config};

    use super::*;

    fn field_proto(num: u32, name: &str) -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: Some(name.to_owned()),
            number: Some(num as i32),
            r#type: Some(Type::Bool.into()),
            ..Default::default()
        }
    }

    #[test]
    fn from_proto_skipped() {
        let config = Box::new(Config::new().skip(true));
        let oneof_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field");
        assert!(OneofField::from_proto(&field, &oneof_conf).is_none());
        let oneof = OneofDescriptorProto::default();
        assert!(Oneof::from_proto(&oneof, oneof_conf, 0).is_none());
    }

    #[test]
    fn from_proto_field() {
        let mut config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field");
        assert_eq!(
            OneofField::from_proto(&field, &field_conf).unwrap(),
            OneofField {
                num: 1,
                tspec: TypeSpec::Bool,
                name: "field",
                rust_name: Ident::new("Field", Span::call_site()),
                boxed: false,
                attrs: vec![]
            }
        );

        config.boxed = Some(true);
        config.field_attributes = Some("#[attr]".to_owned());
        config.rename_field = Some("renamed".to_owned());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            OneofField::from_proto(&field, &field_conf).unwrap(),
            OneofField {
                num: 1,
                tspec: TypeSpec::Bool,
                name: "field",
                rust_name: Ident::new("Renamed", Span::call_site()),
                boxed: true,
                attrs: parse_attributes("#[attr]").unwrap()
            }
        );
    }

    #[test]
    fn from_proto() {
        let mut config = Box::new(Config::new());
        let oneof_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let oneof = OneofDescriptorProto {
            name: Some("oneof".to_owned()),
            options: None,
        };
        assert_eq!(
            Oneof::from_proto(&oneof, oneof_conf, 0).unwrap(),
            Oneof {
                name: "oneof",
                rust_name: Ident::new("oneof", Span::call_site()),
                otype: OneofType::Enum {
                    type_name: Ident::new("Oneof", Span::call_site()),
                    fields: vec![]
                },
                boxed: false,
                field_attrs: vec![],
                type_attrs: vec![],
                derive_dbg: true,
                idx: 0
            }
        );

        config.boxed = Some(true);
        config.field_attributes = Some("#[attr]".to_owned());
        config.type_attributes = Some("#[derive(Eq)]".to_owned());
        config.no_debug_derive = Some(true);
        config.rename_field = Some("renamed_oneof".to_owned());
        let oneof_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Oneof::from_proto(&oneof, oneof_conf, 0).unwrap(),
            Oneof {
                name: "oneof",
                rust_name: Ident::new("renamed_oneof", Span::call_site()),
                otype: OneofType::Enum {
                    type_name: Ident::new("RenamedOneof", Span::call_site()),
                    fields: vec![]
                },
                boxed: true,
                field_attrs: parse_attributes("#[attr]").unwrap(),
                type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
                derive_dbg: false,
                idx: 0
            }
        );
    }

    #[test]
    fn oneof_enum() {
        let gen = Generator::default();
        let mut oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Enum {
                type_name: Ident::new("Oneof", Span::call_site()),
                fields: vec![
                    make_test_oneof_field(0, "a", true, TypeSpec::Float),
                    make_test_oneof_field(1, "b", false, TypeSpec::Bool),
                ],
            },
            boxed: false,
            field_attrs: parse_attributes("#[default]").unwrap(),
            type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
            derive_dbg: true,
            idx: 0,
        };

        let out = oneof.generate_decl(&gen);
        let expected = quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[derive(Eq)]
            pub enum Oneof {
                A(::alloc::boxed::Box<f32>),
                B(bool),
            }
        };
        assert_eq!(out.to_string(), expected.to_string());

        assert_eq!(
            oneof
                .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
                .to_string(),
            quote! { #[default] pub oneof: ::core::option::Option<Msg::Oneof>, }.to_string()
        );

        oneof.boxed = true;
        assert_eq!(
            oneof
                .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
                .to_string(),
            quote! { #[default] pub oneof: ::core::option::Option<::alloc::boxed::Box<Msg::Oneof> >, }.to_string()
        );
    }

    #[test]
    fn oneof_custom() {
        let gen = Generator::default();
        let oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Custom(CustomField::Type(syn::parse_str("Custom<f32>").unwrap())),
            boxed: true,
            field_attrs: vec![],
            type_attrs: vec![],
            derive_dbg: true,
            idx: 0,
        };
        assert!(oneof.generate_decl(&gen).is_empty());
        assert_eq!(
            oneof
                .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
                .to_string(),
            quote! { pub oneof: Custom<f32>, }.to_string()
        );

        let oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Custom(CustomField::Delegate(syn::parse_str("delegate").unwrap())),
            boxed: false,
            field_attrs: vec![],
            type_attrs: vec![],
            derive_dbg: true,
            idx: 0,
        };
        assert!(oneof.generate_decl(&gen).is_empty());
        assert!(oneof
            .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
            .to_string()
            .is_empty());
    }
}
