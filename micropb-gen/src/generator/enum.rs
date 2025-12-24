use std::io;

use micropb::size::sizeof_varint32;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Attribute, Ident};

use super::location::{CommentNode, Comments, get_comments, next_comment_node};
use crate::{
    config::IntSize,
    descriptor::EnumDescriptorProto,
    generator::{Context, CurrentConfig, derive_enum_attr, location, msg_error, sanitized_ident},
};

pub(crate) struct Variant<'proto> {
    pub(crate) num: u32,
    pub(crate) rust_name: Ident,
    pub(crate) comments: Option<&'proto Comments>,
}

pub(crate) struct Enum<'proto> {
    /// Sanitized Rust ident, used for struct name
    pub(crate) rust_name: Ident,
    pub(crate) int_type: IntSize,
    pub(crate) signed: bool,
    pub(crate) variants: Vec<Variant<'proto>>,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) comments: Option<&'proto Comments>,
}

impl<'proto> Enum<'proto> {
    pub(crate) fn from_proto(
        proto: &'proto EnumDescriptorProto,
        ctx: &Context<'proto>,
        enum_conf: &CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
    ) -> io::Result<Option<Self>> {
        if enum_conf.config.skip.unwrap_or(false) {
            return Ok(None);
        }

        let name = &proto.name;
        let rust_name = sanitized_ident(name);
        let int_type = enum_conf.config.enum_int_size.unwrap_or(IntSize::S32);
        let unsigned = enum_conf.config.enum_unsigned.unwrap_or(false);
        let signed = !unsigned;
        let attrs = enum_conf
            .config
            .type_attr_parsed()
            .map_err(|e| msg_error(&ctx.pkg, name, &e))?;
        let comments = get_comments(comment_node);

        let variants = proto
            .value
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let num = v.number as u32;
                let var_name = &v.name;
                let var_rust_name = ctx.enum_variant_name(var_name, &rust_name);
                let var_comment_node =
                    next_comment_node(comment_node, location::path::enum_value(i));
                let var_comments = get_comments(var_comment_node);
                Variant {
                    num,
                    rust_name: var_rust_name,
                    comments: var_comments,
                }
            })
            .collect();

        Ok(Some(Self {
            rust_name,
            int_type,
            signed,
            variants,
            attrs,
            comments,
        }))
    }

    pub(crate) fn generate_decl(&self) -> TokenStream {
        let variants = self.variants.iter().map(|v| {
            let num = Literal::i32_unsuffixed(v.num as i32);
            let var_name = &v.rust_name;
            let var_comments = v.comments.map(Comments::lines).into_iter().flatten();
            quote! { #(#[doc = #var_comments])* pub const #var_name: Self = Self(#num); }
        });

        let name = &self.rust_name;
        let default_num = Literal::i32_unsuffixed(self.variants[0].num as i32);
        let derive_enum = derive_enum_attr();
        let itype = self.int_type.type_name(self.signed);
        let max_size = if self.signed {
            10
        } else {
            sizeof_varint32(self.int_type.max_value().try_into().unwrap_or(u32::MAX))
        };
        let comments = self.comments.map(Comments::lines).into_iter().flatten();
        let attrs = &self.attrs;

        quote! {
            #(#[doc = #comments])*
            #derive_enum
            #[repr(transparent)]
            #(#attrs)*
            pub struct #name(pub #itype);

            impl #name {
                #[doc = " Maximum encoded size of the enum"]
                pub const _MAX_SIZE: usize = #max_size;
                #(#variants)*
            }

            impl core::default::Default for #name {
                fn default() -> Self {
                    Self(#default_num)
                }
            }

            impl core::convert::From<#itype> for #name {
                fn from(val: #itype) -> Self {
                    Self(val)
                }
            }
        }
    }
}
