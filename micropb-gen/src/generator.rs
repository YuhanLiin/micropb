use std::{borrow::Cow, cell::RefCell, collections::HashMap, iter, ops::Deref};

use convert_case::{Case, Casing};
use protox::prost_reflect::prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};

use crate::{
    config::{Config, CustomField, GenConfig, IntType},
    pathtree::Node,
};

static DERIVE_MSG: &str = "#[derive(Clone, PartialEq)]";
static DERIVE_ENUM: &str = "#[derive(Clone, Copy, PartialEq, Eq, Hash)]";
static DERIVE_DEFAULT: &str = "#[derive(Default)]";
static DERIVE_DEBUG: &str = "#[derive(Debug)]";
static REPR_ENUM: &str = "#[repr(transparent)]";

static HAZZER_TYPE: &str = "_Has";
static HAZZER_NAME: &str = "_has";

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

enum TypeOpts {
    Name(String),
    Int(Option<IntType>),
    Container { typ: String, max_bytes: Option<u32> },
}

struct TypeSpec {
    typ: Type,
    opts: TypeOpts,
}

enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map {
        key: TypeSpec,
        val: TypeSpec,
        packed: bool,
        type_name: String,
        max_len: Option<u32>,
    },
    // Implicit presence
    Single(TypeSpec),
    // Explicit presence
    Optional(TypeSpec),
    Repeated {
        typ: TypeSpec,
        packed: bool,
        type_name: String,
        max_len: Option<u32>,
    },
    Custom(String),
    Delegate(String),
}

struct Field<'a> {
    num: u32,
    ftype: FieldType,
    name: &'a str,
    rust_name: Cow<'a, str>,
    default: Option<&'a str>,
    oneof: Option<usize>,
    boxed: bool,
    no_hazzer: bool,
    attrs: String,
}

impl<'a> Field<'a> {
    fn explicit_presence(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_))
    }

    fn is_hazzer(&self) -> bool {
        self.explicit_presence() && !self.boxed && !self.no_hazzer && self.oneof.is_none()
    }

    fn rust_variant_name(&self) -> String {
        self.rust_name.to_case(Case::Pascal)
    }

    fn delegate(&self) -> Option<&str> {
        if let FieldType::Delegate(d) = &self.ftype {
            Some(d)
        } else {
            None
        }
    }

    fn custom_field(&self) -> Option<&str> {
        if let FieldType::Custom(_) = &self.ftype {
            Some(&self.rust_name)
        } else {
            None
        }
    }
}

enum OneofType<'a> {
    Enum {
        type_name: String,
        fields: Vec<Field<'a>>,
    },
    Custom(String),
    Delegate(String),
}

struct Oneof<'a> {
    name: &'a str,
    rust_name: Cow<'a, str>,
    otype: OneofType<'a>,
    boxed: bool,
    field_attrs: String,
    type_attrs: String,
    derive_dbg: &'static str,
    idx: usize,
}

impl<'a> Oneof<'a> {
    fn delegate(&self) -> Option<&str> {
        if let OneofType::Delegate(d) = &self.otype {
            Some(d)
        } else {
            None
        }
    }

    fn custom_field(&self) -> Option<&str> {
        if let OneofType::Custom(_) = &self.otype {
            Some(&self.rust_name)
        } else {
            None
        }
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

    fn derive_dbg(&self) -> &'static str {
        if self.config.no_debug_derive.unwrap_or(false) {
            ""
        } else {
            DERIVE_DEBUG
        }
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

        let mut buf = String::with_capacity(100);

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

        for m in &fdproto.message_type {
            self.generate_msg_type(&mut buf, m, cur_config.next_conf(m.name()));
        }
        for e in &fdproto.enum_type {
            self.generate_enum_type(&mut buf, e, cur_config.next_conf(e.name()));
        }
    }

    fn generate_enum_type(
        &self,
        buf: &mut String,
        enum_type: &EnumDescriptorProto,
        enum_conf: CurrentConfig,
    ) {
        if enum_conf.config.skip.unwrap_or(false) {
            return;
        }

        let name = enum_type.name.as_ref().unwrap();
        let enum_int_type = enum_conf.config.enum_int_type.unwrap_or(IntType::I32);
        let itype = enum_int_type.type_name();

        *buf += DERIVE_ENUM;
        *buf += enum_conf.derive_dbg();
        *buf += REPR_ENUM;
        *buf += enum_conf.config.type_attributes.as_deref().unwrap_or("");
        *buf += "\n";
        *buf += &format!("pub struct {name}(pub {itype});\n");

        *buf += &format!("impl {name} {{\n");
        for v in &enum_type.value {
            let vname = v.name.as_ref().unwrap().to_case(Case::Pascal);
            let vname = self.strip_enum_prefix(&vname, name);
            *buf += &format!("pub const {vname}: Self = {name}({})\n", v.number.unwrap());
        }
        *buf += "}\n";

        let default_num = enum_type.value[0].number.unwrap();
        *buf += &format!(
            r#"impl core::default::Default for {name} {{
                fn default() -> Self {{ {name}({default_num}) }}
            }}
            "#
        );

        *buf += &format!(
            r#"impl core::convert::From<{itype}> for {name} {{
                fn from(val: {itype}) -> Self {{ {name}(val) }}
            }}
            "#
        );
    }

    fn generate_msg_type(
        &self,
        buf: &mut String,
        msg_type: &DescriptorProto,
        msg_conf: CurrentConfig,
    ) {
        if msg_conf.config.skip.unwrap_or(false) {
            return;
        }

        let name = msg_type.name.as_ref().unwrap();
        let fq_name = self.fq_name(name);
        let msg_mod_name = format!("mod_{name}");

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
            let ocustoms = oneofs.iter().filter_map(|o| o.custom_field());
            let fcustoms = fields.iter().filter_map(|f| f.custom_field());
            if ocustoms.chain(fcustoms).any(|custom| delegate == custom) {
                // TODO error about how delegate != custom
            }
        }

        self.type_path.borrow_mut().push(name.to_owned());
        *buf += &format!("pub mod {msg_mod_name} {{\n");

        for m in inner_msgs {
            self.generate_msg_type(buf, m, msg_conf.next_conf(m.name()));
        }
        for e in &msg_type.enum_type {
            self.generate_enum_type(buf, e, msg_conf.next_conf(e.name()));
        }
        for oneof in &oneofs {
            self.generate_oneof_decl(buf, oneof);
        }

        let opt_fields: Vec<_> = fields.iter().filter(|f| f.explicit_presence()).collect();
        let hazzer_exists = !opt_fields.is_empty();
        if hazzer_exists {
            self.generate_hazzer_decl(buf, &opt_fields, &msg_conf);
        }

        *buf += "}\n";
        self.type_path.borrow_mut().pop();

        let derive_default = fields.iter().any(|f| f.default.is_some());
        *buf += msg_conf.derive_dbg();
        *buf += derive_default.then_some(DERIVE_DEFAULT).unwrap_or("");
        *buf += DERIVE_MSG;
        *buf += &msg_conf.config.type_attributes.as_deref().unwrap_or("");

        *buf += &format!("\npub struct {name} {{\n");
        for field in &fields {
            self.generate_field_decl(buf, field);
        }
        for oneof in &oneofs {
            self.generate_oneof_field_decl(buf, &msg_mod_name, oneof);
        }
        if hazzer_exists {
            *buf += &format!("pub {HAZZER_NAME}: {HAZZER_TYPE},\n");
        }
        *buf += "}\n";

        if !derive_default {
            *buf += &format!("impl core::default::Default for {name} {{\n");
            *buf += "fn default() -> Self {\nSelf {\n";
            for field in &fields {
                self.generate_field_default(buf, field);
            }
            if hazzer_exists {
                *buf += &format!("{HAZZER_NAME}: core::default::Default::default(),\n");
            }
            *buf += "}\n}\n}\n";
        }
    }

    fn generate_oneof_decl(&self, buf: &mut String, oneof: &Oneof) {
        if let OneofType::Enum { type_name, fields } = &oneof.otype {
            *buf += oneof.derive_dbg;
            *buf += DERIVE_MSG;
            *buf += &oneof.type_attrs;

            *buf += &format!("\npub enum {type_name} {{\n");
            for field in fields {
                *buf += &field.attrs;
                buf.push(' ');
                *buf += &field.rust_variant_name();
                buf.push('(');
                *buf += &self.rust_type(field);
                *buf += "),\n";
            }
            *buf += "}\n";
        }
    }

    fn generate_hazzer_decl(&self, buf: &mut String, fields: &[&Field], msg_conf: &CurrentConfig) {
        *buf += msg_conf.derive_dbg();
        *buf += DERIVE_MSG;
        *buf += DERIVE_DEFAULT;
        *buf += msg_conf.config.hazzer_attributes.as_deref().unwrap_or("");

        let micropb_path = &self.config.micropb_path;
        let hazzers = fields.iter().filter(|f| f.is_hazzer());
        let count = hazzers.clone().count();
        *buf += &format!(
            "\npub struct {HAZZER_TYPE}({micropb_path}::bitvec::BitArr![for {count}, in u8]);\n"
        );

        *buf += &format!("impl {HAZZER_NAME} {{\n");
        for (i, f) in hazzers.enumerate() {
            let fname = f.name;
            *buf += &format!(
                r#"
                #[inline]
                pub fn {fname}(&self) -> bool {{

                   self.0[{i}]
                }}

                #[inline]
                pub fn set_{fname}(&mut self, val: bool) -> bool {{
                   self.0.set({i}, val);
                }}
                "#
            );
        }
        *buf += "}\n"
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
        let rust_name = oneof_conf
            .config
            .rename_field
            .as_ref()
            .map(|n| Cow::Owned(n.to_owned()))
            .unwrap_or(Cow::Borrowed(name));
        let otype = match &oneof_conf.config.custom_field {
            Some(CustomField::Type(type_name)) => OneofType::Custom(type_name.to_owned()),
            Some(CustomField::Delegate(delegate)) => OneofType::Delegate(delegate.to_owned()),
            None => OneofType::Enum {
                type_name: name.to_case(Case::Pascal),
                fields: vec![],
            },
        };

        Some(Oneof {
            name,
            rust_name,
            idx,
            otype,
            derive_dbg: oneof_conf.derive_dbg(),
            boxed: oneof_conf.config.boxed.unwrap_or(false),
            field_attrs: oneof_conf
                .config
                .field_attributes
                .clone()
                .unwrap_or_default(),
            type_attrs: oneof_conf
                .config
                .type_attributes
                .clone()
                .unwrap_or_default(),
        })
    }

    fn create_type_spec(
        &self,
        proto: &FieldDescriptorProto,
        type_conf: &CurrentConfig,
    ) -> TypeSpec {
        let conf = &type_conf.config;
        let typ = proto.r#type();
        let opts = match typ {
            Type::String => TypeOpts::Container {
                typ: conf.string_type.clone().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Bytes => TypeOpts::Container {
                typ: conf.vec_type.clone().unwrap(),
                max_bytes: conf.max_bytes,
            },
            Type::Enum | Type::Message => TypeOpts::Name(proto.type_name().to_owned()),
            _ => TypeOpts::Int(conf.int_type),
        };
        TypeSpec { typ, opts }
    }

    fn create_field<'a>(
        &self,
        proto: &'a FieldDescriptorProto,
        field_conf: CurrentConfig,
    ) -> Option<Field<'a>> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let name = proto.name();
        let rust_name = field_conf
            .config
            .rename_field
            .as_ref()
            .map(|n| Cow::Owned(n.to_owned()))
            .unwrap_or(Cow::Borrowed(name));
        let num = proto.number.unwrap() as u32;
        let oneof = proto.oneof_index.map(|i| i as usize);

        let ftype = match (&field_conf.config.custom_field, proto.label()) {
            (Some(CustomField::Type(type_name)), _) => FieldType::Custom(type_name.to_owned()),
            (Some(CustomField::Delegate(delegate)), _) if oneof.is_none() => {
                FieldType::Delegate(delegate.to_owned())
            }
            (_, Label::Repeated) => FieldType::Repeated {
                typ: self.create_type_spec(proto, &field_conf.next_conf("elem")),
                type_name: field_conf.config.vec_type.clone().unwrap(),
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

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            oneof,
            default: proto.default_value.as_deref(),
            boxed: field_conf.config.boxed.unwrap_or(false),
            no_hazzer: field_conf.config.no_hazzer.unwrap_or(false),
            attrs: field_conf
                .config
                .field_attributes
                .clone()
                .unwrap_or_default(),
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
        let rust_name = field_conf
            .config
            .rename_field
            .as_ref()
            .map(|n| Cow::Owned(n.to_owned()))
            .unwrap_or(Cow::Borrowed(name));
        let num = proto.number.unwrap() as u32;

        let ftype = match field_conf.config.custom_field {
            Some(CustomField::Type(type_name)) => FieldType::Custom(type_name),
            Some(CustomField::Delegate(delegate)) => FieldType::Delegate(delegate),
            None => {
                let key = self.create_type_spec(&map_msg.field[0], &field_conf.next_conf("key"));
                let val = self.create_type_spec(&map_msg.field[1], &field_conf.next_conf("value"));
                FieldType::Map {
                    key,
                    val,
                    type_name: field_conf.config.vec_type.clone().unwrap(),
                    max_len: field_conf.config.max_len,
                    packed: proto
                        .options
                        .as_ref()
                        .and_then(|opt| opt.packed)
                        .unwrap_or(false),
                }
            }
        };

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            oneof: None,
            default: None,
            boxed: field_conf.config.boxed.unwrap_or(false),
            no_hazzer: field_conf.config.no_hazzer.unwrap_or(false),
            attrs: field_conf
                .config
                .field_attributes
                .clone()
                .unwrap_or_default(),
        })
    }

    fn tspec_rust_type<'a>(&self, tspec: &'a TypeSpec) -> Cow<'a, str> {
        fn int_type<'a>(tspec: &TypeSpec, default: &'a str) -> &'a str {
            let TypeOpts::Int(itype) = tspec.opts else {
                unreachable!()
            };
            itype.map(IntType::type_name).unwrap_or(default)
        }

        match tspec.typ {
            Type::Int32 | Type::Sint32 | Type::Sfixed32 => int_type(tspec, "i32").into(),
            Type::Int64 | Type::Sint64 | Type::Sfixed64 => int_type(tspec, "i64").into(),
            Type::Uint32 | Type::Fixed32 => int_type(tspec, "u32").into(),
            Type::Uint64 | Type::Fixed64 => int_type(tspec, "u64").into(),
            Type::Float => "f32".into(),
            Type::Double => "f64".into(),
            Type::Bool => "bool".into(),
            Type::String => {
                let TypeOpts::Container { typ, max_bytes } = &tspec.opts else {
                    unreachable!()
                };
                if let Some(max_bytes) = max_bytes {
                    format!("{typ}<{max_bytes}>").into()
                } else {
                    typ.into()
                }
            }
            Type::Bytes => {
                let TypeOpts::Container { typ, max_bytes } = &tspec.opts else {
                    unreachable!()
                };
                if let Some(max_bytes) = max_bytes {
                    format!("{typ}<u8, {max_bytes}>").into()
                } else {
                    format!("{typ}<u8>").into()
                }
            }
            Type::Message | Type::Enum => {
                let TypeOpts::Name(tname) = &tspec.opts else {
                    unreachable!()
                };
                self.resolve_type_name(tname).into()
            }
            Type::Group => panic!("Group records are deprecated and unsupported"),
        }
    }

    fn rust_type<'a>(&self, field: &'a Field) -> Cow<'a, str> {
        let typ = match &field.ftype {
            FieldType::Map {
                key,
                val,
                type_name,
                max_len,
                ..
            } => {
                let k = self.tspec_rust_type(key);
                let v = self.tspec_rust_type(val);
                if let Some(max_len) = max_len {
                    format!("{type_name}<{k}, {v}, {max_len}>").into()
                } else {
                    format!("{type_name}<{k}, {v}>").into()
                }
            }

            FieldType::Single(t) | FieldType::Optional(t) => self.tspec_rust_type(t),

            FieldType::Repeated {
                typ,
                type_name,
                max_len,
                ..
            } => {
                let t = self.tspec_rust_type(typ);
                if let Some(max_len) = max_len {
                    format!("{type_name}<{t}, {max_len}>").into()
                } else {
                    format!("{type_name}<{t}>").into()
                }
            }

            FieldType::Custom(t) => t.into(),
            FieldType::Delegate(_) => unreachable!("delegate field cannot have a type"),
        };

        if field.boxed {
            let box_type = self.box_type();
            if field.explicit_presence() {
                format!("core::option::Option<{box_type}<{typ}>>").into()
            } else {
                format!("{box_type}<{typ}>").into()
            }
        } else {
            typ
        }
    }

    fn generate_field_decl(&self, buf: &mut String, field: &Field) {
        if let FieldType::Delegate(_) = field.ftype {
            return;
        }
        *buf += &field.attrs;
        buf.push(' ');
        *buf += &field.rust_name;
        buf.push(':');
        *buf += &self.rust_type(field);
        *buf += ",\n";
    }

    fn generate_oneof_field_decl(&self, buf: &mut String, msg_mod_name: &str, oneof: &Oneof) {
        let type_name: Cow<str> = match &oneof.otype {
            OneofType::Enum { type_name, .. } => format!("{msg_mod_name}::{}", type_name).into(),
            OneofType::Custom(type_name) => type_name.into(),
            OneofType::Delegate(_) => return,
        };
        *buf += &oneof.field_attrs;
        buf.push(' ');
        *buf += &oneof.rust_name;
        *buf += ": core::option::Option";
        *buf += &if oneof.boxed {
            let box_type = self.box_type();
            format!("<{box_type}<{type_name}>>")
        } else {
            format!("<{type_name}>")
        };
        *buf += ",\n";
    }

    fn tspec_default(&self, t: &TypeSpec, default: &str) -> String {
        let micropb_path = &self.config.micropb_path;
        match t.typ {
            Type::String => {
                let string = format!("\"{}\"", default.escape_default());
                format!(
                    r#"{micropb_path}::PbString::from_str({string}).expect("default string went over capacity")"#
                )
            }
            Type::Bytes => {
                let bytes: String = unescape_c_escape_string(default)
                    .into_iter()
                    .flat_map(|b| core::ascii::escape_default(b).map(|c| c as char))
                    .collect();
                let bstr = format!("b\"{bytes}\"");
                format!(
                    r#"{micropb_path}::PbVec::from_slice({bstr}).expect("default bytes went over capacity")"#
                )
            }
            Type::Message => {
                unreachable!("message fields shouldn't have custom defaults")
            }
            Type::Enum => {
                let TypeOpts::Name(tname) = &t.opts else {
                    unreachable!()
                };
                let default = default.to_case(Case::Pascal);
                let variant = self.strip_enum_prefix(
                    &default,
                    tname.rsplit_once('.').map(|(_, s)| s).unwrap_or(tname),
                );
                format!("{tname}::{variant}")
            }
            _ => format!("{default} as _"),
        }
    }

    fn generate_field_default(&self, buf: &mut String, field: &Field) {
        let name = field.name;
        if let Some(default) = field.default {
            match field.ftype {
                FieldType::Single(ref t) | FieldType::Optional(ref t) => {
                    let value = self.tspec_default(t, default);
                    return if field.boxed {
                        let box_type = self.box_type();
                        if field.explicit_presence() {
                            *buf += &format!(
                                "{name}: core::option::Option::Some({box_type}::new({value})),\n"
                            );
                        } else {
                            *buf += &format!("{name}: {box_type}::new({value}),\n");
                        }
                    } else {
                        *buf += &format!("{name}: {value},\n");
                    };
                }
                FieldType::Delegate(_) => return,
                FieldType::Custom(_) => {}
                _ => unreachable!("repeated and map fields shouldn't have custom defaults"),
            }
        }
        *buf += &format!("{name}: core::default::Default::default(),\n");
    }

    fn resolve_type_name(&self, pb_fq_type_name: &str) -> String {
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
            .map(|_| Cow::Borrowed("super"))
            .chain(ident_path.map(|e| self.resolve_path_elem(e)))
            .fold(String::new(), |s, segment| s + "::" + &segment);
        path + ident_type
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

    fn box_type(&self) -> &'static str {
        if self.config.use_std {
            "std::boxed::Box"
        } else {
            "alloc::boxed::Box"
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
