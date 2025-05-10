use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Lifetime};

use crate::config::OptionalRepr;
use crate::descriptor::{
    DescriptorProto, FieldDescriptorProto,
    FieldDescriptorProto_::{Label, Type},
};

use super::Syntax;
use super::{
    type_spec::{find_lifetime_from_type, TypeSpec},
    CurrentConfig, EncodeFunc, Generator,
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum CustomField {
    Type(syn::Type),
    Delegate(Ident),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec,
        val: TypeSpec,
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

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Field<'a> {
    pub(crate) num: u32,
    pub(crate) ftype: FieldType,
    /// Protobuf name
    pub(crate) name: &'a str,
    /// Non-sanitized Rust name after renaming, used for accessor names
    pub(crate) rust_name: String,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) san_rust_name: Ident,
    pub(crate) default: Option<&'a str>,
    pub(crate) boxed: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> Field<'a> {
    pub(crate) fn is_option(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Option))
    }

    pub(crate) fn is_hazzer(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Hazzer))
    }

    pub(crate) fn find_lifetime(&self) -> Option<&Lifetime> {
        match &self.ftype {
            FieldType::Custom(CustomField::Type(ty)) => find_lifetime_from_type(ty),
            _ => None,
        }
    }

    pub(crate) fn from_proto(
        proto: &'a FieldDescriptorProto,
        field_conf: &CurrentConfig,
        syntax: Syntax,
        map_msg: Option<&DescriptorProto>,
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
                let type_path = field_conf.config.map_type_parsed()?.ok_or_else(|| {
                    "Field is of type `map`, but map_type was not configured for it".to_owned()
                })?;
                FieldType::Map {
                    key,
                    val,
                    type_path,
                    max_len: field_conf.config.max_len,
                }
            }

            (None, None, Label::Repeated) => FieldType::Repeated {
                typ: TypeSpec::from_proto(proto, &field_conf.next_conf("elem"))?,
                type_path: field_conf.config.vec_type_parsed()?.ok_or_else(|| {
                    "Field is repeated, but vec_type was not configured for it".to_owned()
                })?,
                max_len: field_conf.config.max_len,
                packed: proto
                    .options()
                    .and_then(|opt| opt.packed().copied())
                    .unwrap_or(false),
            },

            (None, None, Label::Required | Label::Optional)
                if syntax == Syntax::Proto2
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
        let attrs = field_conf.config.field_attr_parsed()?;

        Ok(Some(Field {
            num,
            ftype,
            name,
            rust_name,
            san_rust_name,
            default: proto.default_value().map(String::as_str),
            boxed,
            attrs,
        }))
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
        let name = &self.san_rust_name;
        let attrs = &self.attrs;
        quote! { #(#attrs)* pub #name : #typ, }
    }

    pub(crate) fn generate_default(&self, gen: &Generator) -> Result<TokenStream, String> {
        match self.ftype {
            FieldType::Single(ref t) | FieldType::Optional(ref t, OptionalRepr::Hazzer) => {
                if let Some(default) = self.default {
                    let value = t.generate_default(default, gen)?;
                    return Ok(gen.wrapped_value(value, self.boxed, false));
                }
            }
            // Options don't use custom defaults, they should just default to None
            FieldType::Optional(_, OptionalRepr::Option) => {
                return Ok(quote! { ::core::option::Option::None })
            }
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have default")
            }
            _ => {}
        }
        Ok(quote! { ::core::default::Default::default() })
    }

    pub(crate) fn generate_accessors(&self, gen: &Generator) -> TokenStream {
        match &self.ftype {
            FieldType::Optional(type_spec, opt) => {
                let type_name = type_spec.generate_rust_type(gen);
                let wrapped_type = gen.wrapped_type(type_name.clone(), self.boxed, true);
                let setter_name = format_ident!("set_{}", self.rust_name);
                let muter_name = format_ident!("mut_{}", self.rust_name);
                let clearer_name = format_ident!("clear_{}", self.rust_name);
                let taker_name = format_ident!("take_{}", self.rust_name);
                let init_name = format_ident!("init_{}", self.rust_name);
                let fname = &self.san_rust_name;

                let getter_doc =
                    format!("Return a reference to `{}` as an `Option`", self.rust_name);
                let muter_doc = format!(
                    "Return a mutable reference to `{}` as an `Option`",
                    self.rust_name
                );
                let setter_doc = format!("Set the value and presence of `{}`", self.rust_name);
                let clearer_doc = format!("Clear the presence of `{}`", self.rust_name);
                let taker_doc = format!(
                    "Take the value of `{}` and clear its presence",
                    self.rust_name
                );
                let init_doc = format!(
                    "Builder method that sets the value of `{}`. Useful for initializing the message.",
                    self.rust_name
                );

                if let OptionalRepr::Hazzer = opt {
                    quote! {
                        #[doc = #getter_doc]
                        #[inline]
                        pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                            self._has.#fname().then_some(&self.#fname)
                        }

                        #[doc = #muter_doc]
                        #[inline]
                        pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                            self._has.#fname().then_some(&mut self.#fname)
                        }

                        #[doc = #setter_doc]
                        #[inline]
                        pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                            self._has.#setter_name();
                            self.#fname = value.into();
                            self
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
                } else {
                    let (deref, deref_mut) = if self.boxed {
                        (format_ident!("as_deref"), format_ident!("as_deref_mut"))
                    } else {
                        (format_ident!("as_ref"), format_ident!("as_mut"))
                    };
                    quote! {
                        #[doc = #getter_doc]
                        #[inline]
                        pub fn #fname(&self) -> ::core::option::Option<&#type_name> {
                            self.#fname.#deref()
                        }

                        #[doc = #muter_doc]
                        #[inline]
                        pub fn #muter_name(&mut self) -> ::core::option::Option<&mut #type_name> {
                            self.#fname.#deref_mut()
                        }

                        #[doc = #setter_doc]
                        #[inline]
                        pub fn #setter_name(&mut self, value: #type_name) -> &mut Self {
                            self.#fname = ::core::option::Option::Some(value.into());
                            self
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
            }
            FieldType::Single(type_spec) => {
                let type_name = type_spec.generate_rust_type(gen);
                let setter_name = format_ident!("set_{}", self.rust_name);
                let muter_name = format_ident!("mut_{}", self.rust_name);
                let init_name = format_ident!("init_{}", self.rust_name);
                let fname = &self.san_rust_name;

                let getter_doc = format!("Return a reference to `{}`", self.rust_name);
                let muter_doc = format!("Return a mutable reference to `{}`", self.rust_name);
                let setter_doc = format!("Set the value of `{}`", self.rust_name);
                let init_doc = format!(
                    "Builder method that sets the value of `{}`. Useful for initializing the message.",
                    self.rust_name
                );

                quote! {
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
                }
            }
            _ => quote! {},
        }
    }

    pub(crate) fn generate_decode_branch(
        &self,
        gen: &Generator,
        tag: &Ident,
        decoder: &Ident,
    ) -> TokenStream {
        let fnum = self.num;
        let fname = &self.san_rust_name;
        let mut_ref = Ident::new("mut_ref", Span::call_site());
        let extra_deref = self.boxed.then(|| quote! { * });

        let decode_code = match &self.ftype {
            FieldType::Map { key, val, .. } => {
                let key_decode_expr = key.generate_decode_mut(gen, false, decoder, &mut_ref);
                let val_decode_expr = val.generate_decode_mut(gen, false, decoder, &mut_ref);
                let key_type = key.generate_rust_type(gen);
                let val_type = val.generate_rust_type(gen);
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
                let decode_stmts = tspec.generate_decode_mut(gen, true, decoder, &mut_ref);
                quote! {
                    let #mut_ref = &mut #extra_deref self.#fname;
                    { #decode_stmts };
                }
            }

            FieldType::Optional(tspec, OptionalRepr::Hazzer) => {
                let decode_expr = tspec.generate_decode_mut(gen, false, decoder, &mut_ref);
                let setter = format_ident!("set_{}", self.rust_name);
                quote! {
                    let #mut_ref = &mut #extra_deref self.#fname;
                    { #decode_expr };
                    self._has.#setter();
                }
            }

            FieldType::Optional(tspec, OptionalRepr::Option) => {
                let decode_stmts = tspec.generate_decode_mut(gen, false, decoder, &mut_ref);
                quote! {
                    let #mut_ref = &mut #extra_deref *self.#fname.get_or_insert_with(::core::default::Default::default);
                    { #decode_stmts };
                }
            }

            FieldType::Repeated { typ, .. } => {
                // Type can be packed and is Copy, so we check the wire type to see if we can
                // do packed decoding
                if let Some(val) = typ.generate_decode_val(gen, decoder) {
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
                    let decode_expr = typ.generate_decode_mut(gen, false, decoder, &mut_ref);
                    let rust_type = typ.generate_rust_type(gen);
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

        quote! {
            #fnum => { #decode_code }
        }
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

    pub(crate) fn generate_encode(&self, gen: &Generator, func_type: &EncodeFunc) -> TokenStream {
        let fname = &self.san_rust_name;
        let val_ref = Ident::new("val_ref", Span::call_site());
        let extra_deref = self.boxed.then(|| quote! { * });
        let wire_type = self.wire_type();
        let tag = micropb::Tag::from_parts(self.num, wire_type);
        let tag_val = tag.varint();
        let tag_len = ::micropb::size::sizeof_tag(tag);

        let sizeof_code = match &self.ftype {
            FieldType::Map { key, val, .. } => {
                let key_sizeof = key.generate_sizeof(gen, &val_ref);
                let val_sizeof = val.generate_sizeof(gen, &val_ref);

                let stmts = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        quote! { #size += ::micropb::size::sizeof_len_record(len) + #tag_len; }
                    }
                    EncodeFunc::Encode(encoder) => {
                        let key_encode = key.generate_encode_expr(gen, encoder, &val_ref);
                        let key_wtype = key.wire_type();
                        let val_encode = val.generate_encode_expr(gen, encoder, &val_ref);
                        let val_wtype = val.wire_type();
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encoder.encode_map_elem(
                                len, k, #key_wtype, v, #val_wtype,
                                |#encoder, #val_ref| { #key_encode },
                                |#encoder, #val_ref| { #val_encode }
                            )?;
                        }
                    }
                };
                quote! {
                    for (k, v) in self.#fname.pb_iter() {
                        let len = ::micropb::size::sizeof_map_elem(k, v, |#val_ref| { #key_sizeof }, |#val_ref| { #val_sizeof });
                        #stmts
                    }
                }
            }

            FieldType::Single(tspec) | FieldType::Optional(tspec, _) => {
                let check = if let FieldType::Optional(..) = self.ftype {
                    quote! { if let Some(#val_ref) = self.#fname() }
                } else {
                    let implicit_presence_check = tspec.generate_implicit_presence_check(&val_ref);
                    quote! {
                        let #val_ref = &#extra_deref self.#fname;
                        #implicit_presence_check
                    }
                };
                let stmts = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        let sizeof_expr = tspec.generate_sizeof(gen, &val_ref);
                        quote! { #size += #tag_len + #sizeof_expr; }
                    }
                    EncodeFunc::Encode(encoder) => {
                        let encode_expr = tspec.generate_encode_expr(gen, encoder, &val_ref);
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
                        let sizeof_expr = typ.generate_sizeof(gen, &val_ref);
                        quote! { #size += #tag_len + #sizeof_expr; }
                    }
                    (EncodeFunc::Encode(encoder), _) => {
                        let encode_expr = typ.generate_encode_expr(gen, encoder, &val_ref);
                        quote! {
                            #encoder.encode_varint32(#tag_val)?;
                            #encode_expr?;
                        }
                    }
                };
                quote! {
                    for #val_ref in self.#fname.iter() {
                        #stmts
                    }
                }
            }

            FieldType::Repeated {
                typ, packed: true, ..
            } => {
                let len = if let Some(fixed) = typ.fixed_size() {
                    quote! { self.#fname.len() * #fixed }
                } else {
                    let sizeof_expr = typ.generate_sizeof(gen, &val_ref);
                    quote! { ::micropb::size::sizeof_packed(& #extra_deref self.#fname, |#val_ref| #sizeof_expr) }
                };
                let stmts = match &func_type {
                    EncodeFunc::Sizeof(size) => {
                        quote! { #size += #tag_len + ::micropb::size::sizeof_len_record(len); }
                    }
                    EncodeFunc::Encode(encoder) => {
                        let encode_expr = typ.generate_encode_expr(gen, encoder, &val_ref);
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
                EncodeFunc::Encode(encoder) => quote! { self.#fname.encode_fields(#encoder)?; },
            },

            FieldType::Custom(CustomField::Delegate(_)) => quote! {},
        };

        quote! {{
            #sizeof_code
        }}
    }
}

#[cfg(test)]
pub(crate) fn make_test_field(num: u32, name: &str, boxed: bool, ftype: FieldType) -> Field {
    Field {
        num,
        ftype,
        name,
        rust_name: name.to_owned(),
        san_rust_name: Ident::new_raw(name, proc_macro2::Span::call_site()),
        default: None,
        boxed,
        attrs: vec![],
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::Span;

    use crate::{
        config::{parse_attributes, Config, IntSize},
        generator::type_spec::PbInt,
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
        assert!(Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
            .unwrap()
            .is_none());
    }

    #[test]
    fn from_proto_field() {
        let config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(2, "field", None, false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
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
                attrs: vec![],
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
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
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
                attrs: parse_attributes("#[attr]").unwrap(),
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
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Single(TypeSpec::Bool)
        );
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // Required fields are treated like optionals
        let field = field_proto(0, "field", Some(Label::Required), false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // In proto3, if proto3_optional is set then field is optional
        let field = field_proto(0, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
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
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
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
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
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
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
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
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Custom(CustomField::Delegate(syn::parse_str("field").unwrap()))
        );
    }

    #[test]
    fn from_proto_repeated() {
        // Repeated fields with custom element int type
        let config = Box::new(Config::new().max_len(21).vec_type("Vec"));
        let mut node = Node::default();
        *node.add_path(std::iter::once("elem")).value_mut() =
            Some(Box::new(Config::new().int_size(IntSize::S8)));
        let field_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        let mut field = field_proto(0, "field", Some(Label::Repeated), false);
        field.set_type(Type::Int32);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                packed: false,
                type_path: syn::parse_str("Vec").unwrap(),
                max_len: Some(21)
            }
        );
        field.set_options(Default::default());
        field.options.set_packed(true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                packed: true,
                type_path: syn::parse_str("Vec").unwrap(),
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

        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, Some(&map_elem))
                .unwrap()
                .unwrap()
                .ftype,
            FieldType::Map {
                key: TypeSpec::Int(PbInt::Int32, IntSize::S8),
                val: TypeSpec::String {
                    type_path: syn::parse_str("std::String").unwrap(),
                    max_bytes: None
                },
                type_path: syn::parse_str("std::Map").unwrap(),
                max_len: None
            }
        );
    }
}
