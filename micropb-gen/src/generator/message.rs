use std::collections::HashMap;

use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{DescriptorProto, Syntax};
use quote::{format_ident, quote};
use syn::Ident;

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
    pub(crate) attrs: TokenStream,
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
                    Field::from_proto(f, field_conf, syntax, Some(map_msg))
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
                    Field::from_proto(f, field_conf, syntax, None)
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
    ) -> Option<(TokenStream, TokenStream)> {
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
            #attrs
            pub struct #hazzer_name(micropb::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        Some((decl, conf.config.field_attr_parsed()))
    }

    pub(crate) fn generate_decl(
        &self,
        gen: &Generator,
        hazzer_field_attr: Option<TokenStream>,
    ) -> TokenStream {
        let msg_mod_name = gen.resolve_path_elem(self.name);
        let rust_name = &self.rust_name;
        let msg_fields = self.fields.iter().map(|f| f.generate_field(gen));
        let hazzer_field =
            hazzer_field_attr.map(|attr| quote! { #attr pub _has: #msg_mod_name::_Hazzer, });
        let oneof_fields = self
            .oneofs
            .iter()
            .map(|oneof| oneof.generate_field(gen, &msg_mod_name));

        let derive_msg = derive_msg_attr(self.derive_dbg, true);
        let attrs = &self.attrs;

        quote! {
            #derive_msg
            #attrs
            pub struct #rust_name {
                #(pub #msg_fields)*
                #(pub #oneof_fields)*
                #hazzer_field
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        config::{Config, OptionalRepr},
        generator::{
            field::{make_test_field, FieldType},
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
            attrs: quote! {},
        };

        let (decl, field_attrs) = msg.generate_hazzer_decl(config).unwrap();
        let expected = quote! {
            #[derive(Default, Clone, PartialEq)]
            #[derive(Eq)]
            pub struct _Hazzer(micropb::bitvec::BitArr!(for 2, in u8));

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
        assert_eq!(field_attrs.to_string(), quote! { #[default] }.to_string());
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
            attrs: quote! {},
        };
        assert!(msg.generate_hazzer_decl(config).is_none());
    }
}
