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
    config::{Config, GenConfig, IntType},
    pathtree::Node,
    utils::{suffix, unescape_c_escape_string},
};

fn derive_msg_attr(debug: bool, default: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    let default = default.then(|| quote! { Default, });
    quote! { #[derive(#debug #default Clone, PartialEq)] }
}

fn derive_enum_attr(debug: bool) -> TokenStream {
    let debug = debug.then(|| quote! { Debug, });
    quote! { #[derive(#debug Clone, PartialEq, Copy, Eq, Hash)] }
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
        matches!(self.ftype, FieldType::Optional(_))
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
            if let Some(conf) = next.and_then(|n| n.value()) {
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
            .unwrap_or_else(|| &self.config.default_pkg_filename)
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
        let mut root_conf = root_node.value().expect("root config should exist").clone();
        root_node.get(
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
            .map(|v| Literal::i32_unsuffixed(v.number.unwrap()));
        let var_names = enum_type
            .value
            .iter()
            .map(|v| self.enum_variant_name(&v.name(), &name));
        let default_num = Literal::i32_unsuffixed(enum_type.value[0].number.unwrap());
        let enum_int_type = enum_conf.config.enum_int_type.unwrap_or(IntType::I32);
        let itype = enum_int_type.type_name();
        let attrs = &enum_conf.config.type_attributes;
        let derive_enum = derive_enum_attr(!enum_conf.config.no_debug_derive.unwrap_or(false));

        quote! {
            #derive_enum
            #[repr(transparent)]
            #attrs
            pub struct #name(pub #itype);

            impl #name {
                #(pub const #var_names: Self = #name(#nums);)*
            }

            impl core::default::Default for #name {
                fn default() -> Self {
                    #name(#default_num)
                }
            }

            impl core::convert::From<#itype> for $name {
                fn from(val: #itype) -> Self {
                    #name(val)
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

        let opt_fields: Vec<_> = fields.iter().filter(|f| f.explicit_presence()).collect();
        let hazzer_exists = !opt_fields.is_empty();
        let hazzer_decl = hazzer_exists.then(|| self.generate_hazzer_decl(&opt_fields, &msg_conf));

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
        let msg_fields = fields.iter().map(|f| self.field_decl(f));
        let hazzer_field = hazzer_exists.then(|| quote! { pub _has: #msg_mod_name::_Hazzer, });
        let oneof_fields = oneofs
            .iter()
            .map(|oneof| self.oneof_field_decl(&msg_mod_name, oneof));

        let (derive_default, decl_default) = if fields.iter().any(|f| f.default.is_some()) {
            let defaults = fields.iter().map(|f| self.field_default(f));
            let hazzer_default =
                hazzer_exists.then(|| quote! { _has: core::default::Default::default(), });
            let decl = quote! {
                impl core::default::Default for #rust_name {
                    fn default() -> Self {
                        Self {
                            #(#defaults)*
                            #hazzer_default
                        }
                    }
                }
            };
            (false, Some(decl))
        } else {
            (true, None)
        };

        let derive_msg = derive_msg_attr(
            !msg_conf.config.no_debug_derive.unwrap_or(false),
            derive_default,
        );
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

            #decl_default
        }
    }

    fn generate_oneof_decl(&self, oneof: &Oneof) -> TokenStream {
        if let OneofType::Enum { type_name, fields } = &oneof.otype {
            let fields = fields.iter().map(|f| self.oneof_subfield_decl(f));
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

    fn generate_hazzer_decl(&self, fields: &[&Field], msg_conf: &CurrentConfig) -> TokenStream {
        let hazzer_name = Ident::new("_Hazzer", Span::call_site());
        let attrs = &msg_conf.config.hazzer_attr_parsed();
        let derive_msg = derive_msg_attr(!msg_conf.config.no_debug_derive.unwrap_or(false), true);

        let hazzers = fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        let methods = hazzers.enumerate().map(|(i, f)| {
            let fname = format_ident!("{}", f.name);
            let setter = format_ident!("set_{fname}");

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

        let decl = quote! {
            #derive_msg
            #attrs
            pub struct #hazzer_name(micropb::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        decl
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
                type_name: Ident::new(&name.to_case(Case::Pascal), Span::call_site()),
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

    fn tspec_rust_type(&self, tspec: &TypeSpec) -> TokenStream {
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
                if let Some(max_bytes) = max_bytes {
                    quote! { #type_path <#max_bytes> }
                } else {
                    quote! { #type_path }
                }
            }
            TypeSpec::Bytes {
                type_path,
                max_bytes,
            } => {
                if let Some(max_bytes) = max_bytes {
                    quote! { #type_path <u8, #max_bytes> }
                } else {
                    quote! { #type_path <u8> }
                }
            }
            TypeSpec::Message(tname) | TypeSpec::Enum(tname) => {
                let rust_type = self.resolve_type_name(tname);
                quote! { #rust_type }
            }
        }
    }

    fn rust_type(&self, field: &Field) -> TokenStream {
        let typ = match &field.ftype {
            FieldType::Map {
                key,
                val,
                type_path: type_name,
                max_len,
                ..
            } => {
                let k = self.tspec_rust_type(key);
                let v = self.tspec_rust_type(val);
                if let Some(max_len) = max_len {
                    quote! { #type_name <#k, #v, #max_len> }
                } else {
                    quote! { #type_name <#k, #v> }
                }
            }

            FieldType::Single(t) | FieldType::Optional(t) => self.tspec_rust_type(t),

            FieldType::Repeated {
                typ,
                type_path: type_name,
                max_len,
                ..
            } => {
                let t = self.tspec_rust_type(typ);
                if let Some(max_len) = max_len {
                    quote! { #type_name <#t, #max_len> }
                } else {
                    quote! { #type_name <#t> }
                }
            }

            FieldType::Custom(CustomField::Type(t)) => quote! {#t},
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have a type")
            }
        };

        if field.boxed {
            let box_type = self.box_type();
            if field.explicit_presence() {
                quote! { core::option::Option<#box_type<#typ>> }
            } else {
                quote! { #box_type<#typ> }
            }
        } else {
            typ
        }
    }

    fn field_decl(&self, field: &Field) -> TokenStream {
        if let FieldType::Custom(CustomField::Delegate(_)) = field.ftype {
            return quote! {};
        }
        let typ = self.rust_type(field);
        let name = &field.rust_name;
        let attrs = &field.attrs;
        quote! { #attrs #name : #typ, }
    }

    fn oneof_field_decl(&self, msg_mod_name: &Ident, oneof: &Oneof) -> TokenStream {
        let name = &oneof.rust_name;
        let type_name = match &oneof.otype {
            OneofType::Enum { type_name, .. } => quote! { #msg_mod_name::#type_name },
            OneofType::Custom(CustomField::Type(type_path)) => quote! { #type_path },
            OneofType::Custom(CustomField::Delegate(_)) => return quote! {},
        };
        let attrs = &oneof.field_attrs;
        let typ = if oneof.boxed {
            let box_type = self.box_type();
            quote! { core::option::Option<#box_type<#type_name>> }
        } else {
            quote! { core::option::Option<#type_name> }
        };
        quote! { #attrs #name: #typ, }
    }

    fn oneof_subfield_decl(&self, field: &Field) -> TokenStream {
        let typ = self.rust_type(field);
        let name = &field.rust_name;
        let attrs = &field.attrs;
        quote! { #attrs #name(#typ), }
    }

    fn tspec_default(&self, tspec: &TypeSpec, default: &str) -> TokenStream {
        match tspec {
            TypeSpec::String { .. } => {
                let string = default.escape_default().to_string();
                quote! { micropb::PbString::from_str(#string).expect("default string went over capacity") }
            }
            TypeSpec::Bytes { .. } => {
                let bytes = Literal::byte_string(&unescape_c_escape_string(default));
                quote! { micropb::PbVec::from_slice(#bytes).expect("default bytes went over capacity") }
            }
            TypeSpec::Message(_) => {
                unreachable!("message fields shouldn't have custom defaults")
            }
            TypeSpec::Enum(tname) => {
                let enum_name = Ident::new(&suffix(tname).to_case(Case::Pascal), Span::call_site());
                let variant = self.enum_variant_name(&default, &enum_name);
                quote! { #enum_name::#variant }
            }
            TypeSpec::Bool => {
                // true and false are identifiers, not literals
                let default = Ident::new(default, Span::call_site());
                quote! { #default }
            }
            _ => {
                let lit: Literal =
                    syn::parse_str(&default).expect("numeric default should be valid Rust literal");
                quote! { #lit as _ }
            }
        }
    }

    fn field_default(&self, field: &Field) -> TokenStream {
        let name = &field.rust_name;
        if let Some(default) = field.default {
            match field.ftype {
                FieldType::Single(ref t) | FieldType::Optional(ref t) => {
                    let value = self.tspec_default(t, default);
                    return if field.boxed {
                        let box_type = self.box_type();
                        if field.explicit_presence() {
                            quote! { #name: core::option::Option::Some(#box_type::new(#value)), }
                        } else {
                            quote! { #name: #box_type::new(#value), }
                        }
                    } else {
                        quote! { #name: #value, }
                    };
                }
                FieldType::Custom(CustomField::Delegate(_)) => return quote! {},
                FieldType::Custom(CustomField::Type(_)) => {}
                _ => unreachable!("repeated and map fields shouldn't have custom defaults"),
            }
        }
        quote! { #name: core::default::Default::default(), }
    }

    fn resolve_type_name(&self, pb_fq_type_name: &str) -> TokenStream {
        assert_eq!(".", &pb_fq_type_name[1..]);

        let mut ident_path = pb_fq_type_name[1..].split('.');
        let ident_type = ident_path.next_back().unwrap();
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
            .map(|_| Ident::new("super", Span::call_site()))
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

    fn enum_variant_name(&self, variant_name: &str, enum_name: &Ident) -> Ident {
        let variant_name_cased = variant_name.to_case(Case::Pascal);
        let stripped = if self.config.strip_enum_prefix {
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
