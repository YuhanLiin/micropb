use std::{
    collections::{HashMap, HashSet},
    io,
};

use proc_macro2::{Literal, Span, TokenStream};
use prost_types::DescriptorProto;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    config::OptionalRepr,
    generator::{
        field::{CustomField, FieldType},
        EncodeFunc,
    },
};

use super::{
    derive_msg_attr,
    field::Field,
    field_error, msg_error,
    oneof::{Oneof, OneofField, OneofType},
    type_spec::find_lifetime_from_type,
    CurrentConfig, Generator,
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Message<'a> {
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) oneofs: Vec<Oneof<'a>>,
    pub(crate) fields: Vec<Field<'a>>,
    pub(crate) derive_dbg: bool,
    pub(crate) derive_default: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) unknown_handler: Option<syn::Type>,
    pub(crate) lifetime: Option<syn::Lifetime>,
}

impl<'a> Message<'a> {
    pub(crate) fn from_proto(
        proto: &'a DescriptorProto,
        gen: &Generator,
        msg_conf: &CurrentConfig,
    ) -> io::Result<Option<Self>> {
        if msg_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let msg_name = proto.name();
        let mut oneofs = vec![];
        for (idx, oneof) in proto.oneof_decl.iter().enumerate() {
            let oneof = Oneof::from_proto(oneof, msg_conf.next_conf(oneof.name()), idx)
                .map_err(|e| field_error(&gen.pkg, msg_name, oneof.name(), &e))?;
            if let Some(oneof) = oneof {
                oneofs.push(oneof);
            }
        }

        let mut map_types = HashMap::new();
        for m in &proto.nested_type {
            if m.options.as_ref().map(|o| o.map_entry()).unwrap_or(false) {
                map_types.insert(m.name(), m);
            }
        }

        let mut synthetic_oneof_idx = vec![];

        let mut fields = vec![];
        for f in proto.field.iter() {
            let field_conf = msg_conf.next_conf(f.name());
            let raw_msg_name = f
                .type_name()
                .rsplit_once('.')
                .map(|(_, r)| r)
                .unwrap_or(f.type_name());
            let field = if let Some(map_msg) = map_types.remove(raw_msg_name) {
                Field::from_proto(f, &field_conf, gen.syntax, Some(map_msg))
                    .map_err(|e| field_error(&gen.pkg, msg_name, f.name(), &e))?
            } else {
                if let Some(idx) = f.oneof_index {
                    if f.proto3_optional() {
                        synthetic_oneof_idx.push(idx as usize);
                    } else {
                        match oneofs
                            .iter_mut()
                            .find(|o| o.idx == idx as usize)
                            .map(|o| &mut o.otype)
                        {
                            Some(OneofType::Enum { fields, .. }) => {
                                // Oneof field
                                if let Some(field) = OneofField::from_proto(f, &field_conf)
                                    .map_err(|e| field_error(&gen.pkg, msg_name, f.name(), &e))?
                                {
                                    fields.push(field);
                                }
                            }
                            Some(OneofType::Custom { nums, .. }) => {
                                if !field_conf.config.skip.unwrap_or(false) {
                                    nums.push(f.number());
                                }
                            }
                            _ => (),
                        }
                        continue;
                    }
                }
                // Normal field
                Field::from_proto(f, &field_conf, gen.syntax, None)
                    .map_err(|e| field_error(&gen.pkg, msg_name, f.name(), &e))?
            };
            if let Some(field) = field {
                fields.push(field);
            }
        }

        // Remove all oneofs that are empty enums or synthetic oneofs
        let oneofs: Vec<_> = oneofs
            .into_iter()
            .filter(|o| !matches!(&o.otype, OneofType::Enum { fields, .. } if fields.is_empty()))
            .filter(|o| !synthetic_oneof_idx.contains(&o.idx))
            .collect();

        let attrs = msg_conf
            .config
            .type_attr_parsed()
            .map_err(|e| msg_error(&gen.pkg, msg_name, &e))?;
        let unknown_handler = msg_conf
            .config
            .unknown_handler_parsed()
            .map_err(|e| msg_error(&gen.pkg, msg_name, &e))?;

        // Find any lifetime in the message definition (we only need one)
        let lifetime = fields
            .iter()
            .find_map(|f| f.find_lifetime())
            .or_else(|| oneofs.iter().find_map(|o| o.find_lifetime()))
            .or_else(|| unknown_handler.as_ref().and_then(find_lifetime_from_type))
            .cloned();

        Ok(Some(Self {
            name: msg_name,
            rust_name: Ident::new(msg_name, Span::call_site()),
            oneofs,
            fields,
            derive_dbg: !msg_conf.config.no_debug_impl.unwrap_or(false),
            derive_default: !msg_conf.config.no_default_impl.unwrap_or(false),
            attrs,
            unknown_handler,
            lifetime,
        }))
    }

    pub(crate) fn check_delegates(&self, gen: &Generator) -> io::Result<()> {
        let ocustoms = self.oneofs.iter().filter_map(|o| o.custom_type_field());
        let fcustoms = self.fields.iter().filter_map(|f| f.custom_type_field());
        let customs: HashSet<_> = ocustoms.chain(fcustoms).collect();

        let odelegates = self
            .oneofs
            .iter()
            .filter_map(|o| o.delegate().map(|d| (d, o.name)));
        let fdelegates = self
            .fields
            .iter()
            .filter_map(|f| f.delegate().map(|d| (d, f.name)));
        for (delegate, fname) in odelegates.chain(fdelegates) {
            let delegate = delegate.to_string();
            if !customs.contains(delegate.as_str()) {
                return Err(field_error(
                    &gen.pkg,
                    self.name,
                    fname,
                    &format!(
                        "Delegate field refers to custom field of {delegate}, which doesn't exist"
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn generate_hazzer_decl(
        &self,
        conf: CurrentConfig,
    ) -> Result<Option<(TokenStream, Vec<syn::Attribute>)>, String> {
        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let attrs = &conf.config.type_attr_parsed()?;
        let derive_msg = derive_msg_attr(!conf.config.no_debug_impl.unwrap_or(false), true);

        let hazzers = self.fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        if count == 0 {
            return Ok(None);
        }

        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = &f.raw_rust_name;
            let setter = format_ident!("set_{}", f.rust_name);
            let idx = Literal::usize_unsuffixed(i / 8);
            let mask = Literal::u8_unsuffixed(1 << (i % 8));

            quote! {
                #[inline]
                pub fn #fname(&self) -> bool {
                    (self.0[#idx] & #mask) != 0
                }

                #[inline]
                pub fn #setter(&mut self, val: bool) {
                    let elem = &mut self.0[#idx];
                    if val {
                        *elem |= #mask;
                    } else {
                        *elem &= !#mask;
                    }
                }
            }
        });

        let bytes = Literal::usize_unsuffixed(count.div_ceil(8));
        let decl = quote! {
            #derive_msg
            #(#attrs)*
            pub struct #hazzer_name([u8; #bytes]);

            impl #hazzer_name {
                #(#methods)*
            }
        };
        Ok(Some((decl, conf.config.field_attr_parsed()?)))
    }

    pub(crate) fn generate_decl(
        &self,
        gen: &Generator,
        hazzer_field_attr: Option<Vec<syn::Attribute>>,
        unknown_conf: &CurrentConfig,
    ) -> io::Result<TokenStream> {
        let msg_mod_name = gen.resolve_path_elem(self.name);
        let rust_name = &self.rust_name;
        let lifetime = &self.lifetime;
        let msg_fields = self.fields.iter().map(|f| f.generate_field(gen));
        let hazzer_field_attr = hazzer_field_attr.iter();
        let oneof_fields = self
            .oneofs
            .iter()
            .map(|oneof| oneof.generate_field(gen, &msg_mod_name));

        let unknown_field = if let Some(handler) = &self.unknown_handler {
            let unknown_field_attr = unknown_conf
                .config
                .field_attr_parsed()
                .map_err(|e| field_error(&gen.pkg, self.name, "_unknown", &e))?;
            quote! { #(#unknown_field_attr)* pub _unknown: #handler, }
        } else {
            quote! {}
        };

        let derive_msg = derive_msg_attr(self.derive_dbg, false);
        let attrs = &self.attrs;

        Ok(quote! {
            #derive_msg
            #(#attrs)*
            pub struct #rust_name<#lifetime> {
                #(#msg_fields)*
                #(#oneof_fields)*
                #(#(#hazzer_field_attr)* pub _has: #msg_mod_name::_Hazzer,)*
                #unknown_field
            }
        })
    }

    pub(crate) fn generate_default_impl(
        &self,
        gen: &Generator,
        use_hazzer: bool,
    ) -> io::Result<TokenStream> {
        if !self.derive_default {
            return Ok(quote! {});
        }

        let mut field_defaults = TokenStream::new();
        for f in &self.fields {
            // Skip delegate fields when generating defaults
            if !matches!(f.ftype, FieldType::Custom(CustomField::Delegate(_))) {
                let name = &f.raw_rust_name;
                let default = f
                    .generate_default(gen)
                    .map_err(|e| field_error(&gen.pkg, self.name, f.name, &e))?;
                field_defaults.extend(quote! { #name: #default, });
            }
        }

        let oneof_names = self.oneofs.iter().filter_map(|o| {
            if let OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } = o.otype
            {
                None
            } else {
                Some(&o.raw_rust_name)
            }
        });
        let hazzer_default =
            use_hazzer.then(|| quote! { _has: ::core::default::Default::default(), });
        let unknown_default = self
            .unknown_handler
            .as_ref()
            .map(|_| quote! { _unknown: ::core::default::Default::default(), });
        let rust_name = &self.rust_name;
        let lifetime = &self.lifetime;

        Ok(quote! {
            impl<#lifetime> ::core::default::Default for #rust_name<#lifetime> {
                fn default() -> Self {
                    Self {
                        #field_defaults
                        #(#oneof_names: ::core::default::Default::default(),)*
                        #hazzer_default
                        #unknown_default
                    }
                }
            }
        })
    }

    pub(crate) fn generate_impl(&self, gen: &Generator) -> TokenStream {
        let accessors = self.fields.iter().map(|f| {
            if let FieldType::Optional(type_spec, opt) = &f.ftype {
                let type_name = type_spec.generate_rust_type(gen);
                let setter_name = format_ident!("set_{}", f.rust_name);
                let muter_name = format_ident!("mut_{}", f.rust_name);
                let clearer_name = format_ident!("clear_{}", f.rust_name);
                let fname = &f.raw_rust_name;

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
        let lifetime = &self.lifetime;
        quote! {
            impl<#lifetime> #name<#lifetime> {
                #(#accessors)*
            }
        }
    }

    pub(crate) fn generate_decode_trait(&self, gen: &Generator) -> TokenStream {
        let name = &self.rust_name;
        let lifetime = &self.lifetime;
        let tag = Ident::new("tag", Span::call_site());
        let decoder = Ident::new("decoder", Span::call_site());
        let mod_name = gen.resolve_path_elem(self.name);

        let field_branches = self
            .fields
            .iter()
            .map(|f| f.generate_decode_branch(gen, &tag, &decoder));
        let oneof_branches = self
            .oneofs
            .iter()
            .map(|o| o.generate_decode_branches(gen, &mod_name, &tag, &decoder));

        let unknown_branch = if self.unknown_handler.is_some() {
            // If the unknown handler can't handle a field, skip it
            quote! { if !self._unknown.decode_field(#tag, #decoder)? { #decoder.skip_wire_value(#tag.wire_type())?; } }
        } else {
            quote! { #decoder.skip_wire_value(#tag.wire_type())?; }
        };

        quote! {
            impl<#lifetime> ::micropb::MessageDecode for #name<#lifetime> {
                fn decode<R: ::micropb::PbRead>(
                    &mut self,
                    #decoder: &mut ::micropb::PbDecoder<R>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<R::Error>>
                {
                    use ::micropb::{PbVec, PbMap, PbString, FieldDecode};

                    let before = #decoder.bytes_read();
                    while #decoder.bytes_read() - before < len {
                        let #tag = #decoder.decode_tag()?;
                        match #tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            #(#field_branches)*
                            #(#oneof_branches)*
                            _ => { #unknown_branch }
                        }
                    }
                    Ok(())
                }
            }
        }
    }

    fn generate_encode_func(&self, gen: &Generator, func_type: &EncodeFunc) -> TokenStream {
        let mod_name = gen.resolve_path_elem(self.name);

        let field_logic = self
            .fields
            .iter()
            .map(|f| f.generate_encode(gen, func_type));
        let oneof_logic = self
            .oneofs
            .iter()
            .map(|o| o.generate_encode(gen, &mod_name, func_type));

        let unknown_logic = if self.unknown_handler.is_some() {
            match func_type {
                EncodeFunc::Sizeof(size) => {
                    quote! { #size += self._unknown.compute_fields_size(); }
                }
                EncodeFunc::Encode(encoder) => quote! { self._unknown.encode_fields(#encoder)?; },
            }
        } else {
            quote! {}
        };

        quote! {
            #(#field_logic)*
            #(#oneof_logic)*
            #unknown_logic
        }
    }

    pub(crate) fn generate_encode_trait(&self, gen: &Generator) -> TokenStream {
        let name = &self.rust_name;
        let lifetime = &self.lifetime;
        let sizeof = self.generate_encode_func(
            gen,
            &EncodeFunc::Sizeof(Ident::new("size", Span::call_site())),
        );
        let encode = self.generate_encode_func(
            gen,
            &EncodeFunc::Encode(Ident::new("encoder", Span::call_site())),
        );

        quote! {
            impl<#lifetime> ::micropb::MessageEncode for #name<#lifetime> {
                fn encode<W: ::micropb::PbWrite>(
                    &self,
                    encoder: &mut ::micropb::PbEncoder<W>,
                ) -> Result<(), W::Error>
                {
                    use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                    #encode
                    Ok(())
                }

                fn compute_size(&self) -> usize {
                    use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                    let mut size = 0;
                    #sizeof
                    size
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
        config::{parse_attributes, Config, IntSize, OptionalRepr},
        generator::{
            field::{make_test_field, FieldType},
            oneof::make_test_oneof_field,
            type_spec::{PbInt, TypeSpec},
            Syntax,
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
        let gen = Generator::new();
        assert!(Message::from_proto(&proto, &gen, &msg_conf)
            .unwrap()
            .is_none());
    }

    #[test]
    fn from_proto_skip_fields() {
        let gen = Generator::new();
        let proto = test_msg_proto();
        let empty_msg = Message {
            name: "Message",
            rust_name: Ident::new("Message", Span::call_site()),
            oneofs: vec![],
            fields: vec![],
            derive_dbg: true,
            derive_default: true,
            attrs: vec![],
            unknown_handler: None,
            lifetime: None,
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
            Message::from_proto(&proto, &gen, &msg_conf)
                .unwrap()
                .unwrap(),
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
            Message::from_proto(&proto, &gen, &msg_conf)
                .unwrap()
                .unwrap(),
            empty_msg
        );
    }

    #[test]
    fn from_proto() {
        let gen = Generator::new();
        let proto = test_msg_proto();
        let config = Box::new(
            Config::new()
                .map_type("Map")
                .type_attributes("#[derive(Self)]")
                .no_debug_impl(true)
                .no_default_impl(true)
                .unknown_handler("UnknownType"),
        );
        let mut node = Node::default();
        *node.add_path(["bool_field"].into_iter()).value_mut() =
            Some(Box::new(Config::new().boxed(true)));
        *node.add_path(["oneof_field"].into_iter()).value_mut() =
            Some(Box::new(Config::new().int_size(IntSize::S8)));
        *node.add_path(["oneof_field2"].into_iter()).value_mut() =
            Some(Box::new(Config::new().boxed(true)));
        *node.add_path(["oneof"].into_iter()).value_mut() =
            Some(Box::new(Config::new().type_attributes("#[derive(Eq)]")));
        *node.add_path(["map_field", "key"].into_iter()).value_mut() =
            Some(Box::new(Config::new().int_size(IntSize::S16)));
        *node
            .add_path(["map_field", "value"].into_iter())
            .value_mut() = Some(Box::new(Config::new().int_size(IntSize::S16)));
        let msg_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        assert_eq!(
            Message::from_proto(&proto, &gen, &msg_conf)
                .unwrap()
                .unwrap(),
            Message {
                name: "Message",
                rust_name: Ident::new("Message", Span::call_site()),
                oneofs: vec![Oneof {
                    name: "oneof",
                    rust_name: "oneof".to_owned(),
                    raw_rust_name: Ident::new_raw("oneof", Span::call_site()),
                    otype: OneofType::Enum {
                        type_name: Ident::new("Oneof", Span::call_site()),
                        fields: vec![
                            make_test_oneof_field(
                                2,
                                "oneof_field",
                                false,
                                TypeSpec::Int(PbInt::Sint32, IntSize::S8)
                            ),
                            make_test_oneof_field(4, "oneof_field2", true, TypeSpec::Float),
                        ]
                    },
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
                            key: TypeSpec::Int(PbInt::Int64, IntSize::S16),
                            val: TypeSpec::Int(PbInt::Uint64, IntSize::S16),
                            type_path: syn::parse_str("Map").unwrap(),
                            max_len: None
                        }
                    ),
                ],
                derive_dbg: false,
                derive_default: false,
                attrs: parse_attributes("#[derive(Self)]").unwrap(),
                unknown_handler: Some(syn::parse_str("UnknownType").unwrap()),
                lifetime: None
            }
        )
    }

    #[test]
    fn synthetic_oneof() {
        let mut gen = Generator::new();
        gen.syntax = Syntax::Proto3;

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
            Message::from_proto(&proto, &gen, &msg_conf)
                .unwrap()
                .unwrap(),
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
                derive_default: true,
                attrs: vec![],
                unknown_handler: None,
                lifetime: None
            }
        )
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
            derive_default: true,
            attrs: vec![],
            unknown_handler: None,
            lifetime: None,
        };
        assert!(msg.generate_hazzer_decl(config).unwrap().is_none());
    }
}
