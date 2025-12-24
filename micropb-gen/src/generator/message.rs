use std::{collections::HashMap, io};

use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    descriptor::DescriptorProto,
    generator::{
        Context, EncodeFunc,
        field::{CustomField, FieldType},
        graph::Position,
        location::{self, next_comment_node},
        resolve_path_elem,
        type_spec::TypeSpec,
    },
    utils::{TryIntoTokens, find_lifetime_from_type},
};

use super::{
    CurrentConfig, derive_msg_attr,
    field::Field,
    field_error,
    location::{CommentNode, Comments},
    msg_error,
    oneof::{Oneof, OneofField, OneofType},
    sanitized_ident,
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Hazzer {
    type_attrs: Vec<syn::Attribute>,
    field_attrs: Vec<syn::Attribute>,
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Unknown {
    handler: syn::Type,
    field_attrs: Vec<syn::Attribute>,
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Message<'proto> {
    /// Protobuf name
    pub(crate) name: &'proto str,
    /// Sanitized Rust ident, used for struct name
    pub(crate) rust_name: Ident,
    pub(crate) oneofs: Vec<Oneof<'proto>>,
    pub(crate) fields: Vec<Field<'proto>>,
    pub(crate) derive_dbg: bool,
    pub(crate) impl_default: bool,
    pub(crate) impl_partial_eq: bool,
    pub(crate) derive_clone: bool,
    // Will be populated by graph resolver
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) unknown: Option<Unknown>,
    pub(crate) as_oneof_enum: bool,
    pub(crate) hazzer: Option<Hazzer>,
    comments: Option<&'proto Comments>,
    pub(crate) message_edges: Vec<(Position, &'proto str)>,

    // These fields are populated after the constructor
    pub(crate) parent_edges: Vec<(Position, String)>,
    pub(crate) is_copy: bool,
    pub(crate) lifetime: Option<syn::Lifetime>,
}

impl<'proto> Message<'proto> {
    pub(crate) fn from_proto(
        proto: &'proto DescriptorProto,
        ctx: &Context<'proto>,
        msg_conf: &CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
    ) -> io::Result<Option<Self>> {
        if msg_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let msg_name = &proto.name;
        let mut oneofs = vec![];
        for (idx, oneof) in proto.oneof_decl.iter().enumerate() {
            let oneof = Oneof::from_proto(
                oneof,
                msg_conf.next_conf(&oneof.name),
                next_comment_node(comment_node, location::path::msg_oneof(idx)),
                idx,
            )
            .map_err(|e| field_error(&ctx.pkg, msg_name, &oneof.name, &e))?;
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

        let mut message_edges = vec![];
        let mut fields = vec![];
        for (i, f) in proto.field.iter().enumerate() {
            let field_conf = msg_conf.next_conf(&f.name);
            let field_comments = next_comment_node(comment_node, location::path::msg_field(i));

            let raw_msg_name = f
                .type_name
                .rsplit_once('.')
                .map(|(_, r)| r)
                .unwrap_or(&f.type_name);

            let field = if let Some(map_msg) = map_types.remove(raw_msg_name) {
                Field::from_proto(f, &field_conf, field_comments, ctx, Some(map_msg))
                    .map_err(|e| field_error(&ctx.pkg, msg_name, &f.name, &e))?
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
                            Some(OneofType::Enum {
                                fields: oneof_fields,
                                ..
                            }) => {
                                // Oneof field
                                if let Some(field) =
                                    OneofField::from_proto(f, &field_conf, field_comments)
                                        .map_err(|e| field_error(&ctx.pkg, msg_name, &f.name, &e))?
                                {
                                    if let TypeSpec::Message(field_name) = field.tspec {
                                        message_edges.push((
                                            Position::Oneof(idx as usize, oneof_fields.len()),
                                            field_name,
                                        ));
                                    }
                                    oneof_fields.push(field);
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
                Field::from_proto(f, &field_conf, field_comments, ctx, None)
                    .map_err(|e| field_error(&ctx.pkg, msg_name, &f.name, &e))?
            };
            if let Some(field) = field {
                if let Some(field_name) = field.message_name() {
                    message_edges.push((Position::Field(fields.len()), field_name));
                }
                fields.push(field);
            }
        }
        // Only include fields that aren't handled externally
        let message_edges = message_edges
            .into_iter()
            .filter(|(_, fq_proto_name)| !ctx.params.extern_paths.contains_key(*fq_proto_name))
            .collect();

        // Remove all oneofs that are empty enums or synthetic oneofs
        let oneofs: Vec<_> = oneofs
            .into_iter()
            .filter(|o| !matches!(&o.otype, OneofType::Enum { fields, .. } if fields.is_empty()))
            .filter(|o| !synthetic_oneof_idx.contains(&o.idx))
            .collect();

        let attrs = msg_conf
            .config
            .type_attr_parsed()
            .map_err(|e| msg_error(&ctx.pkg, msg_name, &e))?;

        let as_enum = ctx.params.single_oneof_msg_as_enum
            && fields.is_empty()
            && oneofs.len() == 1
            && matches!(oneofs[0].otype, OneofType::Enum { .. });

        let unknown = if let Some(handler) = msg_conf
            .config
            .unknown_handler_parsed()
            .map_err(|e| msg_error(&ctx.pkg, msg_name, &e))?
        {
            let unknown_conf = msg_conf.next_conf("_unknown");
            Some(Unknown {
                handler,
                field_attrs: unknown_conf
                    .config
                    .field_attr_parsed()
                    .map_err(|e| field_error(&ctx.pkg, msg_name, "_unknown", &e))?,
            })
        } else {
            None
        };

        let is_hazzer = !as_enum && fields.iter().any(|f| f.is_hazzer());
        let hazzer = if is_hazzer {
            let hazzer_conf = msg_conf.next_conf("_has");
            Some(Hazzer {
                type_attrs: hazzer_conf
                    .config
                    .type_attr_parsed()
                    .map_err(|e| field_error(&ctx.pkg, msg_name, "_has", &e))?,
                field_attrs: hazzer_conf
                    .config
                    .field_attr_parsed()
                    .map_err(|e| field_error(&ctx.pkg, msg_name, "_has", &e))?,
            })
        } else {
            None
        };

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
            unknown,
            as_oneof_enum: as_enum,
            hazzer,
            comments: location::get_comments(comment_node),
            message_edges,

            parent_edges: vec![],
            lifetime: None,
            is_copy: false,
        }))
    }

    pub(crate) fn find_lifetime(&mut self) -> Option<&syn::Lifetime> {
        // Find any lifetime in the message definition (we only need one)
        self.lifetime = self
            .fields
            .iter()
            .find_map(|f| f.find_lifetime())
            .or_else(|| {
                self.oneofs
                    .iter_mut()
                    .find_map(|o| o.find_lifetime())
                    .cloned()
            })
            .or_else(|| {
                self.unknown
                    .as_ref()
                    .map(|u| &u.handler)
                    .and_then(find_lifetime_from_type)
                    .cloned()
            });
        self.lifetime.as_ref()
    }

    pub(crate) fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        self.unknown.is_none()
            && self.oneofs.iter().all(|oneof| oneof.is_copy(ctx))
            && self.fields.iter().all(|f| f.is_copy(ctx))
    }

    pub(crate) fn generate_hazzer_decl(&self) -> Option<TokenStream> {
        let Some(Hazzer { type_attrs, .. }) = &self.hazzer else {
            return None;
        };

        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let derive_msg = derive_msg_attr(true, true, true, true, true);

        let hazzers = self.fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();

        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = &f.san_rust_name;
            let setter = format_ident!("set_{}", f.rust_name);
            let clearer = format_ident!("clear_{}", f.rust_name);
            let init = format_ident!("init_{}", f.rust_name);
            let idx = Literal::usize_unsuffixed(i / 8);
            let mask = Literal::u8_unsuffixed(1 << (i % 8));

            let getter_doc = format!(" Query presence of `{}`", f.rust_name);
            let setter_doc = format!(" Set presence of `{}`", f.rust_name);
            let clearer_doc = format!(" Clear presence of `{}`", f.rust_name);
            let init_doc = format!(" Builder method that sets the presence of `{}`. Useful for initializing the Hazzer.", f.rust_name);

            quote! {
                #[doc = #getter_doc]
                #[inline]
                pub const fn #fname(&self) -> bool {
                    (self.0[#idx] & #mask) != 0
                }

                #[doc = #setter_doc]
                #[inline]
                pub const fn #setter(&mut self) -> &mut Self {
                    let elem = &mut self.0[#idx];
                    *elem |= #mask;
                    self
                }

                #[doc = #clearer_doc]
                #[inline]
                pub const fn #clearer(&mut self) -> &mut Self {
                    let elem = &mut self.0[#idx];
                    *elem &= !#mask;
                    self
                }

                #[doc = #init_doc]
                #[inline]
                pub const fn #init(mut self) -> Self {
                    self.#setter();
                    self
                }
            }
        });

        let bytes = Literal::usize_unsuffixed(count.div_ceil(8));
        let decl = quote! {
            #[doc = " Compact bitfield for tracking presence of optional and message fields"]
            #derive_msg
            #(#type_attrs)*
            pub struct #hazzer_name([u8; #bytes]);

            impl #hazzer_name {
                #[doc = " New hazzer with all fields set to off"]
                #[inline]
                pub const fn _new() -> Self {
                    Self([0; #bytes])
                }

                #(#methods)*
            }
        };
        Some(decl)
    }

    pub(crate) fn generate_decl(
        &self,
        ctx: &Context<'proto>,
        proto_default: bool,
    ) -> io::Result<TokenStream> {
        let msg_mod_name = resolve_path_elem(self.name, true);
        let rust_name = &self.rust_name;
        let lifetime = &self.lifetime;
        let attrs = &self.attrs;

        // If message has no hazzer, then we can derive PartialEq instead of implementing it
        let derive_partial_eq = self.impl_partial_eq && self.hazzer.is_none();
        // If message has no Proto default specification, then we can derive Default
        let derive_default = self.impl_default && !proto_default;
        let derive_msg = derive_msg_attr(
            self.derive_dbg,
            derive_default,
            derive_partial_eq,
            self.derive_clone,
            self.is_copy,
        );
        let comments = self.comments.map(Comments::lines).into_iter().flatten();

        if self.as_oneof_enum {
            let OneofType::Enum { fields, .. } = &self.oneofs[0].otype else {
                unreachable!("shouldn't generate enum with custom oneof")
            };
            let variants = fields
                .iter()
                .map(|f| {
                    f.generate_field(ctx)
                        .map_err(|e| field_error(&ctx.pkg, self.name, f.name, &e))
                })
                .try_into_tokens()?;
            let default_variant_attr = derive_default.then(|| quote! { #[default] });

            Ok(quote! {
                #(#[doc = #comments])*
                #derive_msg
                #(#attrs)*
                pub enum #rust_name<#lifetime> {
                    #variants

                    #default_variant_attr
                    None
                }
            })
        } else {
            let msg_fields = self
                .fields
                .iter()
                .map(|f| {
                    f.generate_field(ctx)
                        .map_err(|e| field_error(&ctx.pkg, self.name, f.name, &e))
                })
                .try_into_tokens()?;
            let oneof_fields = self
                .oneofs
                .iter()
                .map(|oneof| oneof.generate_field(ctx, &msg_mod_name));
            let unknown_field = if let Some(unknown) = &self.unknown {
                let handler = &unknown.handler;
                let field_attr = &unknown.field_attrs;
                quote! { #[doc = " Handler for unknown fields on the wire"] #(#field_attr)* pub _unknown: #handler, }
            } else {
                quote! {}
            };
            let hazzer_field_attr = self.hazzer.iter().map(|h| &h.field_attrs);

            Ok(quote! {
                #(#[doc = #comments])*
                #derive_msg
                #(#attrs)*
                pub struct #rust_name<#lifetime> {
                    #msg_fields
                    #(#oneof_fields)*
                    #(#[doc = " Tracks presence of optional and message fields"] #(#hazzer_field_attr)* pub _has: #msg_mod_name::_Hazzer,)*
                    #unknown_field
                }
            })
        }
    }

    pub(crate) fn generate_default_impl(&self, ctx: &Context<'proto>) -> io::Result<TokenStream> {
        if !self.impl_default {
            return Ok(quote! {});
        }
        assert!(
            !self.as_oneof_enum,
            "should not generate default impl if generating enum message"
        );

        let mut field_defaults = TokenStream::new();
        for f in &self.fields {
            // Skip delegate fields when generating defaults
            if !matches!(f.ftype, FieldType::Custom(CustomField::Delegate(_))) {
                let name = &f.san_rust_name;
                let default = f
                    .generate_default(ctx)
                    .map_err(|e| field_error(&ctx.pkg, self.name, f.name, &e))?;
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
        let hazzer_default = self
            .hazzer
            .as_ref()
            .map(|_| quote! { _has: ::core::default::Default::default(), });
        let unknown_default = self
            .unknown
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
        assert!(
            !self.as_oneof_enum,
            "should not generate PartialEq impl if generating enum message"
        );

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

    pub(crate) fn generate_impl(&self, ctx: &Context<'proto>) -> Result<TokenStream, io::Error> {
        if self.as_oneof_enum {
            return Ok(quote! {});
        }

        let accessors = self
            .fields
            .iter()
            .map(|f| {
                f.generate_accessors(ctx)
                    .map_err(|e| field_error(&ctx.pkg, self.name, &f.name, &e))
            })
            .try_into_tokens()?;
        let name = &self.rust_name;
        let lifetime = &self.lifetime;
        Ok(quote! {
            impl<#lifetime> #name<#lifetime> {
                #accessors
            }
        })
    }

    pub(crate) fn generate_decode_trait(
        &self,
        ctx: &Context<'proto>,
    ) -> Result<TokenStream, io::Error> {
        let name = &self.rust_name;
        let lifetime = &self.lifetime;
        let tag = Ident::new("tag", Span::call_site());
        let decoder = Ident::new("decoder", Span::call_site());
        let mod_name = resolve_path_elem(self.name, true);

        let branches = if self.as_oneof_enum {
            let OneofType::Enum { fields, .. } = &self.oneofs[0].otype else {
                unreachable!("shouldn't generate enum with custom oneof")
            };
            let variant_branches = fields
                .iter()
                .map(|f| {
                    f.generate_decode_branch_in_enum_msg(&decoder, ctx)
                        .map_err(|e| field_error(&ctx.pkg, self.name, &f.name, &e))
                })
                .try_into_tokens()?;
            quote! { #variant_branches }
        } else {
            let field_branches = self
                .fields
                .iter()
                .map(|f| {
                    f.generate_decode_branch(ctx, &tag, &decoder)
                        .map_err(|e| field_error(&ctx.pkg, self.name, &f.name, &e))
                })
                .try_into_tokens()?;
            let oneof_branches = self
                .oneofs
                .iter()
                .map(|o| {
                    o.generate_decode_branches(ctx, &mod_name, &tag, &decoder)
                        .map_err(|e| field_error(&ctx.pkg, self.name, &o.name, &e))
                })
                .try_into_tokens()?;

            quote! {
                #field_branches
                #oneof_branches
            }
        };

        // Ignore unknown handler if the message is an enum
        let unknown_branch = if self.unknown.is_some() && !self.as_oneof_enum {
            // If the unknown handler can't handle a field, skip it
            quote! { if !self._unknown.decode_field(#tag, #decoder)? { #decoder.skip_wire_value(#tag.wire_type())?; } }
        } else {
            quote! { #decoder.skip_wire_value(#tag.wire_type())?; }
        };

        let tok = quote! {
            impl<#lifetime> ::micropb::MessageDecode for #name<#lifetime> {
                fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                    &mut self,
                    #decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                    len: usize,
                ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>>
                {
                    use ::micropb::{PbBytes, PbString, PbVec, PbMap, FieldDecode};

                    let before = #decoder.bytes_read();
                    while #decoder.bytes_read() - before < len {
                        let #tag = #decoder.decode_tag()?;
                        match #tag.field_num() {
                            0 => return Err(::micropb::DecodeError::ZeroField),
                            #branches
                            _ => { #unknown_branch }
                        }
                    }
                    Ok(())
                }
            }
        };
        Ok(tok)
    }

    fn generate_encode_func(&self, ctx: &Context<'proto>, func_type: &EncodeFunc) -> TokenStream {
        let mod_name = resolve_path_elem(self.name, true);

        if self.as_oneof_enum {
            let OneofType::Enum { fields, .. } = &self.oneofs[0].otype else {
                unreachable!("shouldn't generate enum with custom oneof")
            };
            let variant_branches = fields
                .iter()
                .map(|f| f.generate_encode_branch(&quote! {Self}, ctx, func_type));

            quote! {
                match &self {
                    #(#variant_branches)*
                    Self::None => {}
                }
            }
        } else {
            let field_logic = self
                .fields
                .iter()
                .map(|f| f.generate_encode(ctx, func_type));
            let oneof_logic = self
                .oneofs
                .iter()
                .map(|o| o.generate_encode(ctx, &mod_name, func_type));

            let unknown_logic = if self.unknown.is_some() {
                match func_type {
                    EncodeFunc::Sizeof(size) => {
                        quote! { #size += self._unknown.compute_fields_size(); }
                    }
                    EncodeFunc::Encode(encoder) => {
                        quote! { self._unknown.encode_fields(#encoder)?; }
                    }
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
    }

    fn generate_max_size(&self, ctx: &Context<'proto>) -> TokenStream {
        if !ctx.params.calculate_max_size {
            return quote! { const MAX_SIZE: ::core::result::Result<usize, &'static str> = ::core::result::Result::Err("calculate_max_size disabled"); };
        }

        let field_sizes = self
            .fields
            .iter()
            .map(|f| f.generate_max_size(ctx, self.name));
        let oneof_sizes = self
            .oneofs
            .iter()
            .map(|o| o.generate_max_size(ctx, self.name));
        let unknown_size = if self.as_oneof_enum {
            // Ignore unknown field if generating an enum msg
            None
        } else {
            self.unknown
                .as_ref()
                .map(|u| &u.handler)
                .map(|handler| quote! { <#handler as ::micropb::field::FieldEncode>::MAX_SIZE })
        };
        let sizes = field_sizes.chain(oneof_sizes).chain(unknown_size);

        quote! {
            const MAX_SIZE: ::core::result::Result<usize, &'static str> = 'msg: {
                let mut max_size = 0;
                #(
                    match #sizes {
                        ::core::result::Result::Ok(size) => {
                            max_size += size;
                        }
                        ::core::result::Result::Err(err) => break 'msg (::core::result::Result::<usize, _>::Err(err)),
                    }
                )*
                ::core::result::Result::Ok(max_size)
            };
        }
    }

    pub(crate) fn generate_encode_trait(&self, ctx: &Context<'proto>) -> TokenStream {
        let name = &self.rust_name;
        let lifetime = &self.lifetime;
        let sizeof = self.generate_encode_func(
            ctx,
            &EncodeFunc::Sizeof(Ident::new("size", Span::call_site())),
        );
        let encode = self.generate_encode_func(
            ctx,
            &EncodeFunc::Encode(Ident::new("encoder", Span::call_site())),
        );
        let max_size_decl = self.generate_max_size(ctx);

        quote! {
            impl<#lifetime> ::micropb::MessageEncode for #name<#lifetime> {
                #max_size_decl

                fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                    &self,
                    encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
                ) -> Result<(), IMPL_MICROPB_WRITE::Error>
                {
                    use ::micropb::{PbMap, FieldEncode};
                    #encode
                    Ok(())
                }

                fn compute_size(&self) -> usize {
                    use ::micropb::{PbMap, FieldEncode};
                    let mut size = 0;
                    #sizeof
                    size
                }
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn make_test_msg(name: &str) -> Message<'_> {
    Message {
        name,
        rust_name: Ident::new(name, Span::call_site()),
        oneofs: vec![],
        fields: vec![],
        derive_dbg: true,
        impl_default: true,
        impl_partial_eq: true,
        derive_clone: true,
        is_copy: false,
        attrs: vec![],
        unknown: None,
        lifetime: None,
        as_oneof_enum: false,
        hazzer: None,
        comments: None,

        message_edges: vec![],
        parent_edges: vec![],
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        config::{Config, IntSize, OptionalRepr, parse_attributes},
        descriptor::{
            FieldDescriptorProto,
            FieldDescriptorProto_::{Label, Type},
            FieldOptions, MessageOptions, OneofDescriptorProto,
        },
        generator::{
            Syntax,
            field::{FieldType, make_test_field},
            make_ctx,
            oneof::{make_test_oneof, make_test_oneof_field},
            type_spec::{PbInt, TypeSpec},
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
        let ctx = make_ctx();
        assert!(
            Message::from_proto(&proto, &ctx, &msg_conf, None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn from_proto_skip_fields() {
        let ctx = make_ctx();
        let proto = test_msg_proto();
        let empty_msg = make_test_msg("Message");
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
            Message::from_proto(&proto, &ctx, &msg_conf, None)
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
            Message::from_proto(&proto, &ctx, &msg_conf, None)
                .unwrap()
                .unwrap(),
            empty_msg
        );
    }

    #[test]
    fn from_proto() {
        let ctx = make_ctx();
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

        let mut expected = make_test_msg("Message");
        expected.oneofs = vec![Oneof {
            name: "oneof",
            san_rust_name: Ident::new_raw("oneof", Span::call_site()),
            otype: OneofType::Enum {
                type_name: Ident::new("Oneof", Span::call_site()),
                fields: vec![
                    make_test_oneof_field(
                        2,
                        "oneof_field",
                        false,
                        TypeSpec::Int(PbInt::Sint32, IntSize::S8),
                    ),
                    make_test_oneof_field(4, "oneof_field2", true, TypeSpec::Float),
                ],
            },
            field_attrs: vec![],
            // Overrides the type attrs of the message
            type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
            boxed: false,
            // Inherits the no_debug_derive setting of the message
            derive_dbg: false,
            derive_partial_eq: true,
            derive_clone: true,
            lifetime: None,
            idx: 0,
            comments: None,
        }];
        expected.fields = vec![
            make_test_field(
                1,
                "bool_field",
                true,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
            ),
            make_test_field(
                3,
                "map_field",
                false,
                FieldType::Map {
                    key: TypeSpec::Int(PbInt::Int64, IntSize::S16),
                    val: TypeSpec::Int(PbInt::Uint64, IntSize::S16),
                    typestr: "Map".to_owned(),
                    max_len: None,
                },
            ),
        ];
        expected.derive_dbg = false;
        expected.impl_default = false;
        expected.attrs = parse_attributes("#[derive(Self)]").unwrap();
        expected.unknown = Some(Unknown {
            handler: syn::parse_str("UnknownType").unwrap(),
            field_attrs: vec![],
        });

        assert_eq!(
            Message::from_proto(&proto, &ctx, &msg_conf, None)
                .unwrap()
                .unwrap(),
            expected
        )
    }

    #[test]
    fn synthetic_oneof() {
        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto3;

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

        let mut expected = make_test_msg("Msg");
        expected.fields = vec![make_test_field(
            1,
            "opt",
            false,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
        )];
        expected.hazzer = Some(Hazzer {
            type_attrs: vec![],
            field_attrs: vec![],
        });

        assert_eq!(
            Message::from_proto(&proto, &ctx, &msg_conf, None)
                .unwrap()
                .unwrap(),
            expected
        )
    }

    #[test]
    fn message_fields() {
        let mut proto = DescriptorProto::default();
        proto.set_name("Message".to_owned());
        proto.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(1);
            f.set_name("internal".to_owned());
            f.set_type(Type::Message);
            f.set_type_name(".Internal".to_owned());
            f
        });
        proto.field.push({
            let mut f = FieldDescriptorProto::default();
            f.set_number(2);
            f.set_name("external".to_owned());
            f.set_type(Type::Message);
            f.set_type_name(".External".to_owned());
            f
        });

        let mut ctx = make_ctx();
        ctx.params.extern_paths.insert(
            ".External".to_owned(),
            syn::parse_str("ex::Ternal").unwrap(),
        );

        let config = Box::new(Config::new().optional_repr(OptionalRepr::Option));
        let msg_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };

        let mut expected = make_test_msg("Message");
        expected.fields = vec![
            make_test_field(
                1,
                "internal",
                false,
                FieldType::Optional(TypeSpec::Message(".Internal"), OptionalRepr::Option),
            ),
            make_test_field(
                2,
                "external",
                false,
                FieldType::Optional(TypeSpec::Message(".External"), OptionalRepr::Option),
            ),
        ];
        // Only the first field should be an edge, since the second field is external
        expected.message_edges = vec![(Position::Field(0), ".Internal")];

        assert_eq!(
            Message::from_proto(&proto, &ctx, &msg_conf, None)
                .unwrap()
                .unwrap(),
            expected
        )
    }

    #[test]
    fn is_copy() {
        let ctx = make_ctx();

        // Not copy (boxed oneof)
        let mut msg = make_test_msg("Msg");
        msg.oneofs.push(make_test_oneof("empty", true));
        assert!(!msg.is_copy(&ctx));

        // Not copy (boxed field)
        let mut msg = make_test_msg("Msg");
        msg.fields.push(make_test_field(
            1,
            "good",
            false,
            FieldType::Single(TypeSpec::Bool),
        ));
        msg.fields.push(make_test_field(
            2,
            "bad",
            true,
            FieldType::Single(TypeSpec::Bool),
        ));
        assert!(!msg.is_copy(&ctx));

        // Not copy (boxed oneof field)
        let mut msg = make_test_msg("Msg");
        msg.oneofs.push(make_test_oneof("content", false));
        msg.oneofs[0]
            .otype
            .fields_mut()
            .unwrap()
            .push(make_test_oneof_field(1, "bad", true, TypeSpec::Bool));
        assert!(!msg.is_copy(&ctx));

        // Not copy (custom field)
        let mut msg = make_test_msg("Msg");
        msg.fields.push(make_test_field(
            1,
            "custom",
            false,
            FieldType::Custom(CustomField::Type(syn::parse_str("Custom").unwrap())),
        ));
        assert!(!msg.is_copy(&ctx));

        // Copy
        let mut msg = make_test_msg("Msg");
        msg.oneofs.push(make_test_oneof("content", false));
        msg.oneofs[0]
            .otype
            .fields_mut()
            .unwrap()
            .push(make_test_oneof_field(1, "good", false, TypeSpec::Bool));
        msg.fields.push(make_test_field(
            2,
            "e",
            false,
            FieldType::Single(TypeSpec::Enum(".Enum")),
        ));
        assert!(msg.is_copy(&ctx));
    }
}
