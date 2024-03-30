use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    iter,
    ops::Deref,
};

use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorProto, FileDescriptorSet, Syntax,
};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    config::{Config, IntType},
    pathtree::{Node, PathTree},
};

use self::message::Message;

pub(crate) mod field;
pub(crate) mod message;
pub(crate) mod oneof;
pub(crate) mod type_spec;

fn derive_msg_attr(debug: bool, default: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    let default = default.then(|| quote! { Default, });
    quote! { #[derive(#debug #default Clone, PartialEq)] }
}

fn derive_enum_attr(debug: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    quote! { #[derive(#debug Clone, Copy, PartialEq, Eq, Hash)] }
}

struct CurrentConfig<'a> {
    node: Option<&'a Node<Box<Config>>>,
    config: Cow<'a, Box<Config>>,
}

impl<'a> CurrentConfig<'a> {
    fn next_conf(&'a self, segment: &str) -> Self {
        let mut config: Cow<Box<Config>> = Cow::Borrowed(self.config.borrow());
        if let Some(node) = self.node {
            let next = node.next(segment);
            if let Some(conf) = next.and_then(|n| n.value().as_ref()) {
                (*config.to_mut()).merge(conf);
            }
            Self { node: next, config }
        } else {
            Self { node: None, config }
        }
    }

    fn derive_dbg(&self) -> bool {
        !self.config.no_debug_derive.unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum EncodeDecode {
    EncodeOnly,
    DecodeOnly,
    #[default]
    Both,
}

fn generate_mod_tree(mod_node: &mut Node<TokenStream>) -> TokenStream {
    let code = mod_node.value_mut().take().unwrap_or_default();
    let submods = mod_node.children_mut().map(|(submod_name, inner_node)| {
        let submod_name = Ident::new(submod_name, Span::call_site());
        let inner = generate_mod_tree(inner_node);
        quote! { pub mod #submod_name { #inner } }
    });

    quote! {
        #code
        #(#submods)*
    }
}

#[derive(Debug)]
pub struct Generator {
    syntax: Syntax,
    pkg_path: Vec<String>,
    type_path: RefCell<Vec<String>>,

    pub(crate) encode_decode: EncodeDecode,
    pub(crate) size_cache: bool,
    pub(crate) retain_enum_prefix: bool,
    pub(crate) format: bool,
    pub(crate) use_std: bool,

    pub(crate) config_tree: PathTree<Box<Config>>,
}

impl Default for Generator {
    fn default() -> Self {
        let config_tree = PathTree::new(Box::new(Config::new()));

        Self {
            syntax: Default::default(),
            pkg_path: Default::default(),
            type_path: Default::default(),

            encode_decode: Default::default(),
            size_cache: Default::default(),
            retain_enum_prefix: Default::default(),
            format: true,
            use_std: Default::default(),
            config_tree,
        }
    }
}

impl Generator {
    pub(crate) fn generate_fdset(&mut self, fdset: &FileDescriptorSet) -> TokenStream {
        let mut mod_tree = PathTree::new(TokenStream::new());

        for file in &fdset.file {
            let code = self.generate_fdproto(file);
            if let Some(pkg_name) = &file.package {
                *mod_tree.root.add_path(pkg_name.split('.')).value_mut() = Some(code);
            } else {
                mod_tree.root.value_mut().as_mut().unwrap().extend([code]);
            }
        }

        generate_mod_tree(&mut mod_tree.root)
    }

    pub(crate) fn generate_fdproto(&mut self, fdproto: &FileDescriptorProto) -> TokenStream {
        self.syntax = match fdproto.syntax.as_deref() {
            Some("proto3") => Syntax::Proto3,
            _ => Syntax::Proto2,
        };
        self.pkg_path = fdproto
            .package
            .as_ref()
            .map(|s| s.split('.').map(ToOwned::to_owned).collect())
            .unwrap_or_default();

        let root_node = &self.config_tree.root;
        let mut conf = root_node
            .value()
            .as_ref()
            .expect("root config should exist")
            .clone();
        let node = root_node.visit_path(
            fdproto.package.as_deref().unwrap_or("").split('.'),
            |next_conf| conf.merge(next_conf),
        );
        let cur_config = CurrentConfig {
            node,
            config: Cow::Owned(conf),
        };

        let msgs = fdproto
            .message_type
            .iter()
            .map(|m| self.generate_msg(m, cur_config.next_conf(m.name())));
        let enums = fdproto
            .enum_type
            .iter()
            .map(|e| self.generate_enum(e, cur_config.next_conf(e.name())));

        quote! {
            #(#msgs)*
            #(#enums)*
        }
    }

    fn generate_enum(
        &self,
        enum_type: &EnumDescriptorProto,
        enum_conf: CurrentConfig,
    ) -> TokenStream {
        if enum_conf.config.skip.unwrap_or(false) {
            return quote! {};
        }

        let name = Ident::new(enum_type.name(), Span::call_site());
        let nums = enum_type
            .value
            .iter()
            .map(|v| Literal::i32_unsuffixed(v.number()));
        let var_names = enum_type
            .value
            .iter()
            .map(|v| self.enum_variant_name(v.name(), &name));
        let default_num = Literal::i32_unsuffixed(enum_type.value[0].number.unwrap());
        let enum_int_type = enum_conf.config.enum_int_type.unwrap_or(IntType::I32);
        let itype = enum_int_type.type_name();
        let attrs = &enum_conf.config.type_attr_parsed();
        let derive_enum = derive_enum_attr(!enum_conf.config.no_debug_derive.unwrap_or(false));

        quote! {
            #derive_enum
            #[repr(transparent)]
            #(#attrs)*
            pub struct #name(pub #itype);

            impl #name {
                #(pub const #var_names: Self = Self(#nums);)*
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

    fn generate_msg_mod(
        &self,
        msg: &Message,
        proto: &DescriptorProto,
        msg_conf: &CurrentConfig,
    ) -> (TokenStream, Option<Vec<syn::Attribute>>) {
        let msg_mod_name = self.resolve_path_elem(msg.name);
        self.type_path.borrow_mut().push(msg.name.to_owned());

        let oneof_decls = msg.oneofs.iter().map(|oneof| oneof.generate_decl(self));
        let nested_msgs = proto
            .nested_type
            .iter()
            .filter(|m| !m.options.as_ref().map(|o| o.map_entry()).unwrap_or(false))
            .map(|m| self.generate_msg(m, msg_conf.next_conf(m.name())));
        let nested_enums = proto
            .enum_type
            .iter()
            .map(|e| self.generate_enum(e, msg_conf.next_conf(e.name())));

        let (hazzer_decl, hazzer_field_attr) =
            match msg.generate_hazzer_decl(msg_conf.next_conf("_has")) {
                Some((d, a)) => (Some(d), Some(a)),
                None => (None, None),
            };

        let msg_mod = quote! {
            pub mod #msg_mod_name {
                #(#nested_msgs)*
                #(#nested_enums)*
                #(#oneof_decls)*
                #hazzer_decl
            }
        };

        self.type_path.borrow_mut().pop();
        (msg_mod, hazzer_field_attr)
    }

    fn generate_msg(&self, proto: &DescriptorProto, msg_conf: CurrentConfig) -> TokenStream {
        let Some(msg) = Message::from_proto(proto, self.syntax, &msg_conf) else {
            return quote! {};
        };
        msg.check_delegates();
        let (msg_mod, hazzer_field_attr) = self.generate_msg_mod(&msg, proto, &msg_conf);
        let unknown_field_attr = msg_conf.next_conf("_unknown").config.field_attr_parsed();

        let default = msg.generate_default_impl(self, hazzer_field_attr.is_some());
        let decl = msg.generate_decl(self, hazzer_field_attr, unknown_field_attr);
        let msg_impl = msg.generate_impl(self);

        quote! {
            #msg_mod
            #decl
            #default
            #msg_impl
        }
    }

    fn resolve_type_name(&self, pb_fq_type_name: &str) -> TokenStream {
        // type names provided by protoc will always be fully-qualified
        assert_eq!(".", &pb_fq_type_name[..1]);

        let mut ident_path = pb_fq_type_name[1..].split('.');
        let ident_type = Ident::new(ident_path.next_back().unwrap(), Span::call_site());
        let mut ident_path = ident_path.peekable();

        let type_path = self.type_path.borrow();
        let mut local_path = self.pkg_path.iter().chain(type_path.iter()).peekable();

        // Skip path elements in common.
        while local_path.peek().is_some()
            && local_path.peek().map(|s| s.as_str()) == ident_path.peek().copied()
        {
            local_path.next();
            ident_path.next();
        }

        let path = local_path
            .map(|_| format_ident!("super"))
            .chain(ident_path.map(|e| self.resolve_path_elem(e)));
        quote! { #(#path ::)* #ident_type }
    }

    fn resolve_path_elem(&self, elem: &str) -> Ident {
        // Assume that type names all start with uppercase
        if elem.starts_with(|c: char| c.is_ascii_uppercase()) {
            format_ident!("mod_{elem}")
        } else {
            format_ident!("{elem}")
        }
    }

    /// Convert variant name to Pascal-case, then strip the enum name from it
    fn enum_variant_name(&self, variant_name: &str, enum_name: &Ident) -> Ident {
        let variant_name_cased = variant_name.to_case(Case::Pascal);
        let stripped = if !self.retain_enum_prefix {
            variant_name_cased
                .strip_prefix(&enum_name.to_string())
                .unwrap_or(&variant_name_cased)
        } else {
            &variant_name_cased
        };
        Ident::new(stripped, Span::call_site())
    }

    fn wrapped_type(&self, typ: TokenStream, boxed: bool, optional: bool) -> TokenStream {
        let boxed_type = if boxed {
            if self.use_std {
                quote! { ::std::boxed::Box<#typ> }
            } else {
                quote! { ::alloc::boxed::Box<#typ> }
            }
        } else {
            typ
        };
        if optional {
            quote! { ::core::option::Option<#boxed_type> }
        } else {
            boxed_type
        }
    }

    fn wrapped_value(&self, val: TokenStream, boxed: bool, optional: bool) -> TokenStream {
        let boxed_type = if boxed {
            if self.use_std {
                quote! { ::std::boxed::Box::new(#val) }
            } else {
                quote! { ::alloc::boxed::Box::new(#val) }
            }
        } else {
            val
        };
        if optional {
            quote! { ::core::option::Option::Some(#boxed_type) }
        } else {
            boxed_type
        }
    }

    fn fq_name(&self, name: &str) -> String {
        self.pkg_path
            .iter()
            .map(Deref::deref)
            .chain(self.type_path.borrow().iter().map(Deref::deref))
            .chain(iter::once(name))
            .fold(String::new(), |acc, s| acc + "." + s)
    }
}

#[cfg(test)]
mod tests {
    use prost_types::EnumValueDescriptorProto;

    use super::*;

    #[test]
    fn enum_variant_name() {
        let mut gen = Generator::default();
        let enum_name = Ident::new("Enum", Span::call_site());
        assert_eq!(
            gen.enum_variant_name("ENUM_VALUE", &enum_name).to_string(),
            "Value"
        );
        assert_eq!(
            gen.enum_variant_name("ALIEN", &enum_name).to_string(),
            "Alien"
        );

        gen.retain_enum_prefix = true;
        assert_eq!(
            gen.enum_variant_name("ENUM_VALUE", &enum_name).to_string(),
            "EnumValue"
        );
    }

    #[test]
    fn resolve_type_name() {
        let mut gen = Generator::default();
        // currently in root-level module
        assert_eq!(gen.resolve_type_name(".Message").to_string(), "Message");
        assert_eq!(
            gen.resolve_type_name(".package.Message").to_string(),
            quote! { package::Message }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".package.Message.Inner").to_string(),
            quote! { package::mod_Message::Inner }.to_string()
        );

        gen.pkg_path.push("package".to_owned());
        gen.type_path.borrow_mut().push("Message".to_owned());
        // currently in package::mod_Message module
        assert_eq!(
            gen.resolve_type_name(".Message").to_string(),
            quote! { super::super::Message }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".package.Message").to_string(),
            quote! { super::Message }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".Message.Item").to_string(),
            quote! { super::super::mod_Message::Item }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".package.Message.Inner").to_string(),
            "Inner"
        );
        assert_eq!(
            gen.resolve_type_name(".abc.d").to_string(),
            quote! { super::super::abc::d }.to_string()
        );
    }

    #[test]
    fn enum_basic() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(Config::new())),
        };
        let enum_proto = EnumDescriptorProto {
            name: Some("Test".to_owned()),
            value: vec![
                EnumValueDescriptorProto {
                    name: Some("TEST_ONE".to_owned()),
                    number: Some(1),
                    options: None,
                },
                EnumValueDescriptorProto {
                    name: Some("OTHER_VALUE".to_owned()),
                    number: Some(2),
                    options: None,
                },
            ],
            options: None,
            reserved_range: vec![],
            reserved_name: vec![],
        };
        let gen = Generator::default();

        let out = gen.generate_enum(&enum_proto, config);
        let expected = quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct Test(pub i32);

            impl Test {
                pub const One: Self = Self(1);
                pub const OtherValue: Self = Self(2);
            }

            impl core::default::Default for Test {
                fn default() -> Self {
                    Self(1)
                }
            }

            impl core::convert::From<i32> for Test {
                fn from(val: i32) -> Self {
                    Self(val)
                }
            }
        };
        assert_eq!(out.to_string(), expected.to_string());

        // skipped enums should generate nothing
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(Config::new().skip(true))),
        };
        assert!(gen.generate_enum(&enum_proto, config).is_empty())
    }

    #[test]
    fn enum_with_config() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(
                Config::new()
                    .enum_int_type(IntType::U8)
                    .type_attributes("#[derive(Serialize)]")
                    .no_debug_derive(true),
            )),
        };
        let enum_proto = EnumDescriptorProto {
            name: Some("Enum".to_owned()),
            value: vec![EnumValueDescriptorProto {
                name: Some("ENUM_ONE".to_owned()),
                number: Some(1),
                options: None,
            }],
            options: None,
            reserved_range: vec![],
            reserved_name: vec![],
        };
        let mut gen = Generator::default();
        gen.retain_enum_prefix = true;

        let out = gen.generate_enum(&enum_proto, config);
        let expected = quote! {
            #[derive(Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            #[derive(Serialize)]
            pub struct Enum(pub u8);

            impl Enum {
                pub const EnumOne: Self = Self(1);
            }

            impl core::default::Default for Enum {
                fn default() -> Self {
                    Self(1)
                }
            }

            impl core::convert::From<u8> for Enum {
                fn from(val: u8) -> Self {
                    Self(val)
                }
            }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }

    #[test]
    fn gen_mod_tree() {
        let mut mod_tree = PathTree::new(quote! { Root });
        *mod_tree
            .root
            .add_path(["foo", "bar"].into_iter())
            .value_mut() = Some(quote! { Bar });
        *mod_tree
            .root
            .add_path(["foo", "baz"].into_iter())
            .value_mut() = Some(quote! { Baz });
        *mod_tree.root.add_path(["bow"].into_iter()).value_mut() = Some(quote! { Bow });

        let out = generate_mod_tree(&mut mod_tree.root);
        let expected = quote! {
            Root

            pub mod foo {
                pub mod bar { Bar }
                pub mod baz { Baz }
            }

            pub mod bow { Bow }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }
}
