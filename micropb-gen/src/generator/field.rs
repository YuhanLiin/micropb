use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Lifetime};

use crate::config::{OptionalRepr, contains_len_param, map_type_parsed, vec_type_parsed};
use crate::descriptor::{
    DescriptorProto, FieldDescriptorProto,
    FieldDescriptorProto_::{Label, Type},
};
use crate::generator::{Context, field_error_str};
use crate::utils::{find_lifetime_from_str, find_lifetime_from_type};

use super::Syntax;
use super::location::{self, CommentNode, Comments};
use super::{CurrentConfig, EncodeFunc, type_spec::TypeSpec};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum CustomField {
    Type(syn::Type),
    Delegate(Ident),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum FieldType<'proto> {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec<'proto>,
        val: TypeSpec<'proto>,
        typestr: String,
        cache_vec_typestr: Option<String>,
        max_len: Option<u32>,
    },
    // Implicit presence
    Single(TypeSpec<'proto>),
    // Explicit presence
    Optional(TypeSpec<'proto>, OptionalRepr),
    Repeated {
        typ: TypeSpec<'proto>,
        packed: bool,
        typestr: String,
        cache_vec_typestr: String,
        max_len: Option<u32>,
    },
    Custom(CustomField),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Field<'proto> {
    pub(crate) num: u32,
    pub(crate) ftype: FieldType<'proto>,
    /// Protobuf name
    pub(crate) name: &'proto str,
    /// Non-sanitized Rust name after renaming, used for accessor names
    pub(crate) rust_name: String,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) san_rust_name: Ident,
    pub(crate) default: Option<&'proto str>,
    pub(crate) boxed: bool,
    pub(crate) max_size_override: Option<Result<usize, String>>,
    pub(crate) attrs: Vec<syn::Attribute>,
    no_accessors: bool,
    comments: Option<&'proto Comments>,
}

impl<'proto> Field<'proto> {
    pub(crate) fn is_option(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Option))
    }

    pub(crate) fn is_hazzer(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Hazzer))
    }

    pub(crate) fn message_name(&self) -> Option<&'proto str> {
        let typ = match &self.ftype {
            FieldType::Map { val, .. } => val,
            FieldType::Single(type_spec) => type_spec,
            FieldType::Optional(type_spec, _) => type_spec,
            FieldType::Repeated { typ, .. } => typ,
            FieldType::Custom(_) => return None,
        };
        if let TypeSpec::Message(name) = typ {
            Some(name)
        } else {
            None
        }
    }

    pub(crate) fn find_lifetime(&self) -> Option<Lifetime> {
        match &self.ftype {
            FieldType::Custom(CustomField::Type(ty)) => find_lifetime_from_type(ty).cloned(),
            FieldType::Single(tspec) | FieldType::Optional(tspec, _) => tspec.find_lifetime(),
            FieldType::Repeated {
                typ,
                typestr: type_path,
                ..
            } => find_lifetime_from_str(type_path).or_else(|| typ.find_lifetime()),
            FieldType::Map {
                key,
                val,
                typestr: type_path,
                ..
            } => find_lifetime_from_str(type_path)
                .or_else(|| key.find_lifetime())
                .or_else(|| val.find_lifetime()),
            _ => None,
        }
    }

    pub(crate) fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        !self.boxed
            && match &self.ftype {
                FieldType::Single(type_spec) | FieldType::Optional(type_spec, _) => {
                    type_spec.is_copy(ctx)
                }
                FieldType::Repeated { .. } | FieldType::Map { .. } | FieldType::Custom(_) => false,
            }
    }

    pub(crate) fn from_proto(
        proto: &'proto FieldDescriptorProto,
        field_conf: &CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
        ctx: &Context<'proto>,
        map_msg: Option<&'proto DescriptorProto>,
    ) -> Result<Option<Self>, String> {
        if field_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let num = proto.number as u32;
        let name = &proto.name;
        let (rust_name, san_rust_name) = field_conf.config.rust_field_name(name)?;
        let boxed = field_conf.config.boxed.unwrap_or(false);

        let ftype = match (
            field_conf.config.custom_field_parsed()?,
            map_msg,
            proto.label,
        ) {
            (Some(t), _, _) => FieldType::Custom(t),

            (None, Some(map_msg), _) => {
                let key = TypeSpec::from_proto(&map_msg.field[0], &field_conf.next_conf("key"))?;
                let val = TypeSpec::from_proto(&map_msg.field[1], &field_conf.next_conf("value"))?;
                let typestr = field_conf
                    .config
                    .map_type
                    .clone()
                    .ok_or_else(|| "map_type not configured".to_owned())?;
                let max_len = field_conf.config.max_len.filter(|_| contains_len_param(&typestr));
                let cache_typestr = if ctx.params.encode_cache {
                    field_conf.config.cache_vec_type.as_ref().or(field_conf.config.vec_type.as_ref()).cloned()
                } else {
                    None
                };
                FieldType::Map {
                    key,
                    val,
                    typestr,
                    cache_vec_typestr: cache_typestr,
                    max_len,
                }
            }

            (None, None, Label::Repeated) => {
                let typ = TypeSpec::from_proto(proto, &field_conf.next_conf("elem"))?;
                let typestr = field_conf
                    .config
                    .vec_type
                    .clone()
                    .ok_or_else(|| "vec_type not configured".to_owned())?;
                let max_len = field_conf.config.max_len.filter(|_| contains_len_param(&typestr));
                let cache_typestr = if ctx.params.encode_cache {
                    field_conf.config.cache_vec_type.as_ref().unwrap_or(&typestr).clone()
                } else {
                    String::new()
                };
                FieldType::Repeated {
                    typestr,
                    typ,
                    max_len,
                    cache_vec_typestr: cache_typestr,
                    packed: proto
                        .options()
                        .and_then(|opt| opt.packed().copied())
                        .unwrap_or(false),
                }
            }

            (None, None, Label::Required | Label::Optional)
                if ctx.syntax == Syntax::Proto2
                    || proto.proto3_optional
                    || proto.r#type == Type::Message =>
            {
                let repr = field_conf.config.optional_repr.unwrap_or(if boxed {
                    OptionalRepr::Option
                } else {
                    OptionalRepr::Hazzer
                });
                FieldType::Optional(TypeSpec::from_proto(proto, field_conf)?, repr)
            }

            (None, None, _) => FieldType::Single(TypeSpec::from_proto(proto, field_conf)?),
        };
        let encoded_max_size = field_conf.config.encoded_max_size;
        let attrs = field_conf.config.field_attr_parsed()?;
        let no_accessors = field_conf.config.no_accessors.unwrap_or(false);

        Ok(Some(Field {
            num,
            ftype,
            name,
            rust_name,
            san_rust_name,
            default: proto.default_value().map(String::as_str),
            max_size_override: encoded_max_size.map(Ok),
            boxed,
            attrs,
            no_accessors,
            comments: location::get_comments(comment_node),
        }))
    }

    pub(crate) fn generate_rust_type(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        let typ = match &self.ftype {
            FieldType::Map {
                typestr,
                key,
                val,
                max_len,
                ..
            } => {
                let key = key.generate_rust_type(ctx)?;
                let val = val.generate_rust_type(ctx)?;
                let ty = map_type_parsed(typestr, key, val, *max_len)?;
                quote! { #ty }
            }

            FieldType::Repeated {
                typestr,
                typ,
                max_len,
                ..
            } => {
                let inner = typ.generate_rust_type(ctx)?;
                let ty = vec_type_parsed(typestr, inner, *max_len)?;
                quote! { #ty }
            }

            FieldType::Single(t) | FieldType::Optional(t, _) => t.generate_rust_type(ctx)?,

            FieldType::Custom(CustomField::Type(t)) => return Ok(quote! {#t}),
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have a type")
            }
        };
        Ok(ctx.wrapped_type(typ, self.boxed, self.is_option()))
    }

    pub(crate) fn generate_field(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        if let FieldType::Custom(CustomField::Delegate(_)) = self.ftype {
            return Ok(quote! {});
        }
        let typ = self.generate_rust_type(ctx)?;
        let name = &self.san_rust_name;
        let attrs = &self.attrs;
        let comments = self.comments.map(Comments::lines).into_iter().flatten();

        let hazzer_warning = self.is_hazzer().then(|| {
            let empty_line = self.comments.map(|_| "").into_iter();
            let warning = std::iter::once(" *Note:* The presence of this field is tracked separately in the `_has` field. It's recommended to access this field via the accessor rather than directly.");
            empty_line.chain(warning)
        }).into_iter().flatten();

        Ok(
            quote! { #(#[doc = #comments])* #(#[doc = #hazzer_warning])* #(#attrs)* pub #name : #typ, },
        )
    }

    pub(crate) fn generate_default(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        match self.ftype {
            FieldType::Single(ref t)
            | FieldType::Optional(ref t, OptionalRepr::Hazzer | OptionalRepr::None) => {
                if let Some(default) = self.default {
                    let value = t.generate_default(default, ctx)?;
                    return Ok(ctx.wrapped_value(value, self.boxed, false));
                }
            }
            // Options don't use custom defaults, they should just default to None
            FieldType::Optional(_, OptionalRepr::Option) => {
                return Ok(quote! { ::core::option::Option::None });
            }
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have default")
            }
            _ => {}
        }
        Ok(quote! { ::core::default::Default::default() })
    }

    pub(crate) fn generate_accessors(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        match &self.ftype {
            FieldType::Optional(type_spec, opt) => {
                let (deref, deref_mut) = if self.boxed {
                    (format_ident!("as_deref"), format_ident!("as_deref_mut"))
                } else {
                    (format_ident!("as_ref"), format_ident!("as_mut"))
                };

                let fname = &self.san_rust_name;
                let getter_doc =
                    format!(" Return a reference to `{}` as an `Option`", self.rust_name);
                let type_name = type_spec.generate_rust_type(ctx)?;

                // Getter is needed for encoding, so we have to generate it
                let mut accessors = match opt {
                    OptionalRepr::Hazzer => {
                        quote! {
                            #[doc = #getter_doc]
                            #[inline]
                            pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                                self._has.#fname().then_some(&self.#fname)
                            }
                        }
                    }
                    OptionalRepr::Option => {
                        quote! {
                            #[doc = #getter_doc]
                            #[inline]
                            pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                                self.#fname.#deref()
                            }
                        }
                    }
                    OptionalRepr::None => {
                        quote! {
                            #[doc = #getter_doc]
                            #[inline]
                            pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                                ::core::option::Option::Some(&self.#fname)
                            }
                        }
                    }
                };

                if !self.no_accessors {
                    let wrapped_type = ctx.wrapped_type(type_name.clone(), self.boxed, true);
                    let setter_name = format_ident!("set_{}", self.rust_name);
                    let muter_name = format_ident!("mut_{}", self.rust_name);
                    let clearer_name = format_ident!("clear_{}", self.rust_name);
                    let taker_name = format_ident!("take_{}", self.rust_name);
                    let init_name = format_ident!("init_{}", self.rust_name);

                    let setter_doc = format!(" Set the value and presence of `{}`", self.rust_name);
                    let muter_doc = format!(
                        " Return a mutable reference to `{}` as an `Option`",
                        self.rust_name
                    );
                    let clearer_doc = format!(" Clear the presence of `{}`", self.rust_name);
                    let taker_doc = format!(
                        " Take the value of `{}` and clear its presence",
                        self.rust_name
                    );
                    let init_doc = format!(
                        " Builder method that sets the value of `{}`. Useful for initializing the message.",
                        self.rust_name
                    );

                    // Add rest of accessors
                    accessors.extend(match opt {
                        OptionalRepr::Hazzer => {
                            quote! {
                                #[doc = #setter_doc]
                                #[inline]
                                pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                                    self._has.#setter_name();
                                    self.#fname = value.into();
                                    self
                                }

                                #[doc = #muter_doc]
                                #[inline]
                                pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                                    self._has.#fname().then_some(&mut self.#fname)
                                }

                                #[doc = #clearer_doc]
                                #[inline]
                                pub fn #clearer_name(&mut self) -> &mut Self {
                                    self._has.#clearer_name();
                                    self
                                }

                                #[doc = #taker_doc]
                                #[inline]
                                pub fn #taker_name(&mut self) -> #wrapped_type {
                                    let val = self._has.#fname().then(|| ::core::mem::take(&mut self.#fname));
                                    self._has.#clearer_name();
                                    val
                                }

                                #[doc = #init_doc]
                                #[inline]
                                pub fn #init_name(mut self, value: #type_name) -> Self {
                                    self.#setter_name(value);
                                    self
                                }
                            }
                        }

                        OptionalRepr::None => {
                            quote! {
                                #[doc = #setter_doc]
                                #[inline]
                                pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                                    self.#fname = value.into();
                                    self
                                }

                                #[doc = #muter_doc]
                                #[inline]
                                pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                                    ::core::option::Option::Some(&mut self.#fname)
                                }

                                #[doc = #init_doc]
                                #[inline]
                                pub fn #init_name(mut self, value: #type_name) -> Self {
                                    self.#setter_name(value);
                                    self
                                }
                            }
                        }

                        OptionalRepr::Option => {
                            quote! {
                                #[doc = #setter_doc]
                                #[inline]
                                pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                                    self.#fname = ::core::option::Option::Some(value.into());
                                    self
                                }

                                #[doc = #muter_doc]
                                #[inline]
                                pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                                    self.#fname.#deref_mut()
                                }

                                #[doc = #clearer_doc]
                                #[inline]
                                pub fn #clearer_name(&mut self) -> &mut Self {
                                    self.#fname = ::core::option::Option::None;
                                    self
                                }

                                #[doc = #taker_doc]
                                #[inline]
                                pub fn #taker_name(&mut self) -> #wrapped_type {
                                    self.#fname.take()
                                }

                                #[doc = #init_doc]
                                #[inline]
                                pub fn #init_name(mut self, value: #type_name) -> Self {
                                    self.#setter_name(value);
                                    self
                                }
                            }
                        }
                    })
                }
                Ok(accessors)
            }

            FieldType::Single(type_spec) if !self.no_accessors => {
                let type_name = type_spec.generate_rust_type(ctx)?;
                let setter_name = format_ident!("set_{}", self.rust_name);
                let muter_name = format_ident!("mut_{}", self.rust_name);
                let init_name = format_ident!("init_{}", self.rust_name);
                let fname = &self.san_rust_name;

                let getter_doc = format!(" Return a reference to `{}`", self.rust_name);
                let muter_doc = format!(" Return a mutable reference to `{}`", self.rust_name);
                let setter_doc = format!(" Set the value of `{}`", self.rust_name);
                let init_doc = format!(
                    " Builder method that sets the value of `{}`. Useful for initializing the message.",
                    self.rust_name
                );

                let accessors = quote! {
                    #[doc = #getter_doc]
                    #[inline]
                    pub fn #fname(&self) -> &#type_name {
                        &self.#fname
                    }

                    #[doc = #muter_doc]
                    #[inline]
                    pub fn #muter_name(&mut self) -> &mut #type_name {
                        &mut self.#fname
                    }

                    #[doc = #setter_doc]
                    #[inline]
                    pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                        self.#fname = value.into();
                        self
                    }

                    #[doc = #init_doc]
                    #[inline]
                    pub fn #init_name(mut self, value: #type_name) -> Self {
                        self.#fname = value.into();
                        self
                    }
                };
                Ok(accessors)
            }
            _ => Ok(quote! {}),
        }
    }

    pub(crate) fn generate_decode_branch(
        &self,
        ctx: &Context<'proto>,
        tag: &Ident,
        decoder: &Ident,
    ) -> Result<TokenStream, String> {
        let fnum = self.num;
        let fname = &self.san_rust_name;
        let mut_ref = Ident::new("mut_ref", Span::call_site());
        let extra_deref = self.boxed.then(|| quote! { * });

        let decode_code = match &self.ftype {
            FieldType::Map { key, val, .. } => {
                let key_decode_expr = key.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                let val_decode_expr = val.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                let key_type = key.generate_rust_type(ctx)?;
                let val_type = val.generate_rust_type(ctx)?;
                quote! {
                    if let Some((k, v)) = #decoder.decode_map_elem(
                        |#mut_ref: &mut #key_type, #decoder| { #key_decode_expr; Ok(()) },
                        |#mut_ref: &mut #val_type, #decoder| { #val_decode_expr; Ok(()) },
                    )?
                    {
                        if let (Err(_), false) = (self.#fname.pb_insert(k, v), #decoder.ignore_repeated_cap_err) {
                            return Err(::micropb::DecodeError::Capacity);
                        }
                    }
                }
            }

            FieldType::Single(tspec) => {
                let decode_stmts = tspec.generate_decode_mut(ctx, true, decoder, &mut_ref)?;
                quote! {
                    let #mut_ref = &mut #extra_deref self.#fname;
                    { #decode_stmts };
                }
            }

            FieldType::Optional(tspec, OptionalRepr::None) => {
                let decode_stmts = tspec.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                quote! {
                    let #mut_ref = &mut #extra_deref self.#fname;
                    { #decode_stmts };
                }
            }

            FieldType::Optional(tspec, OptionalRepr::Hazzer) => {
                let decode_expr = tspec.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                let setter = format_ident!("set_{}", self.rust_name);
                quote! {
                    let #mut_ref = &mut #extra_deref self.#fname;
                    { #decode_expr };
                    self._has.#setter();
                }
            }

            FieldType::Optional(tspec, OptionalRepr::Option) => {
                let decode_stmts = tspec.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                quote! {
                    let #mut_ref = &mut #extra_deref *self.#fname.get_or_insert_with(::core::default::Default::default);
                    { #decode_stmts };
                }
            }

            FieldType::Repeated { typ, .. } => {
                // Type can be packed and is Copy, so we check the wire type to see if we can
                // do packed decoding
                if let Some(val) = typ.generate_decode_val(ctx, decoder) {
                    quote! {
                        if #tag.wire_type() == ::micropb::WIRE_TYPE_LEN {
                            #decoder.decode_packed(&mut #extra_deref self.#fname, |#decoder| #val.map(|v| v as _))?;
                        } else {
                            if let (Err(_), false) = (self.#fname.pb_push(#val? as _), #decoder.ignore_repeated_cap_err) {
                                return Err(::micropb::DecodeError::Capacity);
                            }
                        }
                    }
                } else {
                    let decode_expr = typ.generate_decode_mut(ctx, false, decoder, &mut_ref)?;
                    let rust_type = typ.generate_rust_type(ctx)?;
                    quote! {
                        let mut val: #rust_type = ::core::default::Default::default();
                        let #mut_ref = &mut val;
                        { #decode_expr };
                        if let (Err(_), false) = (self.#fname.pb_push(val), #decoder.ignore_repeated_cap_err) {
                            return Err(::micropb::DecodeError::Capacity);
                        }
                    }
                }
            }

            FieldType::Custom(CustomField::Type(_)) => {
                quote! { if !self.#fname.decode_field(#tag, #decoder)? { return Err(::micropb::DecodeError::CustomField) } }
            }

            FieldType::Custom(CustomField::Delegate(field)) => {
                quote! { if !self.#field.decode_field(#tag, #decoder)? { return Err(::micropb::DecodeError::CustomField) } }
            }
        };

        Ok(quote! {
            #fnum => { #decode_code }
        })
    }

    fn wire_type(&self) -> u8 {
        match &self.ftype {
            FieldType::Single(typ)
            | FieldType::Optional(typ, _)
            | FieldType::Repeated {
                typ, packed: false, ..
            } => typ.wire_type(),

            FieldType::Map { .. } | FieldType::Repeated { packed: true, .. } => {
                micropb::WIRE_TYPE_LEN
            }

            // Custom fields don't need tags, so just return a placeholder wiretype
            FieldType::Custom(_) => micropb::WIRE_TYPE_VARINT,
        }
    }

    pub(crate) fn generate_max_size(
        &self,
        ctx: &Context<'proto>,
        msg_name: &'proto str,
    ) -> TokenStream {
        if let Some(max_size) = &self.max_size_override {
            return match max_size {
                Ok(size) => quote! { ::core::result::Result::Ok(#size) },
                Err(err) => { 
                    let err = field_error_str(&ctx.pkg, msg_name, self.name, err);
                    quote! { ::core::result::Result::<usize, _>::Err(#err) }
                },
            };
        }

        let wire_type = self.wire_type();
        let tag = micropb::Tag::from_parts(self.num, wire_type);
        let tag_len = ::micropb::size::sizeof_tag(tag);

        match &self.ftype {
            FieldType::Map {
                key, val, max_len, ..
            } => max_len
                .map(|len| {
                    let len = len as usize;
                    let key_size = key.generate_max_size(ctx, msg_name, self.name);
                    let val_size = val.generate_max_size(ctx, msg_name, self.name);
                    quote! {
                        match (#key_size, #val_size) {
                            (::core::result::Result::Err(err), _) => ::core::result::Result::<usize, &'static str>::Err(err),
                            (_, ::core::result::Result::Err(err)) => ::core::result::Result::<usize, &'static str>::Err(err),
                            (::core::result::Result::Ok(key_size), ::core::result::Result::Ok(val_size)) => {
                                let max_size = ::micropb::size::sizeof_len_record(key_size + val_size + 2) + #tag_len;
                                ::core::result::Result::Ok(max_size * #len)
                            }
                        }
                    }
                })
                .unwrap_or_else(|| { 
                    let err = field_error_str(&ctx.pkg, msg_name, self.name, "unbounded map");
                    quote! {::core::result::Result::<usize, &'static str>::Err(#err)} 
                }),

            FieldType::Single(type_spec) | FieldType::Optional(type_spec, _) => {
                let size = type_spec.generate_max_size(ctx, msg_name, self.name);
                quote! { ::micropb::const_map!(#size, |size| size + #tag_len) }
            }

            FieldType::Repeated {
                typ,
                packed,
                max_len,
                ..
            } => max_len.map(|len| {
                let len = len as usize;
                let size = typ.generate_max_size(ctx, msg_name, self.name);
                if *packed {
                    quote! { ::micropb::const_map!(#size, |size| ::micropb::size::sizeof_len_record(#len * size) + #tag_len) }
                } else {
                    quote! { ::micropb::const_map!(#size, |size| (size + #tag_len) * #len) }
                }
            }).unwrap_or_else(|| { 
                let err = field_error_str(&ctx.pkg, msg_name, self.name, "unbounded vec");
                quote! { ::core::result::Result::<usize, &'static str>::Err(#err) } 
            }),

            FieldType::Custom(CustomField::Type(custom)) => quote! { <#custom as ::micropb::field::FieldEncode>::MAX_SIZE },
            FieldType::Custom(CustomField::Delegate(_)) => quote! { ::core::result::Result::Ok(0) },
        }
    }

    pub(crate) fn generate_cache_field(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        let typ = match &self.ftype {
            FieldType::Single(type_spec) => type_spec.generate_cache_type(ctx),
            FieldType::Optional(type_spec, _) => type_spec.generate_cache_type(ctx),

            FieldType::Repeated { typ, cache_vec_typestr, max_len, packed: false, .. } => {
                if let Some(cache_type) = typ.generate_cache_type(ctx) {
                    let cache_vec_type = vec_type_parsed(cache_vec_typestr, cache_type, *max_len)?;
                    Some(quote! { #cache_vec_type })
                } else {
                    None
                }
            },
            FieldType::Repeated { packed: true, .. } => {
                Some(quote! { usize })
            },

            FieldType::Map { val, cache_vec_typestr, max_len, .. } => {
                // Key type can't be a message, so we only ever need to cache the value type
                if let Some(cache_type) = val.generate_cache_type(ctx) {
                    let cache_vec_typestr = cache_vec_typestr.as_ref().ok_or_else(|| "missing cache_vec_type".to_owned())?;
                    let cache_vec_type = vec_type_parsed(cache_vec_typestr, cache_type, *max_len)?;
                    Some(quote! { #cache_vec_type })
                } else {
                    None
                }
            },

            FieldType::Custom(_) => None,
        };

        let name = &self.san_rust_name;
        Ok(typ.map(|typ| quote! { pub #name: #typ, }).unwrap_or_default())
    }

    pub(crate) fn generate_encode(
        &self,
        ctx: &Context<'proto>,
        func_type: &EncodeFunc,
    ) -> TokenStream {
        let fname = &self.san_rust_name;
        let val_ref = Ident::new("val_ref", Span::call_site());
        let extra_deref = self.boxed.then(|| quote! { * });
        let wire_type = self.wire_type();
        let tag = micropb::Tag::from_parts(self.num, wire_type);
        let tag_val = tag.varint();
        let tag_len = ::micropb::size::sizeof_tag(tag);

        let sizeof_code = match &self.ftype {
            FieldType::Map { key, val, .. } => {
                let key_sizeof = key.generate_sizeof(ctx, &val_ref);

                let (val_sizeof, stmts) = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        (
                           val.generate_sizeof(ctx, &val_ref),
                           quote! { #size += ::micropb::size::sizeof_len_record(len) + #tag_len; }
                        )                    
                    }
                    EncodeFunc::PopulateCache(cache) => {
                        // The cache vec type should have the same capacity as the container type,
                        // since they both use the same max_len, so we can panic on overflow
                        let val_sizeof = if val.is_cached(ctx) {
                            quote! {
                                let elem = #val_ref.populate_cache();
                                let sz = elem._size;
                                #cache.#fname.pb_push(elem).expect("vec overflow while caching");
                                sz
                            }
                        } else {
                           val.generate_sizeof(ctx, &val_ref)
                        };
                        (
                            val_sizeof,
                            quote! { #cache._size += ::micropb::size::sizeof_len_record(len) + #tag_len; }
                        )                    
                    }

                    EncodeFunc::Encode(encoder) | EncodeFunc::EncodeCached(encoder, _) => {
                        let key_encode = key.generate_encode_expr(ctx, encoder, &val_ref);
                        let key_wtype = key.wire_type();
                        let val_wtype = val.wire_type();

                        let (val_encode, val_sizeof) = if let EncodeFunc::EncodeCached(encoder, cache) = &func_type && val.is_cached(ctx) {
                            (
                                quote! { #val_ref.encode_len_delimited_cached(#encoder, &#cache.#fname[i]) },
                                quote! { #cache.#fname[i]._size }
                            )
                        } else {
                            (val.generate_encode_expr(ctx, encoder, &val_ref), val.generate_sizeof(ctx, &val_ref))
                        };
                        let stmts = quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encoder.encode_map_elem(
                                len, k, #key_wtype, v, #val_wtype,
                                |#encoder, #val_ref| { #key_encode },
                                |#encoder, #val_ref| { #val_encode }
                            )?;
                        };
                        (val_sizeof, stmts)
                    }
                };
                quote! {
                    for (i, (k, v)) in (&#extra_deref self.#fname).into_iter().enumerate() {
                        let len = ::micropb::size::sizeof_map_elem(k, v, |#val_ref| { #key_sizeof }, |#val_ref| { #val_sizeof });
                        #stmts
                    }
                }
            }

            FieldType::Single(tspec) | FieldType::Optional(tspec, _) => {
                let check = if let FieldType::Optional(..) = self.ftype {
                    quote! { if let ::core::option::Option::Some(#val_ref) = self.#fname() }
                } else {
                    let implicit_presence_check = tspec.generate_implicit_presence_check(&val_ref);
                    quote! {
                        let #val_ref = &#extra_deref self.#fname;
                        #implicit_presence_check
                    }
                };
                let stmts = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        let sizeof_expr = tspec.generate_sizeof(ctx, &val_ref);
                        quote! { #size += #tag_len + #sizeof_expr; }
                    }
                    EncodeFunc::PopulateCache(cache) => {
                        if tspec.is_cached(ctx) {
                            quote! {
                                #cache.#fname = #val_ref.populate_cache();
                                #cache._size += #tag_len + ::micropb::size::sizeof_len_record(#cache.#fname._size);
                            }
                        } else {
                            let sizeof_expr = tspec.generate_sizeof(ctx, &val_ref);
                            quote! { #cache._size += #tag_len + #sizeof_expr; }
                        }
                    }

                    EncodeFunc::Encode(encoder) => {
                        let encode_expr = tspec.generate_encode_expr(ctx, encoder, &val_ref);
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encode_expr?;
                        }
                    }
                    EncodeFunc::EncodeCached(encoder, cache) => {
                        let encode_expr = if tspec.is_cached(ctx) {
                            quote! { #val_ref.encode_len_delimited_cached(#encoder, &#cache.#fname) }
                        } else {
                            tspec.generate_encode_expr(ctx, encoder, &val_ref)
                        };
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encode_expr?;
                        }
                    }
                };
                quote! {
                    #check {
                        #stmts
                    }
                }
            }

            FieldType::Repeated {
                typ, packed: false, ..
            } => 'expr: {
                let stmts = match (&func_type, typ.fixed_size()) {
                    (EncodeFunc::Sizeof(size), Some(fixed)) => {
                        break 'expr quote! { #size += self.#fname.len() * (#tag_len + #fixed); };
                    }
                    (EncodeFunc::Sizeof(size), None) => {
                        let sizeof_expr = typ.generate_sizeof(ctx, &val_ref);
                        quote! { #size += #tag_len + #sizeof_expr; }
                    }
                    (EncodeFunc::PopulateCache(cache), Some(fixed)) => {
                        break 'expr quote! { #cache._size += self.#fname.len() * (#tag_len + #fixed); };
                    }
                    (EncodeFunc::PopulateCache(cache), None) => {
                        if typ.is_cached(ctx) {
                            quote! {
                                let elem = #val_ref.populate_cache();
                                #cache._size += #tag_len + ::micropb::size::sizeof_len_record(elem._size);
                                #cache.#fname.pb_push(elem).expect("vec overflow while caching");
                            }
                        } else {
                            let sizeof_expr = typ.generate_sizeof(ctx, &val_ref);
                            quote! { #cache._size += #tag_len + #sizeof_expr; }
                        }
                    }

                    (EncodeFunc::Encode(encoder), _) => {
                        let encode_expr = typ.generate_encode_expr(ctx, encoder, &val_ref);
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encode_expr?;
                        }
                    }
                    (EncodeFunc::EncodeCached(encoder, cache), _) => {
                        let encode_expr = if typ.is_cached(ctx) {
                            quote! { #val_ref.encode_len_delimited_cached(#encoder, &#cache.#fname[i]) }
                        } else {
                            typ.generate_encode_expr(ctx, encoder, &val_ref)
                        };
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encode_expr?;
                        }
                    }
                };
                quote! {
                    for (i, #val_ref) in self.#fname.iter().enumerate() {
                        #stmts
                    }
                }
            }

            FieldType::Repeated {
                typ, packed: true, ..
            } => {
                let len = if let Some(fixed) = typ.fixed_size() {
                    quote! { self.#fname.len() * #fixed }
                } else if let EncodeFunc::EncodeCached(_, cache) = &func_type {
                    quote! { #cache.#fname }
                } else {
                    let sizeof_expr = typ.generate_sizeof(ctx, &val_ref);
                    quote! { ::micropb::size::sizeof_packed(& #extra_deref self.#fname, |#val_ref| #sizeof_expr) }
                };
                let stmts = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        quote! { #size += #tag_len + ::micropb::size::sizeof_len_record(len); }
                    }
                    EncodeFunc::PopulateCache(cache) => {
                        quote! { 
                            #cache._size += #tag_len + ::micropb::size::sizeof_len_record(len);
                            #cache.#fname = len;
                        }
                    }

                    EncodeFunc::Encode(encoder) | EncodeFunc::EncodeCached(encoder, _) => {
                        let encode_expr = typ.generate_encode_expr(ctx, encoder, &val_ref);
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encoder.encode_packed(len, & #extra_deref self.#fname, |#encoder, val| {let #val_ref = &val; #encode_expr})?;
                        }
                    }
                };
                quote! {
                    if !self.#fname.is_empty() {
                        let len = #len;
                        #stmts
                    }
                }
            }

            FieldType::Custom(CustomField::Type(_)) => match &func_type {
                EncodeFunc::Sizeof(size) => quote! { #size += self.#fname.compute_fields_size(); },
                EncodeFunc::PopulateCache(cache) => quote! { #cache._size += self.#fname.compute_fields_size(); },
                EncodeFunc::Encode(encoder) | EncodeFunc::EncodeCached(encoder, _) => quote! { self.#fname.encode_fields(#encoder)?; },
            },

            FieldType::Custom(CustomField::Delegate(_)) => quote! {},
        };

        quote! {{
            #sizeof_code
        }}
    }
}

#[cfg(test)]
pub(crate) fn make_test_field<'a>(
    num: u32,
    name: &'a str,
    boxed: bool,
    ftype: FieldType<'a>,
) -> Field<'a> {
    Field {
        num,
        ftype,
        name,
        rust_name: name.to_owned(),
        san_rust_name: Ident::new_raw(name, proc_macro2::Span::call_site()),
        default: None,
        boxed,
        max_size_override: None,
        attrs: vec![],
        no_accessors: false,
        comments: None,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::Span;

    use crate::{
        config::{Config, IntSize, parse_attributes},
        generator::{make_ctx, type_spec::PbInt},
        pathtree::Node,
    };

    use super::*;

    fn field_proto(
        num: u32,
        name: &str,
        label: Option<Label>,
        proto3_opt: bool,
    ) -> FieldDescriptorProto {
        let mut f = FieldDescriptorProto::default();
        f.set_name(name.to_owned());
        f.set_number(num as i32);
        f.set_type(Type::Bool);
        f.set_proto3_optional(proto3_opt);
        if let Some(label) = label {
            f.set_label(label);
        }
        f
    }

    #[test]
    fn from_proto_skipped() {
        let config = Box::new(Config::new().skip(true));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(2, "field", None, false);

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto2;
        assert!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn from_proto_field() {
        let config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(2, "field", None, false);

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto3;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap(),
            Field {
                num: 2,
                ftype: FieldType::Single(TypeSpec::Bool),
                name: "field",
                rust_name: "field".to_owned(),
                san_rust_name: Ident::new_raw("field", Span::call_site()),
                default: None,
                boxed: false,
                max_size_override: None,
                attrs: vec![],
                no_accessors: false,
                comments: None
            }
        );

        // With some field configs
        let config = Box::new(
            Config::new()
                .boxed(true)
                .rename_field("renamed")
                .field_attributes("#[attr]"),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let mut field = field_proto(2, "field", None, false);
        field.set_default_value("true".to_owned());

        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap(),
            Field {
                num: 2,
                ftype: FieldType::Single(TypeSpec::Bool),
                name: "field",
                rust_name: "renamed".to_owned(),
                san_rust_name: Ident::new("renamed", Span::call_site()),
                default: Some("true"),
                boxed: true,
                max_size_override: None,
                attrs: parse_attributes("#[attr]").unwrap(),
                no_accessors: false,
                comments: None
            }
        );
    }

    #[test]
    fn from_proto_field_type() {
        let config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(0, "field", None, false);

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto3;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Single(TypeSpec::Bool)
        );
        ctx.syntax = Syntax::Proto2;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // Required fields are treated like optionals
        let field = field_proto(0, "field", Some(Label::Required), false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // In proto3, if proto3_optional is set then field is optional
        ctx.syntax = Syntax::Proto3;
        let field = field_proto(0, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // Boxed optionals should default to using Option instead of hazzers
        let config = Box::new(Config::new().boxed(true));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        ctx.syntax = Syntax::Proto2;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option)
        );

        // Explicitly set the optional_repr to Option, overriding the default
        let config = Box::new(Config::new().optional_repr(OptionalRepr::Option));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option)
        );
    }

    #[test]
    fn from_proto_custom() {
        // Even if the field is boxed or optional, as long as we specify a custom field, those
        // other options are all ignored
        let config = Box::new(
            Config::new()
                .boxed(true)
                .custom_field(crate::config::CustomField::Type("Custom<false>".to_owned())),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field", Some(Label::Optional), true);

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto2;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Custom(CustomField::Type(syn::parse_str("Custom<false>").unwrap()))
        );

        let config = Box::new(
            Config::new()
                .boxed(true)
                .custom_field(crate::config::CustomField::Delegate("field".to_owned())),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Custom(CustomField::Delegate(syn::parse_str("field").unwrap()))
        );
    }

    #[test]
    fn from_proto_repeated() {
        // Repeated fields with custom element int type
        let config = Box::new(Config::new().max_len(21).vec_type("Vec<$N>"));
        let mut node = Node::default();
        *node.add_path(std::iter::once("elem")).value_mut() =
            Some(Box::new(Config::new().int_size(IntSize::S8)));
        let field_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        let mut field = field_proto(0, "field", Some(Label::Repeated), false);
        field.set_type(Type::Int32);

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto3;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                packed: false,
                typestr: "Vec<$N>".to_owned(),
                cache_vec_typestr: String::new(),
                max_len: Some(21)
            }
        );
        field.set_options(Default::default());
        field.options.set_packed(true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                packed: true,
                typestr: "Vec<$N>".to_owned(),
                cache_vec_typestr: String::new(),
                max_len: Some(21)
            }
        );
    }

    #[test]
    fn from_proto_map() {
        let config = Box::new(Config::new().map_type("std::Map"));
        let mut node = Node::default();
        *node.add_path(std::iter::once("key")).value_mut() =
            Some(Box::new(Config::new().int_size(IntSize::S8)));
        *node.add_path(std::iter::once("value")).value_mut() =
            Some(Box::new(Config::new().string_type("std::String")));
        let field_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        let mut key = field_proto(1, "key", Some(Label::Optional), false);
        key.set_type(Type::Int32);
        let mut value = field_proto(1, "value", Some(Label::Optional), false);
        value.set_type(Type::String);
        let mut map_elem = DescriptorProto {
            name: "MapElem".to_owned(),
            field: vec![key, value],
            extension: vec![],
            nested_type: vec![],
            enum_type: vec![],
            extension_range: vec![],
            oneof_decl: vec![],
            options: Default::default(),
            reserved_range: vec![],
            reserved_name: vec![],

            _has: Default::default(),
        };
        map_elem._has.set_name();
        map_elem._has.set_options();
        map_elem.options.set_map_entry(true);
        let mut field = field_proto(0, "field", Some(Label::Repeated), false);
        field.set_type(Type::Message);
        field.set_type_name("MapElem".to_owned());

        let mut ctx = make_ctx();
        ctx.syntax = Syntax::Proto2;
        assert_eq!(
            Field::from_proto(&field, &field_conf, None, &ctx, Some(&map_elem))
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Map {
                key: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                val: TypeSpec::String {
                    typestr: "std::String".to_owned(),
                    max_bytes: None
                },
                typestr: "std::Map".to_owned(),
                cache_vec_typestr: None,
                max_len: None
            }
        );
    }
}
