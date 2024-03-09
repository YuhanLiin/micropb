use std::collections::HashMap;

use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{DescriptorProto, Syntax};
use quote::{format_ident, quote};
use syn::Ident;

use crate::generator::field::{CustomField, FieldType};

use super::{
    derive_msg_attr,
    field::Field,
    oneof::{Oneof, OneofField, OneofType},
    CurrentConfig, Generator,
};

pub(crate) struct Message<'a> {
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) oneofs: Vec<Oneof<'a>>,
    pub(crate) fields: Vec<Field<'a>>,
    pub(crate) derive_dbg: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> Message<'a> {
    pub(crate) fn from_proto(
        proto: &'a DescriptorProto,
        syntax: Syntax,
        msg_conf: &CurrentConfig,
    ) -> Option<Self> {
        if msg_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name.as_ref().unwrap();

        let mut oneofs: Vec<_> = proto
            .oneof_decl
            .iter()
            .enumerate()
            .filter_map(|(idx, oneof)| {
                Oneof::from_proto(oneof, msg_conf.next_conf(oneof.name()), idx)
            })
            .collect();

        let mut map_types = HashMap::new();
        for m in &proto.nested_type {
            if m.options.as_ref().map(|o| o.map_entry()).unwrap_or(false) {
                map_types.insert(m.name(), m);
            }
        }

        let fields: Vec<_> = proto
            .field
            .iter()
            .filter_map(|f| {
                let field_conf = msg_conf.next_conf(f.name());
                let raw_msg_name = f
                    .type_name()
                    .rsplit_once('.')
                    .map(|(_, r)| r)
                    .unwrap_or(f.type_name());
                if let Some(map_msg) = map_types.remove(raw_msg_name) {
                    Field::from_proto(f, &field_conf, syntax, Some(map_msg))
                } else {
                    if let Some(idx) = f.oneof_index {
                        if let Some(OneofType::Enum { fields, .. }) = oneofs
                            .iter_mut()
                            .find(|o| o.idx == idx as usize)
                            .map(|o| &mut o.otype)
                        {
                            if let Some(field) = OneofField::from_proto(f, field_conf) {
                                fields.push(field);
                            }
                        }
                        return None;
                    }
                    Field::from_proto(f, &field_conf, syntax, None)
                }
            })
            .collect();

        // Remove all oneofs that are empty enums
        let oneofs: Vec<_> = oneofs
            .into_iter()
            .filter(|o| !matches!(&o.otype, OneofType::Enum { fields, .. } if fields.is_empty()))
            .collect();

        Some(Self {
            name,
            rust_name: Ident::new(name, Span::call_site()),
            oneofs,
            fields,
            derive_dbg: !msg_conf.config.no_debug_derive.unwrap_or(false),
            attrs: msg_conf.config.type_attr_parsed(),
        })
    }

    pub(crate) fn check_delegates(&self) {
        let odelegates = self.oneofs.iter().filter_map(|o| o.delegate());
        let fdelegates = self.fields.iter().filter_map(|f| f.delegate());
        for delegate in odelegates.chain(fdelegates) {
            let ocustoms = self.oneofs.iter().filter_map(|o| o.custom_type_field());
            let fcustoms = self.fields.iter().filter_map(|f| f.custom_type_field());
            if !ocustoms.chain(fcustoms).any(|custom| delegate == custom) {
                // TODO error about how delegate != custom
            }
        }
    }

    pub(crate) fn generate_hazzer_decl(
        &self,
        conf: CurrentConfig,
    ) -> Option<(TokenStream, Vec<syn::Attribute>)> {
        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let attrs = &conf.config.type_attr_parsed();
        let derive_msg = derive_msg_attr(!conf.config.no_debug_derive.unwrap_or(false), true);

        let hazzers = self.fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        if count == 0 {
            return None;
        }

        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = &f.rust_name;
            let setter = format_ident!("set_{}", f.rust_name);
            let i = Literal::usize_unsuffixed(i);

            quote! {
                #[inline]
                pub fn #fname(&self) -> bool {
                    self.0[#i]
                }

                #[inline]
                pub fn #setter(&mut self, val: bool) {
                    self.0.set(#i, val);
                }
            }
        });

        let count = Literal::usize_unsuffixed(count);
        let decl = quote! {
            #derive_msg
            #(#attrs)*
            pub struct #hazzer_name(::micropb::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        Some((decl, conf.config.field_attr_parsed()))
    }

    pub(crate) fn generate_decl(
        &self,
        gen: &Generator,
        hazzer_field_attr: Option<Vec<syn::Attribute>>,
    ) -> TokenStream {
        let msg_mod_name = gen.resolve_path_elem(self.name);
        let rust_name = &self.rust_name;
        let msg_fields = self.fields.iter().map(|f| f.generate_field(gen));
        let hazzer_field =
            hazzer_field_attr.map(|attr| quote! { #(#attr)* pub _has: #msg_mod_name::_Hazzer, });
        let oneof_fields = self
            .oneofs
            .iter()
            .map(|oneof| oneof.generate_field(gen, &msg_mod_name));

        let derive_msg = derive_msg_attr(self.derive_dbg, false);
        let attrs = &self.attrs;

        quote! {
            #derive_msg
            #(#attrs)*
            pub struct #rust_name {
                #(#msg_fields)*
                #(#oneof_fields)*
                #hazzer_field
            }
        }
    }

    pub(crate) fn generate_default_impl(&self, gen: &Generator, use_hazzer: bool) -> TokenStream {
        // Skip delegate fields when generating defaults
        let field_defaults = self.fields.iter().map(|f| {
            if let FieldType::Custom(CustomField::Delegate(_)) = f.ftype {
                None
            } else {
                let name = &f.rust_name;
                let default = f.generate_default(gen);
                Some(quote! { #name: #default, })
            }
        });
        let oneof_names = self.oneofs.iter().filter_map(|o| {
            if let OneofType::Custom(CustomField::Delegate(_)) = o.otype {
                None
            } else {
                Some(&o.rust_name)
            }
        });
        let hazzer_default =
            use_hazzer.then(|| quote! { _has: ::core::default::Default::default(), });
        let rust_name = &self.rust_name;

        quote! {
            impl ::core::default::Default for #rust_name {
                fn default() -> Self {
                    Self {
                        #(#field_defaults)*
                        #(#oneof_names: ::core::option::Option::None,)*
                        #hazzer_default
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        config::{parse_attributes, Config, OptionalRepr},
        generator::{
            field::{make_test_field, CustomField, FieldType},
            oneof::{make_test_oneof, make_test_oneof_field},
            type_spec::TypeSpec,
        },
    };

    use super::*;

    #[test]
    fn hazzer() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(
                Config::new()
                    .type_attributes("#[derive(Eq)]")
                    .field_attributes("#[default]")
                    .no_debug_derive(true),
            )),
        };
        let msg = Message {
            name: "msg",
            rust_name: Ident::new("msg", Span::call_site()),
            oneofs: vec![],
            fields: vec![
                make_test_field(
                    1,
                    "field1",
                    false,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
                ),
                make_test_field(2, "field2", false, FieldType::Single(TypeSpec::Bool)),
                make_test_field(
                    3,
                    "field3",
                    false,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
                ),
            ],
            derive_dbg: true,
            attrs: vec![],
        };

        let (decl, field_attrs) = msg.generate_hazzer_decl(config).unwrap();
        let expected = quote! {
            #[derive(Default, Clone, PartialEq)]
            #[derive(Eq)]
            pub struct _Hazzer(::micropb::bitvec::BitArr!(for 2, in u8));

            impl _Hazzer {
                #[inline]
                pub fn field1(&self) -> bool {
                    self.0[0]
                }

                #[inline]
                pub fn set_field1(&mut self, val: bool) {
                    self.0.set(0, val);
                }

                #[inline]
                pub fn field3(&self) -> bool {
                    self.0[1]
                }

                #[inline]
                pub fn set_field3(&mut self, val: bool) {
                    self.0.set(1, val);
                }
            }
        };
        assert_eq!(decl.to_string(), expected.to_string());
        assert_eq!(
            quote! { #(#field_attrs)* }.to_string(),
            quote! { #[default] }.to_string()
        );
    }

    #[test]
    fn hazzer_empty() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(Config::new())),
        };
        let msg = Message {
            name: "msg",
            rust_name: Ident::new("msg", Span::call_site()),
            oneofs: vec![],
            fields: vec![
                make_test_field(2, "field2", false, FieldType::Single(TypeSpec::Bool)),
                make_test_field(
                    4,
                    "field4",
                    true,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
                ),
            ],
            derive_dbg: true,
            attrs: vec![],
        };
        assert!(msg.generate_hazzer_decl(config).is_none());
    }

    #[test]
    fn default_impl() {
        let mut fields = vec![
            make_test_field(1, "a", false, FieldType::Single(TypeSpec::Bool)),
            make_test_field(
                2,
                "b",
                true,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
            ),
            make_test_field(
                3,
                "c",
                true,
                FieldType::Optional(TypeSpec::Double, OptionalRepr::Hazzer),
            ),
            make_test_field(
                4,
                "d",
                true,
                FieldType::Custom(CustomField::Type(syn::parse_str("Custom").unwrap())),
            ),
            make_test_field(
                5,
                "e",
                true,
                FieldType::Custom(CustomField::Delegate(syn::parse_str("d").unwrap())),
            ),
        ];
        fields[0].default = Some("true");
        fields[1].default = Some("true");
        fields[2].default = Some("-3.45");

        let msg = Message {
            name: "Msg",
            rust_name: Ident::new("Msg", Span::call_site()),
            oneofs: vec![
                make_test_oneof(
                    "oneof",
                    false,
                    OneofType::Enum {
                        type_name: Ident::new("Oneof", Span::call_site()),
                        fields: vec![make_test_oneof_field(7, "x", true, TypeSpec::Float)],
                    },
                ),
                make_test_oneof(
                    "oneof_custom",
                    false,
                    OneofType::Custom(CustomField::Type(syn::parse_str("Custom").unwrap())),
                ),
                make_test_oneof(
                    "oneof_delegate",
                    false,
                    OneofType::Custom(CustomField::Delegate(syn::parse_str("d").unwrap())),
                ),
            ],
            fields,
            derive_dbg: true,
            attrs: vec![],
        };

        let gen = Generator::default();
        let out = msg.generate_default_impl(&gen, true);
        let expected = quote! {
            impl ::core::default::Default for Msg {
                fn default() -> Self {
                    Self {
                        a: true as _,
                        b: ::core::option::Option::None,
                        c: ::alloc::boxed::Box::new(-3.45 as _),
                        d: ::core::default::Default::default(),
                        oneof: ::core::option::Option::None,
                        oneof_custom: ::core::option::Option::None,
                        _has: ::core::default::Default::default(),
                    }
                }
            }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }

    #[test]
    fn msg_decl() {
        let mut fields = vec![
            make_test_field(1, "single", false, FieldType::Single(TypeSpec::Bool)),
            make_test_field(
                2,
                "opt",
                false,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
            ),
            make_test_field(
                3,
                "optbox",
                true,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
            ),
            make_test_field(
                4,
                "boxed",
                true,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
            ),
            make_test_field(
                5,
                "string",
                true,
                FieldType::Optional(
                    TypeSpec::String {
                        type_path: syn::parse_str("String").unwrap(),
                        max_bytes: Some(2),
                    },
                    OptionalRepr::Option,
                ),
            ),
            make_test_field(
                6,
                "custom",
                true,
                FieldType::Custom(CustomField::Type(syn::parse_str("Custom").unwrap())),
            ),
            make_test_field(
                7,
                "delegate",
                true,
                FieldType::Custom(CustomField::Delegate(syn::parse_str("custom").unwrap())),
            ),
        ];
        fields[0].attrs = parse_attributes("#[default]").unwrap();
        fields[1].attrs = parse_attributes("#[attr]").unwrap();

        let mut oneofs = vec![
            make_test_oneof(
                "oneof",
                false,
                OneofType::Enum {
                    type_name: Ident::new("Oneof", Span::call_site()),
                    fields: vec![],
                },
            ),
            make_test_oneof(
                "oneof_custom",
                false,
                OneofType::Custom(CustomField::Type(syn::parse_str("Custom").unwrap())),
            ),
            make_test_oneof(
                "oneof_delegate",
                false,
                OneofType::Custom(CustomField::Delegate(syn::parse_str("custom").unwrap())),
            ),
        ];
        oneofs[1].field_attrs = parse_attributes("#[attr]").unwrap();

        let msg = Message {
            name: "Msg",
            rust_name: Ident::new("Msg", Span::call_site()),
            oneofs,
            fields,
            derive_dbg: true,
            attrs: parse_attributes("#[derive(Eq)]").unwrap(),
        };
        let gen = Generator::default();
        let out = msg.generate_decl(&gen, None);

        let expected = quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[derive(Eq)]
            pub struct Msg {
                #[default]
                pub single: bool,
                #[attr]
                pub opt: ::core::option::Option<bool>,
                pub optbox: ::core::option::Option< ::alloc::boxed::Box<bool> >,
                pub boxed: ::alloc::boxed::Box<bool>,
                pub string: ::core::option::Option< ::alloc::boxed::Box< String<2> > >,
                pub custom: Custom,
                pub oneof: ::core::option::Option<mod_Msg::Oneof>,
                #[attr]
                pub oneof_custom: Custom,
            }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }
}
