use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Lifetime};

use crate::{
    descriptor::{FieldDescriptorProto, OneofDescriptorProto},
    generator::{
        Context, CurrentConfig, EncodeFunc, derive_msg_attr, field::CustomField, field_error,
        field_error_str, location::get_comments, message::Message, sanitized_ident,
        type_spec::TypeSpec,
    },
    utils::{TryIntoTokens, find_lifetime_from_type},
};

use super::location::{self, CommentNode, Comments};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct OneofField<'proto> {
    pub(crate) num: u32,
    pub(crate) tspec: TypeSpec<'proto>,
    #[allow(unused)]
    /// Protobuf name
    pub(crate) name: &'proto str,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) rust_name: Ident,
    pub(crate) boxed: bool,
    pub(crate) max_size_override: Option<Result<usize, String>>,
    pub(crate) attrs: Vec<syn::Attribute>,
    comments: Option<&'proto Comments>,
}

impl<'proto> OneofField<'proto> {
    fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        !self.boxed && self.tspec.is_copy(ctx)
    }

    pub(crate) fn from_proto(
        proto: &'proto FieldDescriptorProto,
        field_conf: &CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
    ) -> Result<Option<Self>, String> {
        if field_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let name = &proto.name;
        // Oneof fields have camelcased variant names
        let rust_name = sanitized_ident(
            &field_conf
                .config
                .rust_field_name(name)?
                .0
                .to_case(Case::Pascal),
        );
        let num = proto.number as u32;
        let tspec = TypeSpec::from_proto(proto, field_conf)?;
        let attrs = field_conf.config.field_attr_parsed()?;

        Ok(Some(OneofField {
            num,
            tspec,
            name,
            rust_name,
            max_size_override: field_conf.config.encoded_max_size.map(Ok),
            boxed: field_conf.config.boxed.unwrap_or(false),
            attrs,
            comments: location::get_comments(comment_node),
        }))
    }

    pub(crate) fn generate_field(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        let typ = ctx.wrapped_type(self.tspec.generate_rust_type(ctx)?, self.boxed, false);
        let name = &self.rust_name;
        let attrs = &self.attrs;
        let comments = self.comments.map(Comments::lines).into_iter().flatten();
        Ok(quote! { #(#[doc = #comments])* #(#attrs)* #name(#typ), })
    }

    pub(crate) fn generate_cache_field(&self, ctx: &Context<'proto>) -> TokenStream {
        if let Some(typ) = self.tspec.generate_cache_type(ctx) {
            let name = &self.rust_name;
            quote! { #name(#typ), }
        } else {
            quote! {}
        }
    }

    fn generate_decode_branch(
        &self,
        oneof_name: &Ident,
        oneof_type: &TokenStream,
        oneof_boxed: bool,
        ctx: &Context<'proto>,
        decoder: &Ident,
    ) -> Result<TokenStream, String> {
        let fnum = self.num;
        let mut_ref = Ident::new("mut_ref", Span::call_site());
        let variant_name = &self.rust_name;
        let extra_deref_of = oneof_boxed.then(|| quote! { * });
        let extra_deref_var = self.boxed.then(|| quote! { * });

        let decode_stmts = self
            .tspec
            .generate_decode_mut(ctx, false, decoder, &mut_ref)?;
        let value = ctx.wrapped_value(
            quote! { #oneof_type::#variant_name(::core::default::Default::default()) },
            oneof_boxed,
            true,
        );
        let tok = quote! {
            #fnum => {
                let #mut_ref = loop {
                    if let ::core::option::Option::Some(variant) = &mut self.#oneof_name {
                        if let #oneof_type::#variant_name(variant) = &mut #extra_deref_of *variant {
                            break &mut #extra_deref_var *variant;
                        }
                    }
                    self.#oneof_name = #value;
                };
                #decode_stmts;
            }
        };
        Ok(tok)
    }

    pub(crate) fn generate_decode_branch_in_enum_msg(
        &self,
        decoder: &Ident,
        ctx: &Context<'proto>,
    ) -> Result<TokenStream, String> {
        let fnum = self.num;
        let mut_ref = Ident::new("mut_ref", Span::call_site());
        let variant_name = &self.rust_name;
        let extra_deref_var = self.boxed.then(|| quote! { * });

        let decode_stmts = self
            .tspec
            .generate_decode_mut(ctx, false, decoder, &mut_ref)?;
        let tok = quote! {
            #fnum => {
                let #mut_ref = loop {
                    if let Self::#variant_name(variant) = self {
                        break &mut #extra_deref_var *variant;
                    }
                    *self = Self::#variant_name(::core::default::Default::default());
                };
                #decode_stmts;
            }
        };
        Ok(tok)
    }

    pub(crate) fn generate_encode_branch(
        &self,
        ctx: &Context<'proto>,
        oneof_type: &TokenStream,
        oneof_name: &Ident,
        cache_enum_type: &TokenStream,
        func_type: &EncodeFunc,
    ) -> TokenStream {
        let val_ref = Ident::new("val_ref", Span::call_site());
        let variant_name = &self.rust_name;
        let extra_deref = self.boxed.then(|| quote! { * });
        let wire_type = self.tspec.wire_type();
        let tag = micropb::Tag::from_parts(self.num, wire_type);
        let tag_val = tag.varint();
        let tag_len = ::micropb::size::sizeof_tag(tag);

        let stmts = match &func_type {
            EncodeFunc::Sizeof(size) => {
                let sizeof_expr = self.tspec.generate_sizeof(ctx, &val_ref);
                quote! { #size += #tag_len + #sizeof_expr; }
            }
            EncodeFunc::PopulateCache(cache) => {
                if self.tspec.is_cached(ctx) {
                    quote! {
                        let subcache = #val_ref.populate_cache();
                        #cache._size += #tag_len + ::micropb::size::sizeof_len_record(subcache._size);
                        #cache.#oneof_name = #cache_enum_type::#variant_name(subcache);
                    }
                } else {
                    let sizeof_expr = self.tspec.generate_sizeof(ctx, &val_ref);
                    quote! { #cache._size += #tag_len + #sizeof_expr; }
                }
            }

            EncodeFunc::Encode(encoder) => {
                let encode_expr = self.tspec.generate_encode_expr(ctx, encoder, &val_ref);
                quote! {
                    #encoder.encode_varint32(#tag_val)?;
                    #encode_expr?;
                }
            }
            EncodeFunc::EncodeCached(encoder, cache) => {
                let encode_expr = if self.tspec.is_cached(ctx) {
                    quote! {
                        if let #cache_enum_type::#variant_name(subcache) = &#cache.#oneof_name {
                            #val_ref.encode_len_delimited_cached(#encoder, subcache)
                        } else {
                            core::unreachable!("unexpected cache variant")
                        }
                    }
                } else {
                    self.tspec.generate_encode_expr(ctx, encoder, &val_ref)
                };
                quote! {
                    #encoder.encode_varint32(#tag_val)?;
                    #encode_expr?;
                }
            }
        };

        quote! {
            #oneof_type::#variant_name(#val_ref) => {
                let #val_ref = &* #extra_deref #val_ref;
                #stmts
            }
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum OneofType<'a> {
    Enum {
        type_name: Ident,
        fields: Vec<OneofField<'a>>,
    },
    Custom {
        field: CustomField,
        nums: Vec<i32>,
    },
}

impl<'proto> OneofType<'proto> {
    pub(crate) fn find_lifetime(&self) -> Option<Lifetime> {
        match self {
            OneofType::Custom {
                field: CustomField::Type(ty),
                ..
            } => find_lifetime_from_type(ty).cloned(),

            OneofType::Custom {
                field: CustomField::Delegate(..),
                ..
            } => None,

            OneofType::Enum { fields, .. } => fields.iter().find_map(|of| of.tspec.find_lifetime()),
        }
    }

    pub(crate) fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        match self {
            OneofType::Custom { .. } => false,
            OneofType::Enum { fields, .. } => fields.iter().all(|of| of.is_copy(ctx)),
        }
    }

    pub(crate) fn fields_mut<'b>(&'b mut self) -> Option<&'b mut Vec<OneofField<'proto>>> {
        if let OneofType::Enum { fields, .. } = self {
            Some(fields)
        } else {
            None
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Oneof<'proto> {
    /// Protobuf name
    #[allow(unused)]
    pub(crate) name: &'proto str,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) san_rust_name: Ident,
    pub(crate) otype: OneofType<'proto>,
    pub(crate) field_attrs: Vec<syn::Attribute>,
    pub(crate) type_attrs: Vec<syn::Attribute>,
    pub(crate) boxed: bool,
    pub(crate) derive_dbg: bool,
    pub(crate) derive_partial_eq: bool,
    pub(crate) derive_clone: bool,
    pub(crate) lifetime: Option<Lifetime>,
    pub(crate) idx: usize,
    pub(crate) comments: Option<&'proto Comments>,
}

impl<'proto> Oneof<'proto> {
    pub(crate) fn from_proto(
        proto: &'proto OneofDescriptorProto,
        oneof_conf: CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
        idx: usize,
    ) -> Result<Option<Self>, String> {
        if oneof_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let name = &proto.name;
        let (rust_name, raw_rust_name) = oneof_conf.config.rust_field_name(name)?;
        let otype = match oneof_conf.config.custom_field_parsed()? {
            Some(custom) => OneofType::Custom {
                field: custom,
                nums: vec![],
            },
            None => OneofType::Enum {
                // Use sanitized pascal case for enum type
                type_name: sanitized_ident(&rust_name.to_case(Case::Pascal)),
                fields: vec![],
            },
        };
        let field_attrs = oneof_conf.config.field_attr_parsed()?;
        let type_attrs = oneof_conf.config.type_attr_parsed()?;
        let boxed = oneof_conf.config.boxed.unwrap_or(false);

        Ok(Some(Oneof {
            name,
            san_rust_name: raw_rust_name,
            idx,
            otype,
            boxed,
            derive_dbg: oneof_conf.derive_dbg(),
            derive_partial_eq: oneof_conf.derive_partial_eq(),
            derive_clone: oneof_conf.derive_clone(),
            lifetime: None,
            field_attrs,
            type_attrs,
            comments: get_comments(comment_node),
        }))
    }

    /// Find lifetime and set the oneof's lifetime field
    pub(crate) fn find_lifetime(&mut self) -> Option<&Lifetime> {
        self.lifetime = self.otype.find_lifetime();
        self.lifetime.as_ref()
    }

    pub(crate) fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        !self.boxed && self.otype.is_copy(ctx)
    }

    pub(crate) fn generate_decl(
        &self,
        ctx: &Context<'proto>,
        msg: &Message<'proto>,
    ) -> std::io::Result<TokenStream> {
        if let OneofType::Enum { type_name, fields } = &self.otype {
            assert!(!fields.is_empty(), "empty enums should have been filtered");
            let fields = fields
                .iter()
                .map(|f| {
                    f.generate_field(ctx)
                        .map_err(|e| field_error(&ctx.pkg, msg.name, f.name, &e))
                })
                .try_into_tokens()?;
            let derive_msg = derive_msg_attr(
                self.derive_dbg,
                false,
                self.derive_partial_eq,
                self.derive_clone,
                // Only derive Copy if the message type is Copy
                msg.is_copy,
            );
            let attrs = &self.type_attrs;
            let lifetime = &self.lifetime;
            let comments = self.comments.map(Comments::lines).into_iter().flatten();

            Ok(quote! {
                #(#[doc = #comments])*
                #derive_msg
                #(#attrs)*
                pub enum #type_name<#lifetime> {
                    #fields
                }
            })
        } else {
            Ok(quote! {})
        }
    }

    pub(crate) fn generate_field(
        &self,
        ctx: &Context<'proto>,
        msg_mod_name: &Ident,
    ) -> TokenStream {
        let name = &self.san_rust_name;
        let oneof_type = match &self.otype {
            OneofType::Enum { type_name, .. } => {
                let lifetime = &self.lifetime;
                ctx.wrapped_type(
                    quote! { #msg_mod_name::#type_name<#lifetime> },
                    self.boxed,
                    true,
                )
            }
            // Don't explicitly write lifetime for custom types, since it's already included
            OneofType::Custom {
                field: CustomField::Type(type_path),
                ..
            } => quote! { #type_path },
            OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } => return quote! {},
        };
        let attrs = &self.field_attrs;
        let comments = self.comments.map(Comments::lines).into_iter().flatten();
        quote! { #(#[doc = #comments])* #(#attrs)* pub #name: #oneof_type, }
    }

    pub(crate) fn generate_cache_decl(&self, ctx: &Context<'proto>) -> TokenStream {
        if let OneofType::Enum { type_name, fields } = &self.otype {
            let fields = fields.iter().map(|f| f.generate_cache_field(ctx));
            let cache_name = oneof_cache_name(type_name);
            quote! {
                #[derive(Default)]
                pub enum #cache_name {
                    #(#fields)*
                    #[default]
                    None
                }
            }
        } else {
            quote! {}
        }
    }

    pub(crate) fn generate_cache_field(&self, msg_mod_name: &Ident) -> TokenStream {
        let name = &self.san_rust_name;
        let oneof_type = match &self.otype {
            OneofType::Enum { type_name, .. } => {
                let cache_name = oneof_cache_name(type_name);
                quote! { #msg_mod_name::#cache_name }
            }
            OneofType::Custom { .. } => return quote! {},
        };
        quote! { pub #name: #oneof_type, }
    }

    pub(crate) fn generate_decode_branches(
        &self,
        ctx: &Context<'proto>,
        msg_mod_name: &Ident,
        tag: &Ident,
        decoder: &Ident,
    ) -> Result<TokenStream, String> {
        let name = &self.san_rust_name;
        let tok = match &self.otype {
            OneofType::Enum { fields, type_name } => {
                let oneof_type = quote! { #msg_mod_name::#type_name };
                let branches = fields
                    .iter()
                    .map(|f| f.generate_decode_branch(name, &oneof_type, self.boxed, ctx, decoder))
                    .try_into_tokens()?;
                quote! { #branches }
            }
            OneofType::Custom {
                field: CustomField::Type(_),
                nums,
            } => {
                let nums = nums.iter().map(|n| Literal::i32_unsuffixed(*n));
                quote! {
                    #(#nums)|* => { if !self.#name.decode_field(#tag, #decoder)? { return Err(::micropb::DecodeError::CustomField) } }
                }
            }
            OneofType::Custom {
                field: CustomField::Delegate(field),
                nums,
            } => {
                let nums = nums.iter().map(|n| Literal::i32_unsuffixed(*n));
                quote! {
                    #(#nums)|* => { if !self.#field.decode_field(#tag, #decoder)? { return Err(::micropb::DecodeError::CustomField) } }
                }
            }
        };
        Ok(tok)
    }

    pub(crate) fn generate_encode(
        &self,
        ctx: &Context<'proto>,
        msg_mod_name: &Ident,
        func_type: &EncodeFunc,
    ) -> TokenStream {
        let name = &self.san_rust_name;
        match &self.otype {
            OneofType::Enum { type_name, fields } => {
                let oneof_type = quote! { #msg_mod_name::#type_name };
                let cache_name = oneof_cache_name(type_name);
                let cache_enum_type = quote! { #msg_mod_name::#cache_name};
                let extra_deref = self.boxed.then(|| quote! { * });
                let branches = fields.iter().map(|f| {
                    f.generate_encode_branch(
                        ctx,
                        &oneof_type,
                        &self.san_rust_name,
                        &cache_enum_type,
                        func_type,
                    )
                });
                quote! {
                    if let Some(oneof) = & self.#name {
                        match &#extra_deref *oneof {
                            #(#branches)*
                        }
                    }
                }
            }

            OneofType::Custom {
                field: CustomField::Type(_),
                ..
            } => match &func_type {
                EncodeFunc::Sizeof(size) => quote! { #size += self.#name.compute_fields_size(); },
                EncodeFunc::PopulateCache(cache) => {
                    quote! { #cache._size += self.#name.compute_fields_size(); }
                }
                EncodeFunc::Encode(encoder) | EncodeFunc::EncodeCached(encoder, _) => {
                    quote! { self.#name.encode_fields(#encoder)?; }
                }
            },

            OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } => quote! {},
        }
    }

    pub(crate) fn generate_max_size(
        &self,
        ctx: &Context<'proto>,
        msg_name: &'proto str,
    ) -> TokenStream {
        match &self.otype {
            OneofType::Custom {
                field: CustomField::Type(custom),
                ..
            } => {
                quote! { <#custom as ::micropb::field::FieldEncode>::MAX_SIZE }
            }
            OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } => quote! { ::core::result::Result::Ok(0) },

            OneofType::Enum { fields, .. } => {
                let variant_sizes = fields.iter().map(|f| {
                    if let Some(max_size) = &f.max_size_override {
                        match max_size {
                            Ok(size) => quote! { ::core::result::Result::Ok(#size) },
                            Err(err) => {
                                let err = field_error_str(&ctx.pkg, msg_name, self.name, err);
                                quote! { ::core::result::Result::<usize, _>::Err(#err) }
                            }
                        }
                    } else {
                        let wire_type = f.tspec.wire_type();
                        let tag = micropb::Tag::from_parts(f.num, wire_type);
                        let tag_len = ::micropb::size::sizeof_tag(tag);
                        let size = f.tspec.generate_max_size(ctx, msg_name, f.name);
                        quote! { ::micropb::const_map!(#size, |size| size + #tag_len) }
                    }
                });

                quote! {'oneof: {
                    let mut max_size = 0;
                    #(
                        match #variant_sizes {
                            ::core::result::Result::Ok(size) => if size > max_size {
                                max_size = size;
                            }
                            ::core::result::Result::Err(err) => break 'oneof (::core::result::Result::<usize, _>::Err(err)),
                        }
                    )*
                    ::core::result::Result::Ok(max_size)
                }}
            }
        }
    }
}

pub(crate) fn oneof_cache_name(type_name: &Ident) -> Ident {
    format_ident!("_{}Cache", type_name)
}

#[cfg(test)]
pub(crate) fn make_test_oneof_field<'a>(
    num: u32,
    name: &'a str,
    boxed: bool,
    tspec: TypeSpec<'a>,
) -> OneofField<'a> {
    OneofField {
        num,
        name,
        tspec,
        rust_name: Ident::new(&name.to_case(Case::Pascal), Span::call_site()),
        boxed,
        max_size_override: None,
        attrs: vec![],
        comments: None,
    }
}

#[cfg(test)]
pub(crate) fn make_test_oneof(name: &str, boxed: bool) -> Oneof<'_> {
    Oneof {
        name,
        san_rust_name: Ident::new_raw(name, Span::call_site()),
        boxed,
        otype: OneofType::Enum {
            type_name: Ident::new(&name.to_case(Case::Pascal), Span::call_site()),
            fields: vec![],
        },
        field_attrs: vec![],
        type_attrs: vec![],
        derive_dbg: true,
        derive_clone: true,
        derive_partial_eq: true,
        lifetime: None,
        idx: 0, // Not used at all
        comments: None,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::descriptor::FieldDescriptorProto_::Type;

    use crate::config::{Config, parse_attributes};
    use crate::generator::make_ctx;
    use crate::generator::message::make_test_msg;

    use super::*;

    fn field_proto(num: u32, name: &str) -> FieldDescriptorProto {
        let mut f = FieldDescriptorProto::default();
        f.set_name(name.to_owned());
        f.set_number(num as i32);
        f.set_type(Type::Bool);
        f
    }

    #[test]
    fn from_proto_skipped() {
        let config = Box::new(Config::new().skip(true));
        let oneof_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field");
        assert!(
            OneofField::from_proto(&field, &oneof_conf, None)
                .unwrap()
                .is_none()
        );
        let oneof = OneofDescriptorProto::default();
        assert!(
            Oneof::from_proto(&oneof, oneof_conf, None, 0,)
                .unwrap()
                .is_none()
        );
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
            OneofField::from_proto(&field, &field_conf, None)
                .unwrap()
                .unwrap(),
            OneofField {
                num: 1,
                tspec: TypeSpec::Bool,
                name: "field",
                rust_name: Ident::new("Field", Span::call_site()),
                boxed: false,
                max_size_override: None,
                attrs: vec![],
                comments: None
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
            OneofField::from_proto(&field, &field_conf, None)
                .unwrap()
                .unwrap(),
            OneofField {
                num: 1,
                tspec: TypeSpec::Bool,
                name: "field",
                rust_name: Ident::new("Renamed", Span::call_site()),
                max_size_override: None,
                boxed: true,
                attrs: parse_attributes("#[attr]").unwrap(),
                comments: None
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
        let mut oneof = OneofDescriptorProto::default();
        oneof.set_name("oneof".to_owned());
        assert_eq!(
            Oneof::from_proto(&oneof, oneof_conf, None, 0)
                .unwrap()
                .unwrap(),
            Oneof {
                name: "oneof",
                san_rust_name: Ident::new_raw("oneof", Span::call_site()),
                otype: OneofType::Enum {
                    type_name: Ident::new("Oneof", Span::call_site()),
                    fields: vec![]
                },
                field_attrs: vec![],
                type_attrs: vec![],
                boxed: false,
                derive_dbg: true,
                derive_partial_eq: true,
                derive_clone: true,
                lifetime: None,
                idx: 0,
                comments: None
            }
        );

        config.field_attributes = Some("#[attr]".to_owned());
        config.type_attributes = Some("#[derive(Eq)]".to_owned());
        config.no_debug_impl = Some(true);
        config.rename_field = Some("renamed_oneof".to_owned());
        let oneof_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Oneof::from_proto(&oneof, oneof_conf, None, 0)
                .unwrap()
                .unwrap(),
            Oneof {
                name: "oneof",
                san_rust_name: Ident::new("renamed_oneof", Span::call_site()),
                otype: OneofType::Enum {
                    type_name: Ident::new("RenamedOneof", Span::call_site()),
                    fields: vec![]
                },
                field_attrs: parse_attributes("#[attr]").unwrap(),
                type_attrs: parse_attributes("#[derive(Eq)]").unwrap(),
                boxed: false,
                derive_dbg: false,
                derive_partial_eq: true,
                derive_clone: true,
                lifetime: None,
                idx: 0,
                comments: None
            }
        );
    }

    #[test]
    fn oneof_custom() {
        let ctx = make_ctx();
        let msg = make_test_msg("Unused");
        let oneof = Oneof {
            name: "oneof",
            san_rust_name: Ident::new_raw("oneof", Span::call_site()),
            otype: OneofType::Custom {
                field: CustomField::Type(syn::parse_str("Custom<f32>").unwrap()),
                nums: vec![1],
            },
            field_attrs: vec![],
            type_attrs: vec![],
            boxed: false,
            derive_dbg: true,
            derive_partial_eq: true,
            derive_clone: true,
            lifetime: None,
            idx: 0,
            comments: None,
        };
        assert!(oneof.generate_decl(&ctx, &msg).unwrap().is_empty());
        assert_eq!(
            oneof
                .generate_field(&ctx, &Ident::new("Msg", Span::call_site()))
                .to_string(),
            quote! { pub r#oneof: Custom<f32>, }.to_string()
        );

        let oneof = Oneof {
            name: "oneof",
            san_rust_name: Ident::new_raw("oneof", Span::call_site()),
            otype: OneofType::Custom {
                field: CustomField::Delegate(syn::parse_str("delegate").unwrap()),
                nums: vec![1],
            },
            field_attrs: vec![],
            type_attrs: vec![],
            boxed: false,
            derive_dbg: true,
            derive_partial_eq: true,
            derive_clone: true,
            lifetime: None,
            idx: 0,
            comments: None,
        };
        assert!(oneof.generate_decl(&ctx, &msg).unwrap().is_empty());
        assert!(
            oneof
                .generate_field(&ctx, &Ident::new("Msg", Span::call_site()))
                .to_string()
                .is_empty()
        );
    }
}
