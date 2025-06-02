use std::{collections::HashMap, io};

use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    descriptor::DescriptorProto,
    generator::{
        field::{CustomField, FieldType},
        resolve_path_elem, EncodeFunc,
    },
};

use super::{
    derive_msg_attr,
    field::Field,
    field_error, msg_error,
    oneof::{Oneof, OneofField, OneofType},
    sanitized_ident,
    type_spec::find_lifetime_from_type,
    CurrentConfig, Generator,
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Message<'a> {
    /// Protobuf name
    pub(crate) name: &'a str,
    /// Sanitized Rust ident, used for struct name
    pub(crate) rust_name: Ident,
    pub(crate) oneofs: Vec<Oneof<'a>>,
    pub(crate) fields: Vec<Field<'a>>,
    pub(crate) derive_dbg: bool,
    pub(crate) impl_default: bool,
    pub(crate) impl_partial_eq: bool,
    pub(crate) derive_clone: bool,
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

        let msg_name = &proto.name;
        let mut oneofs = vec![];
        for (idx, oneof) in proto.oneof_decl.iter().enumerate() {
            let oneof = Oneof::from_proto(oneof, msg_conf.next_conf(&oneof.name), idx)
                .map_err(|e| field_error(&gen.pkg, msg_name, &oneof.name, &e))?;
            if let Some(oneof) = oneof {
                oneofs.push(oneof);
            }
        }

        let mut map_types = HashMap::new();
        for m in &proto.nested_type {
            if m.options().map(|o| o.map_entry).unwrap_or(false) {
                map_types.insert(m.name.as_str(), m);
            }
        }

        let mut synthetic_oneof_idx = vec![];

        let mut fields = vec![];
        for f in proto.field.iter() {
            let field_conf = msg_conf.next_conf(&f.name);
            let raw_msg_name = f
                .type_name
                .rsplit_once('.')
                .map(|(_, r)| r)
                .unwrap_or(&f.type_name);
            let field = if let Some(map_msg) = map_types.remove(raw_msg_name) {
                Field::from_proto(f, &field_conf, gen.syntax, Some(map_msg))
                    .map_err(|e| field_error(&gen.pkg, msg_name, &f.name, &e))?
            } else {
                if let Some(idx) = f.oneof_index().copied() {
                    if f.proto3_optional {
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
                                    .map_err(|e| field_error(&gen.pkg, msg_name, &f.name, &e))?
                                {
                                    fields.push(field);
                                }
                            }
                            Some(OneofType::Custom { nums, .. }) => {
                                if !field_conf.config.skip.unwrap_or(false) {
                                    nums.push(f.number);
                                }
                            }
                            _ => (),
                        }
                        continue;
                    }
                }
                // Normal field
                Field::from_proto(f, &field_conf, gen.syntax, None)
                    .map_err(|e| field_error(&gen.pkg, msg_name, &f.name, &e))?
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
            rust_name: sanitized_ident(msg_name),
            oneofs,
            fields,
            derive_dbg: msg_conf.derive_dbg(),
            impl_default: msg_conf.impl_default(),
            impl_partial_eq: msg_conf.derive_partial_eq(),
            derive_clone: msg_conf.derive_clone(),
            attrs,
            unknown_handler,
            lifetime,
        }))
    }

    pub(crate) fn generate_hazzer_decl(
        &self,
        conf: CurrentConfig,
    ) -> Result<Option<(TokenStream, Vec<syn::Attribute>)>, String> {
        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let attrs = &conf.config.type_attr_parsed()?;
        let derive_msg = derive_msg_attr(true, true, true, true);

        let hazzers = self.fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        if count == 0 {
            return Ok(None);
        }

        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = &f.san_rust_name;
            let setter = format_ident!("set_{}", f.rust_name);
            let clearer = format_ident!("clear_{}", f.rust_name);
            let init = format_ident!("init_{}", f.rust_name);
            let idx = Literal::usize_unsuffixed(i / 8);
            let mask = Literal::u8_unsuffixed(1 << (i % 8));

            let getter_doc = format!("Query presence of `{}`", f.rust_name);
            let setter_doc = format!("Set presence of `{}`", f.rust_name);
            let clearer_doc = format!("Clear presence of `{}`", f.rust_name);
            let init_doc = format!("Builder method that sets the presence of `{}`. Useful for initializing the Hazzer.", f.rust_name);

            quote! {
                #[doc = #getter_doc]
                #[inline]
                pub fn #fname(&self) -> bool {
                    (self.0[#idx] & #mask) != 0
                }

                #[doc = #setter_doc]
                #[inline]
                pub fn #setter(&mut self) -> &mut Self {
                    let elem = &mut self.0[#idx];
                    *elem |= #mask;
                    self
                }

                #[doc = #clearer_doc]
                #[inline]
                pub fn #clearer(&mut self) -> &mut Self {
                    let elem = &mut self.0[#idx];
                    *elem &= !#mask;
                    self
                }

                #[doc = #init_doc]
                #[inline]
                pub fn #init(mut self) -> Self {
                    self.#setter();
                    self
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
        let msg_mod_name = resolve_path_elem(self.name);
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

        let derive_msg = derive_msg_attr(self.derive_dbg, false, false, self.derive_clone);
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
        if !self.impl_default {
            return Ok(quote! {});
        }

        let mut field_defaults = TokenStream::new();
        for f in &self.fields {
            // Skip delegate fields when generating defaults
            if !matches!(f.ftype, FieldType::Custom(CustomField::Delegate(_))) {
                let name = &f.san_rust_name;
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
                Some(&o.san_rust_name)
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

    pub(crate) fn generate_partial_eq(&self) -> TokenStream {
        if !self.impl_partial_eq {
            return quote! {};
        }

        let ret_name = Ident::new("ret", Span::call_site());
        let other_name = Ident::new("other", Span::call_site());
        let mut body = TokenStream::new();
        for f in &self.fields {
            let fname = &f.san_rust_name;
            let comparison = match f.ftype {
                // Retain Option comparison semantics even when using Hazzers
                FieldType::Optional(..) => {
                    quote! { #ret_name &= (self.#fname() == #other_name.#fname()); }
                }
                FieldType::Custom(CustomField::Delegate(_)) => quote! {},
                _ => quote! { #ret_name &= (self.#fname == #other_name.#fname); },
            };
            body.extend(comparison);
        }

        for o in &self.oneofs {
            if let OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } = o.otype
            {
            } else {
                let oname = &o.san_rust_name;
                body.extend(quote! { #ret_name &= (self.#oname == #other_name.#oname); });
            }
        }

        let rust_name = &self.rust_name;
        let lifetime = &self.lifetime;
        quote! {
            impl<#lifetime> ::core::cmp::PartialEq for #rust_name<#lifetime> {
                fn eq(&self, #other_name: &Self) -> bool {
                    let mut #ret_name = true;
                    #body
                    #ret_name
                }
            }
        }
    }

    pub(crate) fn generate_impl(&self, gen: &Generator) -> TokenStream {
        let accessors = self.fields.iter().map(|f| f.generate_accessors(gen));
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
        let mod_name = resolve_path_elem(self.name);

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
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    #decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>>
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
        let mod_name = resolve_path_elem(self.name);

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

    fn generate_max_size(&self, gen: &Generator) -> TokenStream {
        let field_sizes = self.fields.iter().map(|f| f.generate_max_size(gen));
        let oneof_sizes = self.oneofs.iter().map(|o| o.generate_max_size(gen));
        let unknown_size = self
            .unknown_handler
            .as_ref()
            .map(|handler| quote! { <#handler as ::micropb::field::FieldEncode>::MAX_SIZE });
        let sizes = field_sizes.chain(oneof_sizes).chain(unknown_size);

        quote! {
            const MAX_SIZE: ::core::option::Option<usize> = 'msg: {
                let mut max_size = 0;
                #(
                    if let ::core::option::Option::Some(size) = #sizes {
                        max_size += size;
                    } else {
                        break 'msg (::core::option::Option::<usize>::None);
                    };
                )*
                ::core::option::Option::Some(max_size)
            };
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
        let max_size_decl = self.generate_max_size(gen);

        quote! {
            impl<#lifetime> ::micropb::MessageEncode for #name<#lifetime> {
                #max_size_decl

                fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                    &self,
                    encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                ) -> Result<(), IMPL_MICROPB_WRITE::Error>
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

    use crate::{
        config::{parse_attributes, Config, IntSize, OptionalRepr},
        descriptor::{
            FieldDescriptorProto,
            FieldDescriptorProto_::{Label, Type},
            FieldOptions, MessageOptions, OneofDescriptorProto,
        },
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
        let mut map_msg = DescriptorProto::default();
        map_msg.set_name("MapElem".to_owned());
        map_msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(1);
            f.set_name("key".to_owned());
            f.set_type(Type::Int64);
            f
        });
        map_msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(2);
            f.set_name("value".to_owned());
            f.set_type(Type::Uint64);
            f
        });
        map_msg.set_options({
            let mut o = MessageOptions::default();
            o.set_map_entry(true);
            o
        });

        let mut msg = DescriptorProto::default();
        msg.set_name("Message".to_owned());
        msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(1);
            f.set_name("bool_field".to_owned());
            f.set_type(Type::Bool);
            f
        });
        msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(2);
            f.set_name("oneof_field".to_owned());
            f.set_type(Type::Sint32);
            f.set_oneof_index(0);
            f
        });
        msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(3);
            f.set_name("map_field".to_owned());
            f.set_type(Type::Message);
            f.set_label(Label::Repeated);
            f.set_type_name(".Message.MapElem".to_owned());
            f.set_options(FieldOptions::default());
            f.options.set_packed(true);
            f
        });
        msg.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(4);
            f.set_name("oneof_field2".to_owned());
            f.set_type(Type::Float);
            f.set_oneof_index(0);
            f
        });
        msg.oneof_decl.push({
            let mut o = OneofDescriptorProto::default();
            o.set_name("oneof".to_owned());
            o
        });
        msg.nested_type.push(map_msg);
        msg
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
            impl_default: true,
            impl_partial_eq: true,
            derive_clone: true,
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
                    san_rust_name: Ident::new_raw("oneof", Span::call_site()),
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
                    encoded_max_size: None,
                    field_attrs: vec![],
                    // Overrides the type attrs of the message
                    type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
                    boxed: false,
                    // Inherits the no_debug_derive setting of the message
                    derive_dbg: false,
                    derive_partial_eq: true,
                    derive_clone: true,
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
                impl_default: false,
                impl_partial_eq: true,
                derive_clone: true,
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

        let mut proto = DescriptorProto::default();
        proto.set_name("Msg".to_owned());
        proto.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(1);
            f.set_name("opt".to_owned());
            f.set_type(Type::Bool);
            f.set_proto3_optional(true);
            f.set_oneof_index(0);
            f
        });
        proto.oneof_decl.push({
            let mut o = OneofDescriptorProto::default();
            o.set_name("_opt".to_owned());
            o
        });
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
                impl_default: true,
                impl_partial_eq: true,
                derive_clone: true,
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
            impl_default: true,
            impl_partial_eq: true,
            derive_clone: true,
            attrs: vec![],
            unknown_handler: None,
            lifetime: None,
        };
        assert!(msg.generate_hazzer_decl(config).unwrap().is_none());
    }
}
