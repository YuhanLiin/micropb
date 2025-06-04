use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::HashMap,
    ffi::OsString,
    fmt::Display,
    io,
    path::PathBuf,
};

use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Ident};

use crate::{
    config::{Config, IntSize},
    descriptor::{
        DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FileDescriptorProto,
        FileDescriptorSet,
    },
    pathtree::{Node, PathTree},
    split_pkg_name, EncodeDecode,
};

use self::message::Message;
use super::WarningCb;

pub(crate) mod field;
pub(crate) mod message;
pub(crate) mod oneof;
pub(crate) mod type_spec;

fn derive_msg_attr(debug: bool, default: bool, partial_eq: bool, clone: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    let default = default.then(|| quote! { Default, });
    let partial_eq = partial_eq.then(|| quote! { PartialEq, });
    let clone = clone.then(|| quote! { Clone, });
    quote! { #[derive(#debug #default #partial_eq #clone)] }
}

fn derive_enum_attr() -> TokenStream {
    quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] }
}

pub(crate) struct CurrentConfig<'a> {
    node: Option<&'a Node<Box<Config>>>,
    config: Cow<'a, Box<Config>>,
}

impl<'a> CurrentConfig<'a> {
    fn next_conf(&'a self, segment: &str) -> Self {
        let mut config: Cow<Box<Config>> = Cow::Borrowed(self.config.borrow());
        if let Some(node) = self.node {
            let next = node.next(segment);
            if let Some(conf) = next.and_then(|n| n.access_value().as_ref()) {
                (*config.to_mut()).merge(conf);
            }
            Self { node: next, config }
        } else {
            Self { node: None, config }
        }
    }

    fn derive_dbg(&self) -> bool {
        !self.config.no_debug_impl.unwrap_or(false)
    }

    fn impl_default(&self) -> bool {
        !self.config.no_default_impl.unwrap_or(false)
    }

    fn derive_partial_eq(&self) -> bool {
        !self.config.no_partial_eq_impl.unwrap_or(false)
    }

    fn derive_clone(&self) -> bool {
        !self.config.no_clone_impl.unwrap_or(false)
    }
}

fn generate_mod_tree(mod_node: &mut Node<TokenStream>) -> TokenStream {
    let code = mod_node.value_mut().take().unwrap_or_default();
    let submods = mod_node.children_mut().map(|(submod_name, inner_node)| {
        let submod_name = resolve_path_elem(submod_name);
        let inner = generate_mod_tree(inner_node);
        quote! { pub mod #submod_name { #inner } }
    });

    quote! {
        #code
        #(#submods)*
    }
}

fn field_error(pkg: &str, msg_name: &str, field_name: &str, err_text: impl Display) -> io::Error {
    let dot = if pkg.is_empty() { "" } else { "." };
    io::Error::other(format!("({dot}{pkg}.{msg_name}.{field_name}) {err_text}"))
}

fn msg_error(pkg: &str, msg_name: &str, err_text: impl Display) -> io::Error {
    let dot = if pkg.is_empty() { "" } else { "." };
    io::Error::other(format!("({dot}{pkg}.{msg_name}) {err_text}"))
}

pub(crate) enum EncodeFunc {
    Sizeof(Ident),
    Encode(Ident),
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

#[derive(Debug)]
/// Protobuf code generator
///
/// Use this in `build.rs` to compile `.proto` files into a Rust module.
///
/// The main way to control the compilation process is to call [`configure`](Generator::configure),
/// which allows the user to customize how code is generated from Protobuf types and fields of
/// their choosing.
///
/// # Note
/// It's recommended to call one of [`use_container_alloc`](Self::use_container_alloc),
/// [`use_container_heapless`](Self::use_container_heapless), or
/// [`use_container_alloc`](Self::use_container_alloc) to ensure that container types are
/// configured for `string`, `bytes`, repeated, and `map` fields. The generator will throw an
/// error if it reaches any such field that doesn't have a container configured.
///
/// # Example
/// ```no_run
/// use micropb_gen::{Generator, Config};
///
/// let mut gen = Generator::new();
/// // Use container types from `heapless`
/// gen.use_container_heapless()
///     // Set max length of repeated fields in .test.Data to 4
///     .configure(".test.Data", Config::new().max_len(4))
///     // Wrap .test.Data.value inside a Box
///     .configure(".test.Data.value", Config::new().boxed(true))
///     // Compile test.proto into a Rust module
///     .compile_protos(
///         &["test.proto"],
///         std::env::var("OUT_DIR").unwrap() + "/test_proto.rs",
///     )
///     .unwrap();
/// ```
pub struct Generator {
    pub(crate) syntax: Syntax,
    pub(crate) pkg_path: Vec<String>,
    pub(crate) pkg: String,
    pub(crate) type_path: RefCell<Vec<String>>,

    pub(crate) warning_cb: WarningCb,

    pub(crate) encode_decode: EncodeDecode,
    pub(crate) retain_enum_prefix: bool,
    pub(crate) format: bool,
    pub(crate) fdset_path: Option<PathBuf>,
    pub(crate) protoc_args: Vec<OsString>,

    pub(crate) config_tree: PathTree<Box<Config>>,
    pub(crate) extern_paths: HashMap<String, TokenStream>,
}

impl Generator {
    pub(crate) fn warn_unused_configs(&self) {
        self.config_tree.find_all_unaccessed(|_node, path| {
            let path = path.join(".");
            (self.warning_cb)(format_args!("Unused configuration path: \"{path}\". Make sure the path points to an actual Protobuf type or module."));
        });
    }

    pub(crate) fn generate_fdset(&mut self, fdset: &FileDescriptorSet) -> io::Result<TokenStream> {
        let mut mod_tree = PathTree::new(TokenStream::new());

        for file in &fdset.file {
            let code = self.generate_fdproto(file)?;
            if let Some(pkg_name) = file.package() {
                mod_tree
                    .root
                    .add_path(split_pkg_name(pkg_name))
                    .value_mut()
                    .get_or_insert_with(TokenStream::new)
                    .extend(code);
            } else {
                mod_tree
                    .root
                    .value_mut()
                    .as_mut()
                    .expect("root config should exist")
                    .extend([code]);
            }
        }

        Ok(generate_mod_tree(&mut mod_tree.root))
    }

    pub(crate) fn generate_fdproto(
        &mut self,
        fdproto: &FileDescriptorProto,
    ) -> io::Result<TokenStream> {
        self.syntax = match fdproto.syntax.as_str() {
            "proto3" => Syntax::Proto3,
            "proto2" | "" => Syntax::Proto2,
            "editions" => return Err(io::Error::other("Protobuf Editions not supported")),
            syntax => {
                return Err(io::Error::other(format!(
                    "Unexpected Protobuf syntax specifier {syntax}"
                )))
            }
        };
        self.pkg_path = fdproto
            .package()
            .map(|s| split_pkg_name(s).map(ToOwned::to_owned).collect())
            .unwrap_or_default();
        self.pkg = fdproto.package().cloned().unwrap_or_default();

        let root_node = &self.config_tree.root;
        let mut conf = root_node
            .access_value()
            .as_ref()
            .expect("root config should exist")
            .clone();
        let node = root_node.visit_path(
            split_pkg_name(fdproto.package().map(String::as_str).unwrap_or("")),
            |next_conf| conf.merge(next_conf),
        );
        let cur_config = CurrentConfig {
            node,
            config: Cow::Owned(conf),
        };

        let mut out = TokenStream::new();
        for m in &fdproto.message_type {
            out.extend(self.generate_msg(m, cur_config.next_conf(&m.name))?);
        }
        for e in &fdproto.enum_type {
            out.extend(self.generate_enum(e, cur_config.next_conf(&e.name))?);
        }

        Ok(out)
    }

    fn generate_enum_decl(
        &self,
        name: &Ident,
        values: &[EnumValueDescriptorProto],
        enum_int_type: IntSize,
        attrs: &[Attribute],
    ) -> TokenStream {
        let nums = values.iter().map(|v| Literal::i32_unsuffixed(v.number));
        let var_names = values.iter().map(|v| self.enum_variant_name(&v.name, name));
        let default_num = Literal::i32_unsuffixed(values[0].number);
        let derive_enum = derive_enum_attr();
        let itype = enum_int_type.type_name(true);

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

    fn generate_enum(
        &self,
        enum_type: &EnumDescriptorProto,
        enum_conf: CurrentConfig,
    ) -> io::Result<TokenStream> {
        if enum_conf.config.skip.unwrap_or(false) {
            return Ok(quote! {});
        }

        let name = sanitized_ident(&enum_type.name);
        let enum_int_type = enum_conf.config.enum_int_size.unwrap_or(IntSize::S32);
        let attrs = &enum_conf
            .config
            .type_attr_parsed()
            .map_err(|e| msg_error(&self.pkg, &enum_type.name, &e))?;
        let out = self.generate_enum_decl(&name, &enum_type.value, enum_int_type, attrs);
        Ok(out)
    }

    fn generate_msg_mod(
        &self,
        msg: &Message,
        proto: &DescriptorProto,
        msg_conf: &CurrentConfig,
    ) -> io::Result<(TokenStream, Option<Vec<syn::Attribute>>)> {
        let msg_mod_name = resolve_path_elem(msg.name);
        self.type_path.borrow_mut().push(msg.name.to_owned());

        let mut msg_mod_body = TokenStream::new();
        for m in proto
            .nested_type
            .iter()
            .filter(|m| !m.options().map(|o| o.map_entry).unwrap_or(false))
        {
            msg_mod_body.extend(self.generate_msg(m, msg_conf.next_conf(&m.name))?);
        }
        for e in proto.enum_type.iter() {
            msg_mod_body.extend(self.generate_enum(e, msg_conf.next_conf(&e.name))?);
        }
        for o in &msg.oneofs {
            msg_mod_body.extend(o.generate_decl(self));
        }

        let (hazzer_decl, hazzer_field_attr) = match msg
            .generate_hazzer_decl(msg_conf.next_conf("_has"))
            .map_err(|e| field_error(&self.pkg, msg.name, "_has", &e))?
        {
            Some((d, a)) => (Some(d), Some(a)),
            None => (None, None),
        };
        msg_mod_body.extend(hazzer_decl);

        self.type_path.borrow_mut().pop();

        let msg_mod = if msg_mod_body.is_empty() {
            quote! {}
        } else {
            quote! { pub mod #msg_mod_name { #msg_mod_body } }
        };
        Ok((msg_mod, hazzer_field_attr))
    }

    fn generate_msg(
        &self,
        proto: &DescriptorProto,
        msg_conf: CurrentConfig,
    ) -> io::Result<TokenStream> {
        let Some(msg) = Message::from_proto(proto, self, &msg_conf)? else {
            return Ok(quote! {});
        };
        let (msg_mod, hazzer_field_attr) = self.generate_msg_mod(&msg, proto, &msg_conf)?;
        let unknown_conf = msg_conf.next_conf("_unknown");

        let default = msg.generate_default_impl(self, hazzer_field_attr.is_some())?;
        let partial_eq = msg.generate_partial_eq();
        let decl = msg.generate_decl(self, hazzer_field_attr, &unknown_conf)?;
        let msg_impl = msg.generate_impl(self);
        let decode = self
            .encode_decode
            .is_decode()
            .then(|| msg.generate_decode_trait(self));
        let encode = self
            .encode_decode
            .is_encode()
            .then(|| msg.generate_encode_trait(self));

        Ok(quote! {
            #msg_mod
            #decl
            #default
            #partial_eq
            #msg_impl
            #decode
            #encode
        })
    }

    fn resolve_type_name(&self, pb_fq_type_name: &str) -> TokenStream {
        // Type names provided by protoc will always be fully-qualified
        assert_eq!(".", &pb_fq_type_name[..1]);

        // Check if we're substituting with an extern type
        if let Some(rust_type) = self.extern_paths.get(pb_fq_type_name) {
            return rust_type.clone();
        }

        let mut ident_path = pb_fq_type_name[1..].split('.');
        let ident_type = sanitized_ident(ident_path.next_back().unwrap());
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
            .chain(ident_path.map(resolve_path_elem));
        quote! { #(#path ::)* #ident_type }
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
        sanitized_ident(stripped)
    }

    fn wrapped_type(&self, typ: TokenStream, boxed: bool, optional: bool) -> TokenStream {
        let boxed_type = if boxed {
            quote! { ::alloc::boxed::Box<#typ> }
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
            quote! { ::alloc::boxed::Box::new(#val) }
        } else {
            val
        };
        if optional {
            quote! { ::core::option::Option::Some(#boxed_type) }
        } else {
            boxed_type
        }
    }
}

#[inline]
pub(crate) fn resolve_path_elem(elem: &str) -> Ident {
    // Add underscore suffix
    format_ident!("{elem}_")
}

#[inline]
pub(crate) fn sanitized_ident(name: &str) -> Ident {
    match name {
        // These keywords can't be raw idents, so prefix with underscore
        "_" | "super" | "crate" | "self" | "Self" | "extern" => {
            format_ident!("_{name}")
        }
        // Idents can't start with numbers, so prefix with underscore
        name if name.starts_with(|c: char| c.is_numeric()) => {
            format_ident!("_{name}")
        }
        // Use raw idents for other lowercase names, since they may be keywords
        name if name.starts_with(|c: char| c.is_lowercase()) => {
            Ident::new_raw(name, Span::call_site())
        }
        // Use the name as is
        name => Ident::new(name, Span::call_site()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::parse_attributes, descriptor::EnumValueDescriptorProto};

    use super::*;

    #[test]
    fn enum_variant_name() {
        let mut gen = Generator::new();
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
        let mut gen = Generator::new();
        // currently in root-level module
        assert_eq!(gen.resolve_type_name(".Message").to_string(), "Message");
        assert_eq!(
            gen.resolve_type_name(".package.Message").to_string(),
            quote! { package_::Message }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".package.Message.Inner").to_string(),
            quote! { package_::Message_::Inner }.to_string()
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
            quote! { super::super::Message_::Item }.to_string()
        );
        assert_eq!(
            gen.resolve_type_name(".package.Message.Inner").to_string(),
            "Inner"
        );
        assert_eq!(
            gen.resolve_type_name(".abc.d").to_string(),
            quote! { super::super::abc_::r#d }.to_string()
        );
    }

    #[test]
    fn enum_basic() {
        let name = Ident::new("Test", Span::call_site());
        let mut value = vec![
            EnumValueDescriptorProto::default(),
            EnumValueDescriptorProto::default(),
        ];
        value[0].set_name("TEST_ONE".to_owned());
        value[0].set_number(1);
        value[1].set_name("OTHER_VALUE".to_owned());
        value[1].set_number(2);
        let gen = Generator::new();

        let out = gen.generate_enum_decl(&name, &value, IntSize::S32, &[]);
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
    }

    #[test]
    fn enum_with_config() {
        let name = Ident::new("Enum", Span::call_site());
        let mut value = vec![EnumValueDescriptorProto::default()];
        value[0].set_name("ENUM_ONE".to_owned());
        value[0].set_number(1);
        let gen = Generator {
            retain_enum_prefix: true,
            ..Generator::new()
        };

        let out = gen.generate_enum_decl(
            &name,
            &value,
            IntSize::S8,
            &parse_attributes("#[derive(Serialize)]").unwrap(),
        );
        let expected = quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            #[derive(Serialize)]
            pub struct Enum(pub i8);

            impl Enum {
                pub const EnumOne: Self = Self(1);
            }

            impl core::default::Default for Enum {
                fn default() -> Self {
                    Self(1)
                }
            }

            impl core::convert::From<i8> for Enum {
                fn from(val: i8) -> Self {
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

            pub mod foo_ {
                pub mod bar_ { Bar }
                pub mod baz_ { Baz }
            }

            pub mod bow_ { Bow }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }
}
