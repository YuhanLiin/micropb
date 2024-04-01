use std::collections::HashMap;

use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{DescriptorProto, Syntax};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    config::OptionalRepr,
    generator::field::{CustomField, FieldType},
};

use super::{
    derive_msg_attr,
    field::Field,
    oneof::{Oneof, OneofField, OneofType},
    CurrentConfig, Generator,
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Message<'a> {
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) oneofs: Vec<Oneof<'a>>,
    pub(crate) fields: Vec<Field<'a>>,
    pub(crate) derive_dbg: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) unknown_handler: Option<syn::Type>,
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

        let mut synthetic_oneof_idx = vec![];

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
                    // Map field
                    Field::from_proto(f, &field_conf, syntax, Some(map_msg))
                } else {
                    if let Some(idx) = f.oneof_index {
                        if f.proto3_optional() {
                            synthetic_oneof_idx.push(idx as usize);
                        } else {
                            if let Some(OneofType::Enum { fields, .. }) = oneofs
                                .iter_mut()
                                .find(|o| o.idx == idx as usize)
                                .map(|o| &mut o.otype)
                            {
                                // Oneof field
                                if let Some(field) = OneofField::from_proto(f, &field_conf) {
                                    fields.push(field);
                                }
                            }
                            return None;
                        }
                    }
                    // Normal field
                    Field::from_proto(f, &field_conf, syntax, None)
                }
            })
            .collect();

        // Remove all oneofs that are empty enums or synthetic oneofs
        let oneofs: Vec<_> = oneofs
            .into_iter()
            .filter(|o| !matches!(&o.otype, OneofType::Enum { fields, .. } if fields.is_empty()))
            .filter(|o| !synthetic_oneof_idx.contains(&o.idx))
            .collect();

        Some(Self {
            name,
            rust_name: Ident::new(name, Span::call_site()),
            oneofs,
            fields,
            derive_dbg: !msg_conf.config.no_debug_derive.unwrap_or(false),
            attrs: msg_conf.config.type_attr_parsed(),
            unknown_handler: msg_conf.config.unknown_handler_parsed(),
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
        unknown_field_attr: Vec<syn::Attribute>,
    ) -> TokenStream {
        let msg_mod_name = gen.resolve_path_elem(self.name);
        let rust_name = &self.rust_name;
        let msg_fields = self.fields.iter().map(|f| f.generate_field(gen));
        let hazzer_field_attr = hazzer_field_attr.iter();
        let oneof_fields = self
            .oneofs
            .iter()
            .map(|oneof| oneof.generate_field(gen, &msg_mod_name));

        let unknown_field_attr = self
            .unknown_handler
            .as_ref()
            .map(|_| unknown_field_attr)
            .into_iter();
        let unknown_field_type = self.unknown_handler.iter();

        let derive_msg = derive_msg_attr(self.derive_dbg, false);
        let attrs = &self.attrs;

        quote! {
            #derive_msg
            #(#attrs)*
            pub struct #rust_name {
                #(#msg_fields)*
                #(#oneof_fields)*
                #(#(#hazzer_field_attr)* pub _has: #msg_mod_name::_Hazzer,)*
                #(#(#unknown_field_attr)* pub _unknown: #unknown_field_type,)*
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
        let unknown_default = self
            .unknown_handler
            .as_ref()
            .map(|_| quote! { _unknown: ::core::default::Default::default(), });
        let rust_name = &self.rust_name;

        quote! {
            impl ::core::default::Default for #rust_name {
                fn default() -> Self {
                    Self {
                        #(#field_defaults)*
                        #(#oneof_names: ::core::option::Option::None,)*
                        #hazzer_default
                        #unknown_default
                    }
                }
            }
        }
    }

    pub(crate) fn generate_impl(&self, gen: &Generator) -> TokenStream {
        let accessors = self.fields.iter().map(|f| {
            if let FieldType::Optional(type_spec, opt) = &f.ftype {
                let type_name = type_spec.generate_rust_type(gen);
                let setter_name = format_ident!("set_{}", f.rust_name);
                let muter_name = format_ident!("mut_{}", f.rust_name);
                let clearer_name = format_ident!("clear_{}", f.rust_name);
                let fname = &f.rust_name;

                // use value.into() to handle conversion into boxed and non-boxed fields
                if let OptionalRepr::Hazzer = opt {
                    quote! {
                        pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                            self._has.#fname().then_some(&self.#fname)
                        }

                        pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                            self._has.#fname().then_some(&mut self.#fname)
                        }

                        pub fn #setter_name(&mut self, value: #type_name) {
                            self._has.#setter_name(true);
                            self.#fname = value.into();
                        }

                        pub fn #clearer_name(&mut self) {
                            self._has.#setter_name(false);
                        }
                    }
                } else {
                    let (deref, deref_mut) = if f.boxed {
                        (format_ident!("as_deref"), format_ident!("as_deref_mut"))
                    } else {
                        (format_ident!("as_ref"), format_ident!("as_mut"))
                    };
                    quote! {
                        pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                            self.#fname.#deref()
                        }

                        pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                            self.#fname.#deref_mut()
                        }

                        pub fn #setter_name(&mut self, value: #type_name) {
                            self.#fname = ::core::option::Option::Some(value.into());
                        }

                        pub fn #clearer_name(&mut self) {
                            self.#fname = ::core::option::Option::None;
                        }
                    }
                }
            } else {
                quote! {}
            }
        });

        let name = &self.rust_name;
        quote! {
            impl #name {
                pub fn new() -> Self {
                    ::core::default::Default::default()
                }

                #(#accessors)*
            }
        }
    }

    pub(crate) fn generate_decode(&self, gen: &Generator) -> TokenStream {
        let name = &self.rust_name;
        let tag = Ident::new("tag", Span::call_site());
        let decoder = Ident::new("decoder", Span::call_site());

        let branches = self
            .fields
            .iter()
            .map(|f| f.generate_decode_branch(gen, &tag, &decoder));

        let unknown_branch = if self.unknown_handler.is_some() {
            quote! { self._unknown.decode_field(#tag, #decoder)?; }
        } else {
            quote! { self.skip_wire_value(#tag.wire_type())?; }
        };

        quote! {
            impl ::micropb::MessageDecode for #name {
                fn decode<R: ::micropb::PbRead>(
                    &mut self,
                    #decoder: &mut ::micropb::PbDecoder<R>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<R::Error>>
                {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};

                    let before = self.bytes_read();
                    while self.bytes_read() - before < len {
                        let #tag = decoder.decode_tag();
                        match #tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            #(#branches)*
                            _ => { #unknown_branch }
                        }
                    }

                    let actual_len = self.bytes_read() - before;
                    if actual_len != len {
                        Err(DecodeError::WrongLen {
                            expected: len,
                            actual: actual_len,
                        })
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use prost_types::{
        field_descriptor_proto::{Label, Type},
        FieldDescriptorProto, FieldOptions, MessageOptions, OneofDescriptorProto,
    };

    use crate::{
        config::{parse_attributes, Config, IntType, OptionalRepr},
        generator::{
            field::{make_test_field, CustomField, FieldType},
            oneof::{make_test_oneof, make_test_oneof_field},
            type_spec::{PbInt, TypeSpec},
        },
        pathtree::Node,
    };

    use super::*;

    fn test_msg_proto() -> DescriptorProto {
        let map_msg = DescriptorProto {
            name: Some("MapElem".to_owned()),
            field: vec![
                FieldDescriptorProto {
                    number: Some(1),
                    name: Some("key".to_owned()),
                    r#type: Some(Type::Int64.into()),
                    ..Default::default()
                },
                FieldDescriptorProto {
                    number: Some(2),
                    name: Some("value".to_owned()),
                    r#type: Some(Type::Uint64.into()),
                    ..Default::default()
                },
            ],
            options: Some(MessageOptions {
                map_entry: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };

        DescriptorProto {
            name: Some("Message".to_owned()),
            field: vec![
                FieldDescriptorProto {
                    number: Some(1),
                    name: Some("bool_field".to_owned()),
                    r#type: Some(Type::Bool.into()),
                    ..Default::default()
                },
                FieldDescriptorProto {
                    number: Some(2),
                    name: Some("oneof_field".to_owned()),
                    r#type: Some(Type::Sint32.into()),
                    oneof_index: Some(0),
                    ..Default::default()
                },
                FieldDescriptorProto {
                    number: Some(3),
                    name: Some("map_field".to_owned()),
                    r#type: Some(Type::Message.into()),
                    label: Some(Label::Repeated.into()),
                    type_name: Some(".Message.MapElem".to_owned()),
                    options: Some(FieldOptions {
                        packed: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                FieldDescriptorProto {
                    number: Some(4),
                    name: Some("oneof_field2".to_owned()),
                    r#type: Some(Type::Float.into()),
                    oneof_index: Some(0),
                    ..Default::default()
                },
            ],
            oneof_decl: vec![OneofDescriptorProto {
                name: Some("oneof".to_owned()),
                options: None,
            }],
            nested_type: vec![map_msg],
            ..Default::default()
        }
    }

    #[test]
    fn from_proto_skipped() {
        let proto = test_msg_proto();
        let config = Box::new(Config::new().skip(true));
        let msg_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert!(Message::from_proto(&proto, Syntax::Proto2, &msg_conf).is_none());
    }

    #[test]
    fn from_proto_skip_fields() {
        let proto = test_msg_proto();
        let empty_msg = Message {
            name: "Message",
            rust_name: Ident::new("Message", Span::call_site()),
            oneofs: vec![],
            fields: vec![],
            derive_dbg: true,
            attrs: vec![],
            unknown_handler: None,
        };
        let config = Box::new(Config::new());
        let mut node = Node::default();

        // Skip all fields and oneofs, but not oneof fields
        *node.add_path(std::iter::once("bool_field")).value_mut() =
            Some(Box::new(Config::new().skip(true)));
        *node.add_path(std::iter::once("map_field")).value_mut() =
            Some(Box::new(Config::new().skip(true)));
        *node.add_path(std::iter::once("oneof")).value_mut() =
            Some(Box::new(Config::new().skip(true)));
        let msg_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Message::from_proto(&proto, Syntax::Proto2, &msg_conf).unwrap(),
            empty_msg
        );

        // Don't skip oneof, but skip oneof fields (oneof should still be skipped)
        *node.add_path(std::iter::once("oneof")).value_mut() =
            Some(Box::new(Config::new().skip(false)));
        *node.add_path(std::iter::once("oneof_field")).value_mut() =
            Some(Box::new(Config::new().skip(true)));
        *node.add_path(std::iter::once("oneof_field2")).value_mut() =
            Some(Box::new(Config::new().skip(true)));
        let msg_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Message::from_proto(&proto, Syntax::Proto2, &msg_conf).unwrap(),
            empty_msg
        );
    }

    #[test]
    fn from_proto() {
        let proto = test_msg_proto();
        let config = Box::new(
            Config::new()
                .map_type("Map")
                .type_attributes("#[derive(Self)]")
                .no_debug_derive(true)
                .unknown_handler("UnknownType"),
        );
        let mut node = Node::default();
        *node.add_path(["bool_field"].into_iter()).value_mut() =
            Some(Box::new(Config::new().boxed(true)));
        *node.add_path(["oneof_field"].into_iter()).value_mut() =
            Some(Box::new(Config::new().int_type(IntType::U8)));
        *node.add_path(["oneof_field2"].into_iter()).value_mut() =
            Some(Box::new(Config::new().boxed(true)));
        *node.add_path(["oneof"].into_iter()).value_mut() =
            Some(Box::new(Config::new().type_attributes("#[derive(Eq)]")));
        *node.add_path(["map_field", "key"].into_iter()).value_mut() =
            Some(Box::new(Config::new().int_type(IntType::I16)));
        *node
            .add_path(["map_field", "value"].into_iter())
            .value_mut() = Some(Box::new(Config::new().int_type(IntType::U16)));
        let msg_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        assert_eq!(
            Message::from_proto(&proto, Syntax::Proto2, &msg_conf).unwrap(),
            Message {
                name: "Message",
                rust_name: Ident::new("Message", Span::call_site()),
                oneofs: vec![Oneof {
                    name: "oneof",
                    rust_name: Ident::new("oneof", Span::call_site()),
                    otype: OneofType::Enum {
                        type_name: Ident::new("Oneof", Span::call_site()),
                        fields: vec![
                            make_test_oneof_field(
                                2,
                                "oneof_field",
                                false,
                                TypeSpec::Int(PbInt::Sint32, IntType::U8)
                            ),
                            make_test_oneof_field(4, "oneof_field2", true, TypeSpec::Float),
                        ]
                    },
                    boxed: false,
                    field_attrs: vec![],
                    // Overrides the type attrs of the message
                    type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
                    // Inherits the no_debug_derive setting of the message
                    derive_dbg: false,
                    idx: 0
                }],
                fields: vec![
                    make_test_field(
                        1,
                        "bool_field",
                        true,
                        FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option)
                    ),
                    make_test_field(
                        3,
                        "map_field",
                        false,
                        FieldType::Map {
                            key: TypeSpec::Int(PbInt::Int64, IntType::I16),
                            val: TypeSpec::Int(PbInt::Uint64, IntType::U16),
                            packed: true,
                            type_path: syn::parse_str("Map").unwrap(),
                            max_len: None
                        }
                    ),
                ],
                derive_dbg: false,
                attrs: parse_attributes("#[derive(Self)]").unwrap(),
                unknown_handler: Some(syn::parse_str("UnknownType").unwrap())
            }
        )
    }

    #[test]
    fn synthetic_oneof() {
        let proto = DescriptorProto {
            name: Some("Msg".to_owned()),
            field: vec![FieldDescriptorProto {
                number: Some(1),
                name: Some("opt".to_owned()),
                r#type: Some(Type::Bool.into()),
                proto3_optional: Some(true),
                oneof_index: Some(0),
                ..Default::default()
            }],
            oneof_decl: vec![OneofDescriptorProto {
                name: Some("_opt".to_owned()),
                options: None,
            }],
            ..Default::default()
        };
        let msg_conf = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(Config::new())),
        };

        assert_eq!(
            Message::from_proto(&proto, Syntax::Proto3, &msg_conf).unwrap(),
            Message {
                name: "Msg",
                rust_name: Ident::new("Msg", Span::call_site()),
                oneofs: vec![],
                fields: vec![make_test_field(
                    1,
                    "opt",
                    false,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
                )],
                derive_dbg: true,
                attrs: vec![],
                unknown_handler: None
            }
        )
    }

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
            unknown_handler: None,
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
            unknown_handler: None,
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
            unknown_handler: Some(syn::parse_str("UnknownType").unwrap()),
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
                        _unknown: ::core::default::Default::default(),
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
            unknown_handler: Some(syn::parse_str("UnknownType").unwrap()),
        };
        let gen = Generator::default();
        let out = msg.generate_decl(
            &gen,
            Some(parse_attributes("#[attr1] #[attr2]").unwrap()),
            parse_attributes("#[attr3] #[attr4]").unwrap(),
        );

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
                #[attr1]
                #[attr2]
                pub _has: mod_Msg::_Hazzer,
                #[attr3]
                #[attr4]
                pub _unknown: UnknownType,
            }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }

    #[test]
    fn msg_impl() {
        let msg = Message {
            name: "Msg",
            rust_name: Ident::new("Msg", Span::call_site()),
            fields: vec![
                make_test_field(
                    1,
                    "haz",
                    false,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
                ),
                make_test_field(
                    1,
                    "opt",
                    false,
                    FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
                ),
            ],
            derive_dbg: true,
            oneofs: vec![],
            attrs: vec![],
            unknown_handler: None,
        };
        let gen = Generator::default();
        let out = msg.generate_impl(&gen);

        let expected = quote! {
            impl Msg {
                pub fn haz(&self) -> ::core::option::Option<&bool> {
                    self._has.haz().then_some(&self.haz)
                }

                pub fn mut_haz(&mut self) -> ::core::option::Option<&mut bool> {
                    self._has.haz().then_some(&mut self.haz)
                }

                pub fn set_haz(&mut self, value: bool) {
                    self._has.set_haz(true);
                    self.haz = value.into();
                }

                pub fn clear_haz(&mut self) {
                    self._has.set_haz(false);
                }

                pub fn opt(&self) -> ::core::option::Option<&bool> {
                    self.opt.as_ref()
                }

                pub fn mut_opt(&mut self) -> ::core::option::Option<&mut bool> {
                    self.opt.as_mut()
                }

                pub fn set_opt(&mut self, value: bool) {
                    self.opt = ::core::option::Option::Some(value.into());
                }

                pub fn clear_opt(&mut self) {
                    self.opt = ::core::option::Option::None;
                }
            }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }
}
