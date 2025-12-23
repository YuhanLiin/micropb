use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::BTreeMap,
    fmt::Display,
    io,
};

use convert_case::{Case, Casing};
use location::{CommentNode, Comments, add_location_comments, next_comment_node};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    EncodeDecode, Generator, WarningCb,
    config::Config,
    descriptor::{DescriptorProto, EnumDescriptorProto, FileDescriptorProto, FileDescriptorSet},
    generator::{r#enum::Enum, graph::TypeGraph},
    pathtree::{Node, PathTree},
    split_pkg_name,
};

use self::message::Message;

pub(crate) mod r#enum;
pub(crate) mod field;
mod graph;
pub(crate) mod location;
pub(crate) mod message;
pub(crate) mod oneof;
pub(crate) mod type_spec;

fn derive_msg_attr(
    debug: bool,
    default: bool,
    partial_eq: bool,
    clone: bool,
    copy: bool,
) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    let default = default.then(|| quote! { Default, });
    let partial_eq = partial_eq.then(|| quote! { PartialEq, });
    let copy = (clone && copy).then(|| quote! { Copy, });
    let clone = clone.then(|| quote! { Clone, });
    quote! { #[derive(#debug #default #partial_eq #clone #copy)] }
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

#[derive(Default)]
pub(crate) struct Params {
    pub(crate) extern_paths: BTreeMap<String, TokenStream>,
    pub(crate) encode_decode: EncodeDecode,
    pub(crate) calculate_max_size: bool,
    pub(crate) retain_enum_prefix: bool,
    pub(crate) suffixed_package_names: bool,
    pub(crate) single_oneof_msg_as_enum: bool,
}

pub(crate) struct Context<'proto> {
    // FileDescriptorSet-level context
    pub(crate) params: Params,
    pub(crate) graph: TypeGraph<'proto>,
    pub(crate) warning_cb: WarningCb,

    // File-level context
    pub(crate) syntax: Syntax,
    pub(crate) pkg_path: Vec<String>,
    pub(crate) pkg: String,
    pub(crate) type_path: RefCell<Vec<String>>,
}

impl<'proto> Context<'proto> {
    pub(crate) fn new(generator: Generator) -> (Self, PathTree<Box<Config>>) {
        let ctx = Self {
            params: Params {
                extern_paths: generator.extern_paths,
                encode_decode: generator.encode_decode,
                calculate_max_size: generator.calculate_max_size,
                retain_enum_prefix: generator.retain_enum_prefix,
                suffixed_package_names: generator.suffixed_package_names,
                single_oneof_msg_as_enum: generator.single_oneof_msg_as_enum,
            },
            warning_cb: generator.warning_cb,
            graph: TypeGraph::default(),

            syntax: Default::default(),
            pkg_path: Default::default(),
            pkg: Default::default(),
            type_path: Default::default(),
        };
        (ctx, generator.config_tree)
    }

    fn warn_unused_configs(&self, config_tree: &PathTree<Box<Config>>) {
        config_tree.find_all_unaccessed(|_node, path| {
            let path = path.join(".");
            (self.warning_cb)(format_args!("Unused configuration path: \"{path}\". Make sure the path points to an actual Protobuf type or module."));
        });
    }

    pub(crate) fn generate_fdset(
        generator: Generator,
        fdset: &'proto FileDescriptorSet,
    ) -> io::Result<TokenStream> {
        // Pre-generate the comment trees for every file
        let mut comment_trees = vec![];
        for file in &fdset.file {
            let mut comment_tree = PathTree::new(Comments::default());
            if let Some(src) = file.source_code_info() {
                for location in &src.location {
                    add_location_comments(&mut comment_tree, location);
                }
            }
            comment_trees.push(comment_tree);
        }

        let (mut ctx, config_tree) = Context::new(generator);

        // First, convert and accumulate all message and enum types
        for (file, comment_tree) in fdset.file.iter().zip(comment_trees.iter()) {
            ctx.add_fdproto(&config_tree, comment_tree, file)?;
        }

        // Resolve the type graph
        ctx.graph.resolve_all();

        // Generate Rust code
        let mut mod_tree = PathTree::new(TokenStream::new());
        for file in &fdset.file {
            let code = ctx.generate_fdproto(file)?;
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
                    .extend(code);
            }
        }

        let module = ctx.generate_mod_tree(&mut mod_tree.root);
        ctx.warn_unused_configs(&config_tree);
        Ok(module)
    }

    fn setup_file_context(&mut self, fdproto: &FileDescriptorProto) -> io::Result<()> {
        self.syntax = match fdproto.syntax.as_str() {
            "proto3" => Syntax::Proto3,
            "proto2" | "" => Syntax::Proto2,
            "editions" => return Err(io::Error::other("Protobuf Editions not supported")),
            syntax => {
                return Err(io::Error::other(format!(
                    "Unexpected Protobuf syntax specifier {syntax}"
                )));
            }
        };
        self.pkg_path = fdproto
            .package()
            .map(|s| split_pkg_name(s).map(ToOwned::to_owned).collect())
            .unwrap_or_default();
        self.pkg = fdproto.package().cloned().unwrap_or_default();
        self.type_path = RefCell::new(vec![]);
        Ok(())
    }

    fn add_fdproto(
        &mut self,
        config_tree: &PathTree<Box<Config>>,
        comment_tree: &'proto PathTree<Comments, (i32, i32)>,
        fdproto: &'proto FileDescriptorProto,
    ) -> io::Result<()> {
        self.setup_file_context(fdproto)?;

        let root_node = &config_tree.root;
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

        for (i, m) in fdproto.message_type.iter().enumerate() {
            self.add_message(
                m,
                cur_config.next_conf(&m.name),
                comment_tree.root.next(&location::path::fdset_msg(i)),
            )?;
        }
        for (i, e) in fdproto.enum_type.iter().enumerate() {
            self.add_enum(
                e,
                cur_config.next_conf(&e.name),
                comment_tree.root.next(&location::path::fdset_enum(i)),
            )?;
        }

        Ok(())
    }

    fn generate_fdproto(&mut self, fdproto: &FileDescriptorProto) -> io::Result<TokenStream> {
        self.setup_file_context(fdproto)?;

        // Generate Rust code from message and enum types
        let mut out = TokenStream::new();
        for proto in fdproto.message_type.iter() {
            let m = self.graph.get_message(&self.fq_proto_name(&proto.name));
            out.extend(self.generate_msg(m, proto)?);
        }
        for proto in fdproto.enum_type.iter() {
            let e = self.graph.get_enum(&self.fq_proto_name(&proto.name));
            out.extend(self.generate_enum(e));
        }

        Ok(out)
    }

    fn generate_mod_tree(&self, mod_node: &mut Node<TokenStream>) -> TokenStream {
        let code = mod_node.value_mut().take().unwrap_or_default();
        let submods = mod_node.children_mut().map(|(submod_name, inner_node)| {
            let submod_name = resolve_path_elem(submod_name, self.params.suffixed_package_names);
            let inner = self.generate_mod_tree(inner_node);
            quote! { pub mod #submod_name { #inner } }
        });

        quote! {
            #code
            #(#submods)*
        }
    }

    fn generate_enum(&self, e: Option<&Enum>) -> TokenStream {
        // None means enum has been skipped
        let Some(e) = e else { return quote! {} };
        e.generate_decl()
    }

    fn generate_msg_mod(&self, msg: &Message, proto: &DescriptorProto) -> io::Result<TokenStream> {
        let msg_mod_name = resolve_path_elem(msg.name, self.params.suffixed_package_names);

        self.type_path.borrow_mut().push(msg.name.to_owned());
        let mut msg_mod_body = TokenStream::new();
        for m in proto
            .nested_type
            .iter()
            .filter(|m| !m.options().map(|o| o.map_entry).unwrap_or(false))
        {
            let sub_msg_fq_name = self.fq_proto_name(&m.name);
            let sub_msg = self.graph.get_message(&sub_msg_fq_name);
            msg_mod_body.extend(self.generate_msg(sub_msg, m)?);
        }
        for e in proto.enum_type.iter() {
            let enum_fq_name = self.fq_proto_name(&e.name);
            let e = self.graph.get_enum(&enum_fq_name);
            msg_mod_body.extend(self.generate_enum(e));
        }

        if !msg.as_oneof_enum {
            for o in &msg.oneofs {
                msg_mod_body.extend(o.generate_decl(self, msg.is_copy));
            }
        }

        msg_mod_body.extend(msg.generate_hazzer_decl());

        self.type_path.borrow_mut().pop();

        let msg_mod = if msg_mod_body.is_empty() {
            quote! {}
        } else {
            let doc = format!(" Inner types for `{}`", msg.name);
            quote! { #[doc = #doc] pub mod #msg_mod_name { #msg_mod_body } }
        };
        Ok(msg_mod)
    }

    fn add_message(
        &mut self,
        proto: &'proto DescriptorProto,
        msg_conf: CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
    ) -> io::Result<()> {
        let fq_name = self.fq_proto_name(&proto.name);
        if self.params.extern_paths.contains_key(&fq_name) {
            return Ok(());
        }
        let Some(msg) = Message::from_proto(proto, self, &msg_conf, comment_node)? else {
            return Ok(());
        };
        let msg_name = msg.name;
        self.graph.add_message(fq_name, msg);

        self.type_path.borrow_mut().push(msg_name.to_owned());
        for (i, m) in proto
            .nested_type
            .iter()
            .enumerate()
            .filter(|(_, m)| !m.options().map(|o| o.map_entry).unwrap_or(false))
        {
            self.add_message(
                m,
                msg_conf.next_conf(&m.name),
                next_comment_node(comment_node, location::path::msg_msg(i)),
            )?;
        }
        for (i, e) in proto.enum_type.iter().enumerate() {
            self.add_enum(
                e,
                msg_conf.next_conf(&e.name),
                next_comment_node(comment_node, location::path::msg_enum(i)),
            )?;
        }
        self.type_path.borrow_mut().pop();

        Ok(())
    }

    fn add_enum(
        &mut self,
        proto: &'proto EnumDescriptorProto,
        enum_conf: CurrentConfig,
        comment_node: Option<&'proto CommentNode>,
    ) -> io::Result<()> {
        let fq_name = self.fq_proto_name(&proto.name);
        if self.params.extern_paths.contains_key(&fq_name) {
            return Ok(());
        }
        let Some(e) = Enum::from_proto(proto, self, &enum_conf, comment_node)? else {
            return Ok(());
        };
        self.graph.add_enum(fq_name, e);
        Ok(())
    }

    fn generate_msg(
        &self,
        msg: Option<&Message>,
        proto: &DescriptorProto,
    ) -> io::Result<TokenStream> {
        // None means message has been skipped
        let Some(msg) = msg else { return Ok(quote! {}) };

        let msg_mod = self.generate_msg_mod(msg, proto)?;
        let proto_default = msg.fields.iter().any(|f| f.default.is_some());

        // Only manually implement Default if there's a Protobuf default specification
        let default = proto_default
            .then(|| msg.generate_default_impl(self))
            .transpose()?;
        // Only manually implement PartialEq if there's a hazzer
        let partial_eq = msg.hazzer.as_ref().map(|_| msg.generate_partial_eq());
        let decl = msg.generate_decl(self, proto_default)?;
        let msg_impl = msg.generate_impl(self);
        let decode = self
            .params
            .encode_decode
            .is_decode()
            .then(|| msg.generate_decode_trait(self));
        let encode = self
            .params
            .encode_decode
            .is_encode()
            .then(|| msg.generate_encode_trait(self));

        Ok(quote! {
            #decl
            #default
            #partial_eq
            #msg_impl
            #decode
            #encode
            #msg_mod
        })
    }

    fn resolve_type_name(&self, pb_fq_type_name: &str) -> TokenStream {
        // Type names provided by protoc will always be fully-qualified
        assert_eq!(".", &pb_fq_type_name[..1]);

        // Check if we're substituting with an extern type
        if let Some(rust_type) = self.params.extern_paths.get(pb_fq_type_name) {
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

        let path = local_path.map(|_| format_ident!("super")).chain(
            ident_path.map(|elem| resolve_path_elem(elem, self.params.suffixed_package_names)),
        );
        quote! { #(#path ::)* #ident_type }
    }

    fn fq_proto_name(&self, proto_name: &str) -> String {
        let pkg_path = &self.pkg_path;
        let type_path = self.type_path.borrow();

        let mut fq_proto_name = String::new();
        for elem in pkg_path.iter().chain(type_path.iter()) {
            fq_proto_name.push('.');
            fq_proto_name.push_str(elem);
        }
        fq_proto_name.push('.');
        fq_proto_name.push_str(proto_name);
        fq_proto_name
    }

    /// Convert variant name to Pascal-case, then strip the enum name from it
    fn enum_variant_name(&self, variant_name: &str, enum_name: &Ident) -> Ident {
        let variant_name_cased = variant_name.to_case(Case::Pascal);
        let stripped = if !self.params.retain_enum_prefix {
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
pub(crate) fn resolve_path_elem(elem: &str, suffixed: bool) -> Ident {
    if suffixed || matches!(elem, "super" | "crate" | "self" | "Self" | "extern") {
        // Add underscore suffix
        format_ident!("{elem}_")
    } else {
        Ident::new_raw(elem, Span::call_site())
    }
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
fn make_ctx() -> Context<'static> {
    Context {
        params: Params::default(),
        graph: TypeGraph::default(),
        syntax: Default::default(),
        pkg_path: Default::default(),
        pkg: Default::default(),
        type_path: Default::default(),
        warning_cb: |_| {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_variant_name() {
        let mut ctx = make_ctx();
        let enum_name = Ident::new("Enum", Span::call_site());
        assert_eq!(
            ctx.enum_variant_name("ENUM_VALUE", &enum_name).to_string(),
            "Value"
        );
        assert_eq!(
            ctx.enum_variant_name("ALIEN", &enum_name).to_string(),
            "Alien"
        );

        ctx.params.retain_enum_prefix = true;
        assert_eq!(
            ctx.enum_variant_name("ENUM_VALUE", &enum_name).to_string(),
            "EnumValue"
        );
    }

    #[test]
    fn resolve_type_name() {
        let mut ctx = make_ctx();
        ctx.params.suffixed_package_names = true;
        // currently in root-level module
        assert_eq!(ctx.resolve_type_name(".Message").to_string(), "Message");
        assert_eq!(
            ctx.resolve_type_name(".package.Message").to_string(),
            quote! { package_::Message }.to_string()
        );
        assert_eq!(
            ctx.resolve_type_name(".package.Message.Inner").to_string(),
            quote! { package_::Message_::Inner }.to_string()
        );

        ctx.pkg_path.push("package".to_owned());
        ctx.type_path.borrow_mut().push("Message".to_owned());
        // currently in package::mod_Message module
        assert_eq!(
            ctx.resolve_type_name(".Message").to_string(),
            quote! { super::super::Message }.to_string()
        );
        assert_eq!(
            ctx.resolve_type_name(".package.Message").to_string(),
            quote! { super::Message }.to_string()
        );
        assert_eq!(
            ctx.resolve_type_name(".Message.Item").to_string(),
            quote! { super::super::Message_::Item }.to_string()
        );
        assert_eq!(
            ctx.resolve_type_name(".package.Message.Inner").to_string(),
            "Inner"
        );
        assert_eq!(
            ctx.resolve_type_name(".abc.d").to_string(),
            quote! { super::super::abc_::r#d }.to_string()
        );
    }

    #[test]
    fn gen_mod_tree() {
        let mk_tree = || {
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
            mod_tree
        };

        let mut ctx = make_ctx();

        ctx.params.suffixed_package_names = true;
        let mut mod_tree = mk_tree();
        let out = ctx.generate_mod_tree(&mut mod_tree.root);
        let expected = quote! {
            Root

            pub mod foo_ {
                pub mod bar_ { Bar }
                pub mod baz_ { Baz }
            }

            pub mod bow_ { Bow }
        };
        assert_eq!(out.to_string(), expected.to_string());

        ctx.params.suffixed_package_names = false;
        let mut mod_tree = mk_tree();
        let out = ctx.generate_mod_tree(&mut mod_tree.root);
        let expected = quote! {
            Root

            pub mod r#foo {
                pub mod r#bar { Bar }
                pub mod r#baz { Baz }
            }

            pub mod r#bow { Bow }
        };
        assert_eq!(out.to_string(), expected.to_string());
    }
}
