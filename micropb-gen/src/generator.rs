use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::HashMap,
    iter,
    ops::Deref,
};

use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use protox::prost_reflect::prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    config::{Config, IntType},
    pathtree::{Node, PathTree},
    utils::{path_suffix, unescape_c_escape_string},
};

fn derive_msg_attr(debug: bool, default: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    let default = default.then(|| quote! { Default, });
    quote! { #[derive(#debug #default Clone, PartialEq)] }
}

fn derive_enum_attr(debug: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    quote! { #[derive(#debug Clone, Copy, PartialEq, Eq, Hash)] }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

enum PbInt {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sfixed32,
    Sint64,
    Sfixed64,
    Fixed32,
    Fixed64,
}

enum TypeSpec {
    Message(String),
    Enum(String),
    Float,
    Double,
    Bool,
    Int(PbInt, IntType),
    String {
        type_path: syn::Path,
        max_bytes: Option<u32>,
    },
    Bytes {
        type_path: syn::Path,
        max_bytes: Option<u32>,
    },
}

pub(crate) enum CustomField {
    Type(syn::Type),
    Delegate(Ident),
}

enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec,
        val: TypeSpec,
        packed: bool,
        type_path: syn::Path,
        max_len: Option<u32>,
    },
    // Implicit presence
    Single(TypeSpec),
    // Explicit presence
    Optional(TypeSpec),
    Repeated {
        typ: TypeSpec,
        packed: bool,
        type_path: syn::Path,
        max_len: Option<u32>,
    },
    Custom(CustomField),
}

struct Field<'a> {
    num: u32,
    ftype: FieldType,
    name: &'a str,
    rust_name: Ident,
    default: Option<&'a str>,
    oneof: Option<usize>,
    boxed: bool,
    no_hazzer: bool,
    attrs: TokenStream,
}

impl<'a> Field<'a> {
    fn explicit_presence(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_)) && self.oneof.is_none()
    }

    fn is_hazzer(&self) -> bool {
        self.explicit_presence() && !self.boxed && !self.no_hazzer && self.oneof.is_none()
    }

    fn delegate(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Delegate(d)) = &self.ftype {
            Some(d)
        } else {
            None
        }
    }

    fn custom_type_field(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Type(_)) = &self.ftype {
            Some(&self.rust_name)
        } else {
            None
        }
    }
}

enum OneofType<'a> {
    Enum {
        type_name: Ident,
        fields: Vec<Field<'a>>,
    },
    Custom(CustomField),
}

struct Oneof<'a> {
    name: &'a str,
    rust_name: Ident,
    otype: OneofType<'a>,
    boxed: bool,
    field_attrs: TokenStream,
    type_attrs: TokenStream,
    derive_dbg: bool,
    idx: usize,
}

impl<'a> Oneof<'a> {
    fn delegate(&self) -> Option<&Ident> {
        if let OneofType::Custom(CustomField::Delegate(d)) = &self.otype {
            Some(d)
        } else {
            None
        }
    }

    fn custom_type_field(&self) -> Option<&Ident> {
        if let OneofType::Custom(CustomField::Type(_)) = &self.otype {
            Some(&self.rust_name)
        } else {
            None
        }
    }
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

#[derive(Debug, Default)]
pub struct GenConfig {
    pub(crate) encode_decode: EncodeDecode,
    pub(crate) size_cache: bool,
    pub(crate) default_pkg_filename: String,
    pub(crate) retain_enum_prefix: bool,
    pub(crate) format: bool,
    pub(crate) use_std: bool,

    pub(crate) field_configs: PathTree<Box<Config>>,
}

#[derive(Debug, Default)]
struct Generator {
    config: GenConfig,
    syntax: Syntax,
    pkg_path: Vec<String>,
    type_path: RefCell<Vec<String>>,
}

impl Generator {
    fn generate_fdset(&mut self, fdset: &FileDescriptorSet) {
        for file in &fdset.file {
            self.generate_fdproto(file);
        }
    }

    fn generate_fdproto(&mut self, fdproto: &FileDescriptorProto) {
        let filename = fdproto
            .package
            .as_ref()
            .unwrap_or(&self.config.default_pkg_filename)
            .to_owned();

        self.syntax = match fdproto.syntax.as_deref() {
            Some("proto3") => Syntax::Proto3,
            _ => Syntax::Proto2,
        };
        self.pkg_path = fdproto
            .package
            .as_ref()
            .map(|s| s.split('.').map(ToOwned::to_owned).collect())
            .unwrap_or_default();

        let root_node = &self.config.field_configs.root;
        let mut root_conf = root_node
            .value()
            .as_ref()
            .expect("root config should exist")
            .clone();
        root_node.visit_path(
            fdproto.package.as_deref().unwrap_or("").split('.'),
            |conf| root_conf.merge(conf),
        );
        let cur_config = CurrentConfig {
            node: Some(root_node),
            config: Cow::Owned(root_conf),
        };

        let msgs = fdproto
            .message_type
            .iter()
            .map(|m| self.generate_msg_type(m, cur_config.next_conf(m.name())));
        let enums = fdproto
            .enum_type
            .iter()
            .map(|e| self.generate_enum_type(e, cur_config.next_conf(e.name())));

        let code = quote! {
            #(#msgs)*
            #(#enums)*
        };
    }

    fn generate_enum_type(
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
            #attrs
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

    fn generate_msg_type(
        &self,
        msg_type: &DescriptorProto,
        msg_conf: CurrentConfig,
    ) -> TokenStream {
        if msg_conf.config.skip.unwrap_or(false) {
            return quote! {};
        }

        let name = msg_type.name.as_ref().unwrap();
        let msg_mod_name = format_ident!("mod_{name}");
        let mut oneofs: Vec<_> = msg_type
            .oneof_decl
            .iter()
            .enumerate()
            .filter_map(|(idx, oneof)| {
                self.create_oneof(idx, oneof, msg_conf.next_conf(oneof.name()))
            })
            .collect();
        let mut map_types = HashMap::new();
        let inner_msgs: Vec<_> = msg_type
            .nested_type
            .iter()
            .filter(|m| {
                if m.options.as_ref().map(|o| o.map_entry()).unwrap_or(false) {
                    map_types.insert(m.name(), *m);
                    false
                } else {
                    true
                }
            })
            .collect();

        let fields: Vec<_> = msg_type
            .field
            .iter()
            .filter_map(|f| {
                let field_conf = msg_conf.next_conf(f.name());
                let raw_msg_name = f
                    .type_name()
                    .rsplit_once('.')
                    .map(|(_, r)| r)
                    .unwrap_or(f.type_name());
                if let Some(map_msg) = map_types.remove(raw_msg_name) {
                    self.create_map_field(f, map_msg, field_conf)
                } else {
                    if let Some(idx) = f.oneof_index {
                        if let Some(OneofType::Enum { fields, .. }) = oneofs
                            .iter_mut()
                            .find(|o| o.idx == idx as usize)
                            .map(|o| &mut o.otype)
                        {
                            if let Some(field) = self.create_field(f, field_conf) {
                                fields.push(field);
                            }
                        }
                        return None;
                    }
                    self.create_field(f, field_conf)
                }
            })
            .collect();

        // Remove all oneofs that are empty enums
        let oneofs: Vec<_> = oneofs
            .into_iter()
            .filter(|o| !matches!(&o.otype, OneofType::Enum { fields, .. } if fields.is_empty()))
            .collect();

        let odelegates = oneofs.iter().filter_map(|o| o.delegate());
        let fdelegates = fields.iter().filter_map(|f| f.delegate());
        for delegate in odelegates.chain(fdelegates) {
            let ocustoms = oneofs.iter().filter_map(|o| o.custom_type_field());
            let fcustoms = fields.iter().filter_map(|f| f.custom_type_field());
            if ocustoms.chain(fcustoms).any(|custom| delegate == custom) {
                // TODO error about how delegate != custom
            }
        }

        self.type_path.borrow_mut().push(name.to_owned());

        let oneof_decls = oneofs.iter().map(|oneof| self.generate_oneof_decl(oneof));
        let nested_msgs = inner_msgs
            .iter()
            .map(|m| self.generate_msg_type(m, msg_conf.next_conf(m.name())));
        let nested_enums = msg_type
            .enum_type
            .iter()
            .map(|e| self.generate_enum_type(e, msg_conf.next_conf(e.name())));

        let (hazzer_decl, hazzer_field_attr) =
            match self.generate_hazzer_decl(&fields, msg_conf.next_conf("_has")) {
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

        let rust_name = Ident::new(name, Span::call_site());
        let msg_fields = fields.iter().map(|f| self.generate_field_decl(f));
        let hazzer_field =
            hazzer_field_attr.map(|attr| quote! { #attr pub _has: #msg_mod_name::_Hazzer, });
        let oneof_fields = oneofs
            .iter()
            .map(|oneof| self.generate_oneof_field_decl(&msg_mod_name, oneof));

        let derive_msg = derive_msg_attr(!msg_conf.config.no_debug_derive.unwrap_or(false), true);
        let attrs = &msg_conf.config.type_attr_parsed();

        quote! {
            #msg_mod

            #derive_msg
            #attrs
            pub struct #rust_name {
                #(pub #msg_fields)*
                #(pub #oneof_fields)*
                #hazzer_field
            }
        }
    }

    fn generate_oneof_decl(&self, oneof: &Oneof) -> TokenStream {
        if let OneofType::Enum { type_name, fields } = &oneof.otype {
            assert!(!fields.is_empty(), "empty enums should have been filtered");
            let fields = fields.iter().map(|f| self.generate_oneof_subfield_decl(f));
            let derive_msg = derive_msg_attr(oneof.derive_dbg, false);
            let attrs = &oneof.type_attrs;

            quote! {
                #derive_msg
                #attrs
                pub enum #type_name {
                    #(#fields)*
                }
            }
        } else {
            quote! {}
        }
    }

    fn generate_hazzer_decl(
        &self,
        fields: &[Field],
        msg_conf: CurrentConfig,
    ) -> Option<(TokenStream, TokenStream)> {
        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let attrs = &msg_conf.config.type_attr_parsed();
        let derive_msg = derive_msg_attr(!msg_conf.config.no_debug_derive.unwrap_or(false), true);

        let hazzers = fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        if count == 0 {
            return None;
        }

        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = &f.rust_name;
            let setter = format_ident!("set_{}", f.rust_name);
            let i = Literal::usize_unsuffixed(i);

            quote! {
                #[inline]
                pub fn #fname(&self) -> bool {
                    self.0[#i]
                }

                #[inline]
                pub fn #setter(&mut self, val: bool) {
                    self.0.set(#i, val);
                }
            }
        });

        let count = Literal::usize_unsuffixed(count);
        let decl = quote! {
            #derive_msg
            #attrs
            pub struct #hazzer_name(micropb::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        Some((decl, msg_conf.config.field_attr_parsed()))
    }

    fn create_oneof<'a>(
        &self,
        idx: usize,
        proto: &'a OneofDescriptorProto,
        oneof_conf: CurrentConfig,
    ) -> Option<Oneof<'a>> {
        if oneof_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name();
        let rust_name = oneof_conf.config.rust_field_name(name);
        let otype = match oneof_conf.config.custom_field_parsed() {
            Some(custom) => OneofType::Custom(custom),
            None => OneofType::Enum {
                type_name: Ident::new(
                    &rust_name.to_string().to_case(Case::Pascal),
                    Span::call_site(),
                ),
                fields: vec![],
            },
        };
        let field_attrs = oneof_conf.config.field_attr_parsed();
        let type_attrs = oneof_conf.config.type_attr_parsed();

        Some(Oneof {
            name,
            rust_name,
            idx,
            otype,
            derive_dbg: oneof_conf.derive_dbg(),
            boxed: oneof_conf.config.boxed.unwrap_or(false),
            field_attrs,
            type_attrs,
        })
    }

    fn create_type_spec(
        &self,
        proto: &FieldDescriptorProto,
        type_conf: &CurrentConfig,
    ) -> TypeSpec {
        let conf = &type_conf.config;
        match proto.r#type() {
            Type::Group => panic!("Groups are unsupported"),
            Type::Double => TypeSpec::Double,
            Type::Float => TypeSpec::Float,
            Type::Bool => TypeSpec::Bool,
            Type::String => TypeSpec::String {
                type_path: conf.string_type_parsed().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Bytes => TypeSpec::Bytes {
                type_path: conf.vec_type_parsed().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Message => TypeSpec::Message(proto.type_name().to_owned()),
            Type::Enum => TypeSpec::Enum(proto.type_name().to_owned()),
            Type::Uint32 => TypeSpec::Int(PbInt::Uint32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Int64 => TypeSpec::Int(PbInt::Int64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Uint64 => TypeSpec::Int(PbInt::Uint64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Int32 => TypeSpec::Int(PbInt::Int32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Fixed64 => TypeSpec::Int(PbInt::Fixed64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Fixed32 => TypeSpec::Int(PbInt::Fixed32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sfixed32 => TypeSpec::Int(PbInt::Sfixed32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sfixed64 => TypeSpec::Int(PbInt::Sfixed64, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sint32 => TypeSpec::Int(PbInt::Sint32, conf.int_type.unwrap_or(IntType::I32)),
            Type::Sint64 => TypeSpec::Int(PbInt::Sint64, conf.int_type.unwrap_or(IntType::I32)),
        }
    }

    fn create_field<'a>(
        &self,
        proto: &'a FieldDescriptorProto,
        field_conf: CurrentConfig,
    ) -> Option<Field<'a>> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let oneof = proto.oneof_index.map(|i| i as usize);
        let num = proto.number.unwrap() as u32;
        let name = proto.name();
        // Oneof names are uppercased, since they are used as enum variant names
        let cased_name: Cow<str> = if oneof.is_some() {
            name.to_case(Case::Pascal).into()
        } else {
            name.into()
        };
        let rust_name = field_conf.config.rust_field_name(&cased_name);

        let ftype = match (field_conf.config.custom_field_parsed(), proto.label()) {
            (Some(t @ CustomField::Type(_)), _) => FieldType::Custom(t),
            // Only allow delegate fields for non-oneof fields
            (Some(t @ CustomField::Delegate(_)), _) if oneof.is_none() => FieldType::Custom(t),

            (_, Label::Repeated) => FieldType::Repeated {
                typ: self.create_type_spec(proto, &field_conf.next_conf("elem")),
                type_path: field_conf.config.vec_type_parsed().unwrap(),
                max_len: field_conf.config.max_len,
                packed: proto
                    .options
                    .as_ref()
                    .and_then(|opt| opt.packed)
                    .unwrap_or(false),
            },

            (_, Label::Required) | (None, Label::Optional)
                if self.syntax == Syntax::Proto2
                    || proto.proto3_optional()
                    || proto.r#type() == Type::Message =>
            {
                FieldType::Optional(self.create_type_spec(proto, &field_conf))
            }

            (_, _) => FieldType::Single(self.create_type_spec(proto, &field_conf)),
        };
        let attrs = field_conf.config.field_attr_parsed();

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            oneof,
            default: proto.default_value.as_deref(),
            boxed: field_conf.config.boxed.unwrap_or(false),
            no_hazzer: field_conf.config.no_hazzer.unwrap_or(false),
            attrs,
        })
    }

    fn create_map_field<'a>(
        &self,
        proto: &'a FieldDescriptorProto,
        map_msg: &DescriptorProto,
        field_conf: CurrentConfig,
    ) -> Option<Field<'a>> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name();
        let rust_name = field_conf.config.rust_field_name(name);
        let num = proto.number.unwrap() as u32;

        let ftype = match field_conf.config.custom_field_parsed() {
            Some(custom) => FieldType::Custom(custom),
            None => {
                let key = self.create_type_spec(&map_msg.field[0], &field_conf.next_conf("key"));
                let val = self.create_type_spec(&map_msg.field[1], &field_conf.next_conf("value"));
                let type_name = field_conf.config.vec_type_parsed().unwrap();
                FieldType::Map {
                    key,
                    val,
                    type_path: type_name,
                    max_len: field_conf.config.max_len,
                    packed: proto
                        .options
                        .as_ref()
                        .and_then(|opt| opt.packed)
                        .unwrap_or(false),
                }
            }
        };
        let attrs = field_conf.config.field_attr_parsed();

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            oneof: None,
            default: None,
            boxed: field_conf.config.boxed.unwrap_or(false),
            no_hazzer: field_conf.config.no_hazzer.unwrap_or(false),
            attrs,
        })
    }

    fn generate_tspec_rust_type(&self, tspec: &TypeSpec) -> TokenStream {
        match tspec {
            TypeSpec::Int(_, itype) => {
                let typ = itype.type_name();
                quote! { #typ }
            }
            TypeSpec::Float => quote! {f32},
            TypeSpec::Double => quote! {f64},
            TypeSpec::Bool => quote! {bool},
            TypeSpec::String {
                type_path,
                max_bytes,
            } => {
                let max_bytes = max_bytes.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path #(<#max_bytes>)* }
            }
            TypeSpec::Bytes {
                type_path,
                max_bytes,
            } => {
                let max_bytes = max_bytes.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path <u8 #(, #max_bytes)* > }
            }
            TypeSpec::Message(tname) | TypeSpec::Enum(tname) => {
                let rust_type = self.resolve_type_name(tname);
                quote! { #rust_type }
            }
        }
    }

    fn generate_field_rust_type(&self, field: &Field) -> TokenStream {
        let typ = match &field.ftype {
            FieldType::Map {
                key,
                val,
                type_path: type_name,
                max_len,
                ..
            } => {
                let k = self.generate_tspec_rust_type(key);
                let v = self.generate_tspec_rust_type(val);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_name <#k, #v #(, #max_len)* > }
            }

            FieldType::Single(t) | FieldType::Optional(t) => self.generate_tspec_rust_type(t),

            FieldType::Repeated {
                typ,
                type_path,
                max_len,
                ..
            } => {
                let t = self.generate_tspec_rust_type(typ);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path <#t #(, #max_len)* > }
            }

            FieldType::Custom(CustomField::Type(t)) => return quote! {#t},
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have a type")
            }
        };

        if field.boxed {
            let box_type = self.box_type();
            let boxed = quote! { #box_type<#typ> };
            if field.explicit_presence() {
                quote! { core::option::Option<#boxed> }
            } else {
                boxed
            }
        } else {
            typ
        }
    }

    fn generate_field_decl(&self, field: &Field) -> TokenStream {
        if let FieldType::Custom(CustomField::Delegate(_)) = field.ftype {
            return quote! {};
        }
        let typ = self.generate_field_rust_type(field);
        let name = &field.rust_name;
        let attrs = &field.attrs;
        quote! { #attrs #name : #typ, }
    }

    fn generate_oneof_field_decl(&self, msg_mod_name: &Ident, oneof: &Oneof) -> TokenStream {
        let name = &oneof.rust_name;
        let oneof_type = match &oneof.otype {
            OneofType::Enum { type_name, .. } => {
                let typ = if oneof.boxed {
                    let box_type = self.box_type();
                    quote! { #box_type<#msg_mod_name::#type_name> }
                } else {
                    quote! { #msg_mod_name::#type_name }
                };
                quote! { core::option::Option<#typ> }
            }
            OneofType::Custom(CustomField::Type(type_path)) => quote! { #type_path },
            OneofType::Custom(CustomField::Delegate(_)) => return quote! {},
        };
        let attrs = &oneof.field_attrs;
        quote! { #attrs #name: #oneof_type, }
    }

    fn generate_oneof_subfield_decl(&self, field: &Field) -> TokenStream {
        let typ = self.generate_field_rust_type(field);
        let name = &field.rust_name;
        let attrs = &field.attrs;
        quote! { #attrs #name(#typ), }
    }

    fn generate_tspec_default(&self, tspec: &TypeSpec, default: &str) -> TokenStream {
        match tspec {
            TypeSpec::String { .. } => {
                quote! { #default }
            }
            TypeSpec::Bytes { .. } => {
                let bytes = Literal::byte_string(&unescape_c_escape_string(default));
                quote! { #bytes }
            }
            TypeSpec::Message(_) => {
                unreachable!("message fields shouldn't have custom defaults")
            }
            TypeSpec::Enum(tname) => {
                let enum_name =
                    Ident::new(&path_suffix(tname).to_case(Case::Pascal), Span::call_site());
                let variant = self.enum_variant_name(default, &enum_name);
                quote! { #enum_name::#variant }
            }
            _ => {
                let default: TokenStream =
                    syn::parse_str(default).expect("default value tokenization error");
                quote! { #default as _ }
            }
        }
    }

    fn generate_field_default(&self, field: &Field) -> TokenStream {
        if let Some(default) = field.default {
            match field.ftype {
                FieldType::Single(ref t) | FieldType::Optional(ref t) => {
                    let value = self.generate_tspec_default(t, default);
                    return if field.boxed {
                        let box_type = self.box_type();
                        let typ = quote! { #box_type::new(#value) };
                        if field.explicit_presence() {
                            quote! { core::option::Option::Some(#typ) }
                        } else {
                            typ
                        }
                    } else {
                        value
                    };
                }
                FieldType::Custom(CustomField::Delegate(_)) => return quote! {},
                FieldType::Custom(CustomField::Type(_)) => {}
                _ => unreachable!("repeated and map fields shouldn't have custom defaults"),
            }
        }
        quote! { core::default::Default::default() }
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
        let stripped = if !self.config.retain_enum_prefix {
            variant_name_cased
                .strip_prefix(&enum_name.to_string())
                .unwrap_or(&variant_name_cased)
        } else {
            &variant_name_cased
        };
        Ident::new(stripped, Span::call_site())
    }

    fn box_type(&self) -> TokenStream {
        if self.config.use_std {
            quote! { std::boxed::Box }
        } else {
            quote! { alloc::boxed::Box }
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
    use protox::prost_reflect::prost_types::EnumValueDescriptorProto;

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

        gen.config.retain_enum_prefix = true;
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
    fn tspec_rust_type() {
        let gen = Generator::default();
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Bool).to_string(),
            "bool"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Float).to_string(),
            "f32"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Double).to_string(),
            "f64"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Int(PbInt::Int32, IntType::I32))
                .to_string(),
            "i32"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Int(PbInt::Fixed32, IntType::I32))
                .to_string(),
            "i32"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Int(PbInt::Fixed32, IntType::Usize))
                .to_string(),
            "usize"
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::String {
                type_path: syn::parse_str("heapless::String").unwrap(),
                max_bytes: Some(10)
            })
            .to_string(),
            quote! { heapless::String<10> }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::String {
                type_path: syn::parse_str("heapless::String").unwrap(),
                max_bytes: None
            })
            .to_string(),
            quote! { heapless::String }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Enum(".package.Enum".to_owned()))
                .to_string(),
            quote! { package::Enum }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_rust_type(&TypeSpec::Message(".package.Msg".to_owned()))
                .to_string(),
            quote! { package::Msg }.to_string()
        );
    }

    fn make_test_field(num: u32, name: &str, boxed: bool, ftype: FieldType) -> Field {
        Field {
            num,
            ftype,
            name,
            rust_name: Ident::new(name, Span::call_site()),
            default: None,
            oneof: None,
            boxed,
            no_hazzer: false,
            attrs: quote! {},
        }
    }

    #[test]
    fn field_rust_type() {
        let gen = Generator::default();
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool)
            ))
            .to_string(),
            "bool"
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                false,
                FieldType::Repeated {
                    typ: TypeSpec::Message(".Message".to_owned()),
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_len: None,
                    packed: true
                }
            ))
            .to_string(),
            quote! { Vec<Message> }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                false,
                FieldType::Repeated {
                    typ: TypeSpec::String {
                        type_path: syn::parse_str("String").unwrap(),
                        max_bytes: Some(4)
                    },
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_len: Some(10),
                    packed: true
                }
            ))
            .to_string(),
            quote! { Vec<String<4>, 10> }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                false,
                FieldType::Map {
                    key: TypeSpec::Float,
                    val: TypeSpec::Int(PbInt::Uint64, IntType::U32),
                    type_path: syn::parse_str("std::HashMap").unwrap(),
                    max_len: None,
                    packed: true
                }
            ))
            .to_string(),
            quote! { std::HashMap<f32, u32> }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                false,
                FieldType::Map {
                    key: TypeSpec::Int(PbInt::Uint64, IntType::U32),
                    val: TypeSpec::Float,
                    type_path: syn::parse_str("std::HashMap").unwrap(),
                    max_len: Some(8),
                    packed: true
                }
            ))
            .to_string(),
            quote! { std::HashMap<u32, f32, 8> }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                true,
                FieldType::Custom(CustomField::Type(
                    syn::parse_str("custom::Type<true>").unwrap()
                ))
            ))
            .to_string(),
            quote! { custom::Type<true> }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                true,
                FieldType::Optional(TypeSpec::Message(".Config".to_owned()))
            ))
            .to_string(),
            quote! { core::option::Option<alloc::boxed::Box<Config> > }.to_string()
        );
        assert_eq!(
            gen.generate_field_rust_type(&make_test_field(
                0,
                "field",
                true,
                FieldType::Single(TypeSpec::Message(".Config".to_owned()))
            ))
            .to_string(),
            quote! { alloc::boxed::Box<Config> }.to_string()
        );
    }

    #[test]
    fn tspec_default() {
        let gen = Generator::default();
        assert_eq!(
            gen.generate_tspec_default(&TypeSpec::Bool, "true")
                .to_string(),
            quote! { true as _ }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(&TypeSpec::Bool, "false")
                .to_string(),
            quote! { false as _ }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(&TypeSpec::Float, "0.1")
                .to_string(),
            quote! { 0.1 as _ }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(&TypeSpec::Double, "-4.1")
                .to_string(),
            quote! { -4.1 as _ }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(&TypeSpec::Int(PbInt::Int32, IntType::I8), "-99")
                .to_string(),
            quote! { -99 as _ }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(
                &TypeSpec::String {
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_bytes: None
                },
                "abc\n\tddd"
            )
            .to_string(),
            quote! { "abc\n\tddd" }.to_string()
        );
        assert_eq!(
            gen.generate_tspec_default(
                &TypeSpec::Bytes {
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_bytes: None
                },
                "abc\\n\\t\\a\\xA0ddd"
            )
            .to_string(),
            quote! { b"abc\n\t\x07\xA0ddd" }.to_string()
        );
    }

    #[test]
    fn field_default() {
        let gen = Generator::default();
        // no special default
        assert_eq!(
            gen.generate_field_default(&make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool)
            ))
            .to_string(),
            quote! { core::default::Default::default() }.to_string()
        );

        let mut field = make_test_field(0, "field", false, FieldType::Optional(TypeSpec::Bool));
        field.default = Some("false");
        assert_eq!(
            gen.generate_field_default(&field).to_string(),
            quote! { false as _ }.to_string()
        );

        let mut field = make_test_field(0, "field", true, FieldType::Single(TypeSpec::Bool));
        field.default = Some("false");
        assert_eq!(
            gen.generate_field_default(&field).to_string(),
            quote! { alloc::boxed::Box::new(false as _) }.to_string()
        );

        let mut field = make_test_field(0, "field", true, FieldType::Optional(TypeSpec::Bool));
        field.default = Some("false");
        assert_eq!(
            gen.generate_field_default(&field).to_string(),
            quote! { core::option::Option::Some(alloc::boxed::Box::new(false as _)) }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Custom(CustomField::Type(syn::parse_str("Map").unwrap())),
        );
        field.default = Some("false");
        assert_eq!(
            gen.generate_field_default(&field).to_string(),
            quote! { core::default::Default::default() }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Custom(CustomField::Delegate(syn::parse_str("del").unwrap())),
        );
        field.default = Some("false");
        assert_eq!(gen.generate_field_default(&field).to_string(), "");
    }

    #[test]
    fn hazzer() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(
                Config::new()
                    .type_attributes("#[derive(Eq)]")
                    .field_attributes("#[default]")
                    .no_debug_derive(true),
            )),
        };
        let gen = Generator::default();
        let (decl, field_attrs) = gen
            .generate_hazzer_decl(
                &[
                    make_test_field(1, "field1", false, FieldType::Optional(TypeSpec::Bool)),
                    make_test_field(2, "field2", false, FieldType::Single(TypeSpec::Bool)),
                    make_test_field(3, "field3", false, FieldType::Optional(TypeSpec::Bool)),
                    make_test_field(4, "field4", true, FieldType::Optional(TypeSpec::Bool)),
                ],
                config,
            )
            .unwrap();

        let expected = quote! {
            #[derive(Default, Clone, PartialEq)]
            #[derive(Eq)]
            pub struct _Hazzer(micropb::bitvec::BitArr!(for 2, in u8));

            impl _Hazzer {
                #[inline]
                pub fn field1(&self) -> bool {
                    self.0[0]
                }

                #[inline]
                pub fn set_field1(&mut self, val: bool) {
                    self.0.set(0, val);
                }

                #[inline]
                pub fn field3(&self) -> bool {
                    self.0[1]
                }

                #[inline]
                pub fn set_field3(&mut self, val: bool) {
                    self.0.set(1, val);
                }
            }
        };
        assert_eq!(decl.to_string(), expected.to_string());
        assert_eq!(field_attrs.to_string(), quote! { #[default] }.to_string());
    }

    #[test]
    fn hazzer_empty() {
        let config = CurrentConfig {
            node: None,
            config: Cow::Owned(Box::new(Config::new())),
        };
        let gen = Generator::default();
        assert!(gen
            .generate_hazzer_decl(
                &[
                    make_test_field(2, "field2", false, FieldType::Single(TypeSpec::Bool)),
                    make_test_field(4, "field4", true, FieldType::Optional(TypeSpec::Bool)),
                ],
                config,
            )
            .is_none());
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

        let out = gen.generate_enum_type(&enum_proto, config);
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
        assert!(gen.generate_enum_type(&enum_proto, config).is_empty())
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
        gen.config.retain_enum_prefix = true;

        let out = gen.generate_enum_type(&enum_proto, config);
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
    fn oneof_enum() {
        let gen = Generator::default();
        let mut fields = vec![
            make_test_field(0, "A", true, FieldType::Optional(TypeSpec::Float)),
            make_test_field(1, "B", false, FieldType::Single(TypeSpec::Bool)),
        ];
        fields[0].oneof = Some(0);
        fields[1].oneof = Some(0);
        let oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Enum {
                type_name: Ident::new("Oneof", Span::call_site()),
                fields,
            },
            boxed: false,
            field_attrs: quote! { #[default] },
            type_attrs: quote! { #[derive(Eq)] },
            derive_dbg: true,
            idx: 0,
        };

        let out = gen.generate_oneof_decl(&oneof);
        let expected = quote! {
            #[derive(Debug, Clone, PartialEq)]
            #[derive(Eq)]
            pub enum Oneof {
                A(alloc::boxed::Box<f32>),
                B(bool),
            }
        };
        assert_eq!(out.to_string(), expected.to_string());

        assert_eq!(
            gen.generate_oneof_field_decl(&Ident::new("Msg", Span::call_site()), &oneof)
                .to_string(),
            quote! { #[default] oneof: core::option::Option<Msg::Oneof>, }.to_string()
        );
    }

    #[test]
    fn oneof_custom() {
        let gen = Generator::default();
        let oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Custom(CustomField::Type(syn::parse_str("Custom<f32>").unwrap())),
            boxed: true,
            field_attrs: quote! {},
            type_attrs: quote! {},
            derive_dbg: true,
            idx: 0,
        };
        assert!(gen.generate_oneof_decl(&oneof).is_empty());
        assert_eq!(
            gen.generate_oneof_field_decl(&Ident::new("Msg", Span::call_site()), &oneof)
                .to_string(),
            quote! { oneof: Custom<f32>, }.to_string()
        );

        let oneof = Oneof {
            name: "oneof",
            rust_name: Ident::new("oneof", Span::call_site()),
            otype: OneofType::Custom(CustomField::Delegate(syn::parse_str("delegate").unwrap())),
            boxed: false,
            field_attrs: quote! {},
            type_attrs: quote! {},
            derive_dbg: true,
            idx: 0,
        };
        assert!(gen.generate_oneof_decl(&oneof).is_empty());
        assert!(gen
            .generate_oneof_field_decl(&Ident::new("Msg", Span::call_site()), &oneof)
            .is_empty());
    }
}
