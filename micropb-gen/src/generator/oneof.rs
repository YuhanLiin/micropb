use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Ident, Lifetime};

use super::{
    derive_msg_attr,
    field::CustomField,
    sanitized_ident,
    type_spec::{find_lifetime_from_type, TypeSpec},
    CurrentConfig, EncodeFunc, Generator,
};

use crate::descriptor::{FieldDescriptorProto, OneofDescriptorProto};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct OneofField<'a> {
    pub(crate) num: u32,
    pub(crate) tspec: TypeSpec,
    #[allow(unused)]
    /// Protobuf name
    pub(crate) name: &'a str,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) rust_name: Ident,
    pub(crate) boxed: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> OneofField<'a> {
    pub(crate) fn from_proto(
        proto: &'a FieldDescriptorProto,
        field_conf: &CurrentConfig,
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
            boxed: field_conf.config.boxed.unwrap_or(false),
            attrs,
        }))
    }

    fn generate_field(&self, gen: &Generator) -> TokenStream {
        let typ = gen.wrapped_type(self.tspec.generate_rust_type(gen), self.boxed, false);
        let name = &self.rust_name;
        let attrs = &self.attrs;
        quote! { #(#attrs)* #name(#typ), }
    }

    fn generate_decode_branch(
        &self,
        oneof_name: &Ident,
        oneof_type: &TokenStream,
        oneof_boxed: bool,
        gen: &Generator,
        decoder: &Ident,
    ) -> TokenStream {
        let fnum = self.num;
        let mut_ref = Ident::new("mut_ref", Span::call_site());
        let variant_name = &self.rust_name;
        let extra_deref_of = oneof_boxed.then(|| quote! { * });
        let extra_deref_var = self.boxed.then(|| quote! { * });

        let decode_stmts = self
            .tspec
            .generate_decode_mut(gen, false, decoder, &mut_ref);
        let value = gen.wrapped_value(
            quote! { #oneof_type::#variant_name(::core::default::Default::default()) },
            oneof_boxed,
            true,
        );
        quote! {
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
        }
    }

    fn generate_encode_branch(
        &self,
        oneof_type: &TokenStream,
        gen: &Generator,
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
                let sizeof_expr = self.tspec.generate_sizeof(gen, &val_ref);
                quote! { #size += #tag_len + #sizeof_expr; }
            }
            EncodeFunc::Encode(encoder) => {
                let encode_expr = self.tspec.generate_encode_expr(gen, encoder, &val_ref);
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

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Oneof<'a> {
    /// Protobuf name
    #[allow(unused)]
    pub(crate) name: &'a str,
    /// Sanitized Rust ident after renaming, used for field name
    pub(crate) san_rust_name: Ident,
    pub(crate) otype: OneofType<'a>,
    pub(crate) field_attrs: Vec<syn::Attribute>,
    pub(crate) type_attrs: Vec<syn::Attribute>,
    pub(crate) boxed: bool,
    pub(crate) derive_dbg: bool,
    pub(crate) derive_partial_eq: bool,
    pub(crate) derive_clone: bool,
    pub(crate) idx: usize,
}

impl<'a> Oneof<'a> {
    pub(crate) fn find_lifetime(&self) -> Option<&Lifetime> {
        match &self.otype {
            OneofType::Custom {
                field: CustomField::Type(ty),
                ..
            } => find_lifetime_from_type(ty),
            _ => None,
        }
    }

    pub(crate) fn from_proto(
        proto: &'a OneofDescriptorProto,
        oneof_conf: CurrentConfig,
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
            field_attrs,
            type_attrs,
        }))
    }

    pub(crate) fn generate_decl(&self, gen: &Generator) -> TokenStream {
        if let OneofType::Enum { type_name, fields } = &self.otype {
            assert!(!fields.is_empty(), "empty enums should have been filtered");
            let fields = fields.iter().map(|f| f.generate_field(gen));
            let derive_msg = derive_msg_attr(
                self.derive_dbg,
                false,
                self.derive_partial_eq,
                self.derive_clone,
            );
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
        let name = &self.san_rust_name;
        let oneof_type = match &self.otype {
            OneofType::Enum { type_name, .. } => {
                gen.wrapped_type(quote! { #msg_mod_name::#type_name }, self.boxed, true)
            }
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
        quote! { #(#attrs)* pub #name: #oneof_type, }
    }

    pub(crate) fn generate_decode_branches(
        &self,
        gen: &Generator,
        msg_mod_name: &Ident,
        tag: &Ident,
        decoder: &Ident,
    ) -> TokenStream {
        let name = &self.san_rust_name;
        match &self.otype {
            OneofType::Enum { fields, type_name } => {
                let oneof_type = quote! { #msg_mod_name::#type_name };
                let branches = fields
                    .iter()
                    .map(|f| f.generate_decode_branch(name, &oneof_type, self.boxed, gen, decoder));
                quote! {
                    #(#branches)*
                }
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
        }
    }

    pub(crate) fn generate_encode(
        &self,
        gen: &Generator,
        msg_mod_name: &Ident,
        func_type: &EncodeFunc,
    ) -> TokenStream {
        let name = &self.san_rust_name;
        match &self.otype {
            OneofType::Enum { type_name, fields } => {
                let oneof_type = quote! { #msg_mod_name::#type_name };
                let extra_deref = self.boxed.then(|| quote! { * });
                let branches = fields
                    .iter()
                    .map(|f| f.generate_encode_branch(&oneof_type, gen, func_type));
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
                EncodeFunc::Encode(encoder) => quote! { self.#name.encode_fields(#encoder)?; },
            },

            OneofType::Custom {
                field: CustomField::Delegate(_),
                ..
            } => quote! {},
        }
    }

    pub(crate) fn generate_max_size(&self, gen: &Generator) -> TokenStream {
        match &self.otype {
            OneofType::Custom { .. } => quote! { ::core::option::Option::None },

            OneofType::Enum { fields, .. } => {
                let variant_sizes = fields.iter().map(|f| {
                    let wire_type = f.tspec.wire_type();
                    let tag = micropb::Tag::from_parts(f.num, wire_type);
                    let tag_len = ::micropb::size::sizeof_tag(tag);
                    let size = f.tspec.generate_max_size(gen);
                    quote! { ::micropb::const_map!(#size, |size| size + #tag_len) }
                });

                quote! {'oneof: {
                    let mut max_size = 0;
                    #(
                        if let ::core::option::Option::Some(size) = #variant_sizes {
                            if size > max_size {
                                max_size = size;
                            }
                        } else {
                            break 'oneof (::core::option::Option::None);
                        }
                    )*
                    ::core::option::Option::Some(max_size)
                }}
            }
        }
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
mod tests {
    use std::borrow::Cow;

    use crate::descriptor::FieldDescriptorProto_::Type;

    use crate::config::{parse_attributes, Config};

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
        assert!(OneofField::from_proto(&field, &oneof_conf)
            .unwrap()
            .is_none());
        let oneof = OneofDescriptorProto::default();
        assert!(Oneof::from_proto(&oneof, oneof_conf, 0).unwrap().is_none());
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
            OneofField::from_proto(&field, &field_conf)
                .unwrap()
                .unwrap(),
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
            OneofField::from_proto(&field, &field_conf)
                .unwrap()
                .unwrap(),
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
        let mut oneof = OneofDescriptorProto::default();
        oneof.set_name("oneof".to_owned());
        assert_eq!(
            Oneof::from_proto(&oneof, oneof_conf, 0).unwrap().unwrap(),
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
                idx: 0
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
            Oneof::from_proto(&oneof, oneof_conf, 0).unwrap().unwrap(),
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
                idx: 0
            }
        );
    }

    #[test]
    fn oneof_custom() {
        let gen = Generator::new();
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
            idx: 0,
        };
        assert!(oneof.generate_decl(&gen).is_empty());
        assert_eq!(
            oneof
                .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
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
            idx: 0,
        };
        assert!(oneof.generate_decl(&gen).is_empty());
        assert!(oneof
            .generate_field(&gen, &Ident::new("Msg", Span::call_site()))
            .to_string()
            .is_empty());
    }
}
