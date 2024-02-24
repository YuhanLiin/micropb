use std::{borrow::Cow, cell::RefCell, collections::HashMap, iter, ops::Deref};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use protox::prost_reflect::prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet,
};
use quote::quote;

use crate::{
    config::{Config, GenConfig, IntType},
    pathtree::Node,
};

static DERIVE_MSG: &str = "#[derive(Clone, PartialEq)]";
static DERIVE_ENUM: &str = "#[derive(Clone, Copy, PartialEq, Eq, Hash)]";
static DERIVE_DEFAULT: &str = "#[derive(Default)]";
static DERIVE_DEBUG: &str = "#[derive(Debug)]";
static REPR_ENUM: &str = "#[repr(transparent)]";

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

enum TypeOpt {
    Name(String),
    Int(IntType),
    Container {
        type_name: String,
        fixed_len: Option<u32>,
    },
}

struct TypeSpec {
    typ: Type,
    int_type: Option<IntType>,
    name: Option<String>,
    fixed_len: Option<u32>,
}

enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec,
        val: TypeSpec,
        packed: bool,
        type_name: String,
        fixed_len: Option<u32>,
    },
    // Implicit presence
    Single(TypeSpec),
    // Explicit presence
    Optional(TypeSpec),
    Repeated {
        typ: TypeSpec,
        packed: bool,
        type_name: String,
        fixed_len: Option<u32>,
    },
    Custom(String),
}

struct Field<'a> {
    num: u32,
    ftype: FieldType,
    name: &'a str,
    default: Option<&'a str>,
    oneof: Option<&'a str>,
    boxed: bool,
    attrs: Option<String>,
}

impl<'a> Field<'a> {
    fn explicit_presence(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_))
    }
}

struct CurrentConfig<'a> {
    node: Option<&'a Node<Config>>,
    config: Config,
}

impl<'a> CurrentConfig<'a> {
    fn next_conf(&self, segment: &str) -> Self {
        let mut config = self.config.clone();
        if let Some(node) = self.node {
            let next = node.next(segment);
            if let Some(conf) = next.and_then(|n| n.value()) {
                config.merge(conf);
            }
            Self { node: next, config }
        } else {
            Self { node: None, config }
        }
    }

    fn derive_dbg(&self) -> Option<&str> {
        (!self.config.no_debug_derive.unwrap_or(false)).then_some(DERIVE_DEBUG)
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
            config: root_conf,
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

        let name = enum_type.name.as_ref().unwrap();
        let nums = enum_type.value.iter().map(|v| v.number.unwrap());
        let var_names = enum_type
            .value
            .iter()
            .map(|v| v.name.as_ref().unwrap().to_case(Case::Pascal))
            .map(|v| self.strip_enum_prefix(&v, name).to_owned());
        let default_num = enum_type.value[0].number.unwrap();
        let enum_int_type = enum_conf.config.enum_int_type.unwrap_or(IntType::I32);
        let itype = enum_int_type.type_name();
        let attrs = &enum_conf.config.type_attributes;
        let derive_dbg = enum_conf.derive_dbg();

        quote! {
            #derive_dbg
            #DERIVE_ENUM
            #REPR_ENUM
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

            impl core::convert::From<i32> for $name {
                fn from(val: i32) -> Self {
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
        let fq_name = self.fq_name(name);
        let msg_mod_name = format!("mod_{name}");
        let oneofs: Vec<_> = msg_type
            .oneof_decl
            .iter()
            .map(|oneof| oneof.name.as_deref().unwrap())
            .collect();
        let oneofs_types: Vec<_> = oneofs.iter().map(|o| o.to_case(Case::Pascal)).collect();
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

        let (fields, oneof_fields): (Vec<_>, Vec<_>) = msg_type
            .field
            .iter()
            .filter_map(|f| {
                let field_conf = msg_conf.next_conf(f.name());
                if field_conf.config.skip.unwrap_or(false) {
                    return None;
                }

                let raw_msg_name = f
                    .type_name()
                    .rsplit_once('.')
                    .map(|(_, r)| r)
                    .unwrap_or(f.type_name());
                if let Some(map_msg) = map_types.remove(raw_msg_name) {
                    Some(self.create_map_field(f, map_msg, field_conf))
                } else {
                    Some(self.create_field(f, &oneofs, field_conf))
                }
            })
            .partition(|f| f.oneof.is_none());

        self.type_path.borrow_mut().push(name.to_owned());
        let oneof_decls = oneofs
            .iter()
            .zip(oneofs_types.iter())
            .map(|(oneof, oneof_type)| {
                let fields = oneof_fields
                    .iter()
                    .filter(|f| f.oneof == Some(*oneof))
                    .map(|f| self.field_decl(f));

                quote! {
                    #DERIVE_MSG
                    pub enum #oneof_type {
                        #(#fields)*
                    }
                }
            });

        let nested_msgs = inner_msgs
            .iter()
            .map(|m| self.generate_msg_type(m, msg_conf.next_conf(m.name())));
        let nested_enums = msg_type
            .enum_type
            .iter()
            .map(|e| self.generate_enum_type(e, msg_conf.next_conf(e.name())));
        let msg_mod = quote! {
            pub mod #msg_mod_name {
                #(#oneof_decls)*
                #(#nested_msgs)*
                #(#nested_enums)*
            }
        };
        self.type_path.borrow_mut().pop();

        let msg_fields = fields.iter().map(|f| self.field_decl(f));
        let opt_fields: Vec<_> = fields.iter().filter(|f| f.explicit_presence()).collect();
        let (hazzer_name, hazzer_decl) = if !opt_fields.is_empty() {
            let (n, t) = self.generate_hazzer(name, &opt_fields, &msg_conf);
            (Some(n), Some(t))
        } else {
            (None, None)
        };
        let hazzer_field = hazzer_name.as_ref().map(|n| quote! { pub has: #n, });

        let (derive_default, decl_default) = if fields.iter().any(|f| f.default.is_some()) {
            let defaults = fields.iter().map(|f| self.field_default(f));
            let hazzer_default = hazzer_name
                .as_ref()
                .map(|_| quote! { has: core::default::Default::default(), });
            let decl = quote! {
                impl core::default::Default for #name {
                    fn default() -> Self {
                        Self {
                            #(#defaults)*
                            #hazzer_default
                        }
                    }
                }
            };
            (None, Some(decl))
        } else {
            (Some(DERIVE_DEFAULT), None)
        };

        let attrs = &msg_conf.config.type_attributes;
        let derive_dbg = msg_conf.derive_dbg();

        quote! {
            #msg_mod

            #hazzer_decl

            #derive_dbg
            #derive_default
            #DERIVE_MSG
            #attrs
            pub struct #name {
                #(pub #msg_fields)*
                #(pub #oneofs: Option<#msg_mod_name::#oneofs_types>)*
                #hazzer_field
            }

            #decl_default
        }
    }

    fn generate_hazzer(
        &self,
        name: &str,
        fields: &[&Field],
        msg_conf: &CurrentConfig,
    ) -> (String, TokenStream) {
        let count = fields.len();
        let micropb_path = &self.config.micropb_path;
        let hazzer_name = format!("{name}Hazzer");
        let attrs = &msg_conf.config.hazzer_attributes;
        let derive_dbg = msg_conf.derive_dbg();

        let methods = fields.iter().enumerate().map(|(i, f)| {
            let fname = f.name;
            let setter = format!("set_{fname}");

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
            #derive_dbg
            #DERIVE_MSG
            #DERIVE_DEFAULT
            #attrs
            pub struct #hazzer_name(#micropb_path::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        (hazzer_name, decl)
    }

    fn create_type_spec(
        &self,
        proto: &FieldDescriptorProto,
        type_conf: &CurrentConfig,
    ) -> TypeSpec {
        let conf = &type_conf.config;
        let typ = proto.r#type();
        let name = match typ {
            Type::String => conf.string_type.clone(),
            Type::Bytes => conf.vec_type.clone(),
            Type::Enum | Type::Message => proto.type_name.clone(),
            _ => None,
        };
        TypeSpec {
            typ,
            name,
            int_type: conf.int_type,
            fixed_len: conf.fixed_len,
        }
    }

    fn create_field<'a>(
        &self,
        proto: &'a FieldDescriptorProto,
        oneofs: &'a [&str],
        field_conf: CurrentConfig,
    ) -> Field<'a> {
        let name = proto.name.as_ref().unwrap();
        let num = proto.number.unwrap() as u32;

        let ftype = if let Some(custom_type) = field_conf.config.custom_type {
            FieldType::Custom(custom_type)
        } else {
            match proto.label() {
                Label::Repeated => FieldType::Repeated {
                    typ: self.create_type_spec(proto, &field_conf.next_conf("elem")),
                    type_name: field_conf.config.vec_type.clone().unwrap(),
                    fixed_len: field_conf.config.fixed_len,
                    packed: proto
                        .options
                        .as_ref()
                        .and_then(|opt| opt.packed)
                        .unwrap_or(false),
                },
                Label::Required | Label::Optional
                    if self.syntax == Syntax::Proto2
                        || proto.proto3_optional()
                        || proto.r#type() == Type::Message =>
                {
                    FieldType::Optional(self.create_type_spec(proto, &field_conf))
                }
                _ => FieldType::Single(self.create_type_spec(proto, &field_conf)),
            }
        };
        let oneof = proto.oneof_index.map(|i| oneofs[i as usize]);
        let default = proto.default_value.as_deref();

        Field {
            num,
            ftype,
            name,
            oneof,
            default,
            boxed: field_conf.config.boxed.unwrap_or(false),
            attrs: field_conf.config.field_attributes.clone(),
        }
    }

    fn create_map_field<'a>(
        &self,
        proto: &'a FieldDescriptorProto,
        map_msg: &DescriptorProto,
        field_conf: CurrentConfig,
    ) -> Field<'a> {
        let name = proto.name.as_ref().unwrap();
        let num = proto.number.unwrap() as u32;

        let ftype = if let Some(custom_type) = field_conf.config.custom_type {
            FieldType::Custom(custom_type)
        } else {
            let key = self.create_type_spec(&map_msg.field[0], &field_conf.next_conf("key"));
            let val = self.create_type_spec(&map_msg.field[1], &field_conf.next_conf("value"));
            FieldType::Map {
                key,
                val,
                type_name: field_conf.config.vec_type.clone().unwrap(),
                fixed_len: field_conf.config.fixed_len,
                packed: proto
                    .options
                    .as_ref()
                    .and_then(|opt| opt.packed)
                    .unwrap_or(false),
            }
        };

        Field {
            num,
            ftype,
            name,
            oneof: None,
            default: None,
            boxed: field_conf.config.boxed.unwrap_or(false),
            attrs: field_conf.config.field_attributes.clone(),
        }
    }

    fn tspec_rust_type(&self, tspec: &TypeSpec) -> TokenStream {
        fn int_type(itype: Option<IntType>, default: &str) -> TokenStream {
            let typ = itype.map(IntType::type_name).unwrap_or(default);
            quote! { #typ }
        }

        match tspec.typ {
            Type::Int32 | Type::Sint32 | Type::Sfixed32 => int_type(tspec.int_type, "i32"),
            Type::Int64 | Type::Sint64 | Type::Sfixed64 => int_type(tspec.int_type, "i64"),
            Type::Uint32 | Type::Fixed32 => int_type(tspec.int_type, "u32"),
            Type::Uint64 | Type::Fixed64 => int_type(tspec.int_type, "u64"),
            Type::Float => quote! {f32},
            Type::Double => quote! {f64},
            Type::Bool => quote! {bool},
            Type::String => {
                let str_type = tspec.name.as_ref().unwrap();
                if let Some(max_bytes) = tspec.fixed_len {
                    quote! { #str_type <#max_bytes> }
                } else {
                    quote! { #str_type }
                }
            }
            Type::Bytes => {
                let vec_type = tspec.name.as_ref().unwrap();
                if let Some(max_len) = tspec.fixed_len {
                    quote! { #vec_type <u8, #max_len> }
                } else {
                    quote! { #vec_type <u8> }
                }
            }
            Type::Message | Type::Enum => self.resolve_ident(tspec.name.as_ref().unwrap()),
            Type::Group => panic!("Group records are deprecated and unsupported"),
        }
    }

    fn rust_type(&self, field: &Field) -> TokenStream {
        let typ = match &field.ftype {
            FieldType::Map {
                key,
                val,
                type_name,
                fixed_len,
                ..
            } => {
                let k = self.tspec_rust_type(key);
                let v = self.tspec_rust_type(val);
                if let Some(max_len) = fixed_len {
                    quote! { #type_name <#k, #v, #max_len> }
                } else {
                    quote! { #type_name <#k, #v> }
                }
            }

            FieldType::Single(t) | FieldType::Optional(t) => self.tspec_rust_type(t),

            FieldType::Repeated {
                typ,
                type_name,
                fixed_len,
                ..
            } => {
                let t = self.tspec_rust_type(typ);
                if let Some(max_len) = fixed_len {
                    quote! { #type_name <#t, #max_len> }
                } else {
                    quote! { #type_name <#t> }
                }
            }

            FieldType::Custom(t) => quote! {#t},
        };

        // TODO make it optional
        if field.boxed {
            quote! { Box<#typ> }
        } else {
            typ
        }
    }

    fn field_decl(&self, field: &Field) -> TokenStream {
        let typ = self.rust_type(field);
        let name = field.name;
        let attrs = &field.attrs;
        quote! { #attrs #name : #typ, }
    }

    fn field_default(&self, field: &Field) -> TokenStream {
        let name = field.name;
        let micropb_path = &self.config.micropb_path;
        if let Some(default) = field.default {
            match field.ftype {
                FieldType::Single(ref t) | FieldType::Optional(ref t) => {
                    return match t.typ {
                        Type::String => {
                            let string = format!("\"{}\"", default.escape_default());
                            quote! { #name: #micropb_path::PbString::from_str(#string).expect("default string went over capacity"), }
                        }
                        Type::Bytes => {
                            let bytes: String = unescape_c_escape_string(default)
                                .into_iter()
                                .flat_map(|b| core::ascii::escape_default(b).map(|c| c as char))
                                .collect();
                            let bstr = format!("b\"{bytes}\"");
                            quote! { #name: #micropb_path::PbVec::from_slice(#bstr).expect("default bytes went over capacity"), }
                        }
                        Type::Message => {
                            unreachable!("message fields shouldn't have custom defaults")
                        }
                        _ => quote! { #name: #default.into(), },
                    }
                }
                FieldType::Custom(_) => {}
                _ => unreachable!("repeated and map fields shouldn't have custom defaults"),
            }
        }
        quote! { #name: core::default::Default::default(), }
    }

    fn resolve_ident(&self, pb_ident: &str) -> TokenStream {
        assert_eq!(".", &pb_ident[1..]);

        let mut ident_path = pb_ident[1..].split('.');
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
            .map(|_| Cow::Borrowed("super"))
            .chain(ident_path.map(|e| self.resolve_path_elem(e)));
        quote! { #(#path ::)* #ident_type }
    }

    fn resolve_path_elem<'a>(&self, elem: &'a str) -> Cow<'a, str> {
        // Assume that type names all start with uppercase
        if elem.starts_with(|c: char| c.is_ascii_uppercase()) {
            Cow::Owned(format!("mod_{elem}"))
        } else {
            Cow::Borrowed(elem)
        }
    }

    fn strip_enum_prefix<'a>(&self, variant_name: &'a str, enum_name: &str) -> &'a str {
        if self.config.strip_enum_prefix {
            variant_name.strip_prefix(enum_name).unwrap_or(variant_name)
        } else {
            variant_name
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

fn unescape_c_escape_string(s: &str) -> Vec<u8> {
    let src = s.as_bytes();
    let len = src.len();
    let mut dst = Vec::new();

    let mut p = 0;

    while p < len {
        if src[p] != b'\\' {
            dst.push(src[p]);
            p += 1;
        } else {
            p += 1;
            if p == len {
                panic!(
                    "invalid c-escaped default binary value ({}): ends with '\'",
                    s
                )
            }
            match src[p] {
                b'a' => {
                    dst.push(0x07);
                    p += 1;
                }
                b'b' => {
                    dst.push(0x08);
                    p += 1;
                }
                b'f' => {
                    dst.push(0x0C);
                    p += 1;
                }
                b'n' => {
                    dst.push(0x0A);
                    p += 1;
                }
                b'r' => {
                    dst.push(0x0D);
                    p += 1;
                }
                b't' => {
                    dst.push(0x09);
                    p += 1;
                }
                b'v' => {
                    dst.push(0x0B);
                    p += 1;
                }
                b'\\' => {
                    dst.push(0x5C);
                    p += 1;
                }
                b'?' => {
                    dst.push(0x3F);
                    p += 1;
                }
                b'\'' => {
                    dst.push(0x27);
                    p += 1;
                }
                b'"' => {
                    dst.push(0x22);
                    p += 1;
                }
                b'0'..=b'7' => {
                    let mut octal = 0;
                    for _ in 0..3 {
                        if p < len && src[p] >= b'0' && src[p] <= b'7' {
                            octal = octal * 8 + (src[p] - b'0');
                            p += 1;
                        } else {
                            break;
                        }
                    }
                    dst.push(octal);
                }
                b'x' | b'X' => {
                    if p + 3 > len {
                        panic!(
                            "invalid c-escaped default binary value ({}): incomplete hex value",
                            s
                        )
                    }
                    match u8::from_str_radix(&s[p + 1..p + 3], 16) {
                        Ok(b) => dst.push(b),
                        _ => panic!(
                            "invalid c-escaped default binary value ({}): invalid hex value",
                            &s[p..p + 2]
                        ),
                    }
                    p += 3;
                }
                _ => panic!(
                    "invalid c-escaped default binary value ({}): invalid escape",
                    s
                ),
            }
        }
    }
    dst
}
