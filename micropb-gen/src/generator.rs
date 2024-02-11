use std::{borrow::Cow, cell::RefCell};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use protox::prost_reflect::prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet,
};
use quote::quote;

static DERIVE_MSG: &str = "#[derive(Debug, Clone, PartialEq)]";
static DERIVE_ENUM: &str = "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]";
static DERIVE_DEFAULT: &str = "#[derive(Default)]";
static REPR_ENUM: &str = "#[repr(i32)]";

#[derive(Debug, Clone, Copy, Default)]
enum EncodeDecode {
    EncodeOnly,
    DecodeOnly,
    #[default]
    Both,
}

pub struct GenConfig {
    encode_decode: EncodeDecode,
    size_cache: bool,
    default_pkg_filename: String,
    prefix_module: String,
    vec_type: String,
    string_type: String,
    map_type: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Syntax {
    #[default]
    Proto2,
    Proto3,
}

struct TypeSpec {
    typ: Type,
    name: Option<String>,
}

enum FieldType {
    // Can't be put in oneof, key type can't be message or enum
    Map(TypeSpec, TypeSpec),
    // Implicit presence
    Single(TypeSpec),
    // Explicit presence
    Optional(TypeSpec),
    Repeated { typ: TypeSpec, packed: bool },
    Custom(String),
}

#[derive(Debug, Default)]
struct FieldOptions {
    max_bytes: Option<u32>,
    max_len: Option<u32>,
}

struct Field<'a> {
    num: u32,
    ftype: FieldType,
    name: &'a str,
    options: FieldOptions,
    default: Option<&'a str>,
    oneof: Option<&'a str>,
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

        let msgs = fdproto
            .message_type
            .iter()
            .map(|m| self.generate_msg_type(m));
        let enums = fdproto.enum_type.iter().map(|e| self.generate_enum_type(e));

        let code = quote! {
            #(#msgs)*
            #(#enums)*
        };
    }

    fn generate_enum_type(&self, enum_type: &EnumDescriptorProto) -> TokenStream {
        let name = enum_type.name.as_ref().unwrap();
        let nums = enum_type.value.iter().map(|v| v.number.unwrap());
        let var_names = enum_type
            .value
            .iter()
            .map(|v| v.name.as_ref().unwrap().to_case(Case::Pascal));
        let default_num = enum_type.value[0].number.unwrap();

        quote! {
            #DERIVE_ENUM
            #REPR_ENUM
            pub struct #name(pub i32);

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

    fn generate_msg_type(&self, msg_type: &DescriptorProto) -> TokenStream {
        let name = msg_type.name.as_ref().unwrap();
        let oneofs: Vec<_> = msg_type
            .oneof_decl
            .iter()
            .map(|oneof| oneof.name.as_deref().unwrap())
            .collect();
        let oneofs_types: Vec<_> = oneofs.iter().map(|o| o.to_case(Case::Pascal)).collect();
        let (fields, oneof_fields): (Vec<_>, Vec<_>) = msg_type
            .field
            .iter()
            .map(|f| self.create_field(f, &oneofs))
            .partition(|f| f.oneof.is_none());
        let msg_mod_name = format!("mod_{name}");

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

        let nested_msgs = msg_type
            .nested_type
            .iter()
            .map(|m| self.generate_msg_type(m));
        let nested_enums = msg_type
            .enum_type
            .iter()
            .map(|e| self.generate_enum_type(e));
        let msg_mod = quote! {
            pub mod #msg_mod_name {
                #(#oneof_decls)*
                #(#nested_msgs)*
                #(#nested_enums)*
            }
        };
        self.type_path.borrow_mut().pop();

        let msg_fields = fields.iter().map(|f| self.field_decl(f));
        let (derive_default, decl_default) = if fields.iter().any(|f| f.default.is_some()) {
            let defaults = fields.iter().map(|f| self.field_default(f));
            let decl = quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self {
                            #(#defaults)*
                        }
                    }
                }
            };
            (None, Some(decl))
        } else {
            (Some(DERIVE_DEFAULT), None)
        };

        quote! {
            #msg_mod

            #DERIVE_MSG
            #derive_default
            pub struct #name {
                #(pub #msg_fields)*
                #(pub #oneofs: Option<#msg_mod_name::#oneofs_types>)*
            }

            #decl_default
        }
    }

    fn create_field<'a>(&self, proto: &'a FieldDescriptorProto, oneofs: &[&str]) -> Field<'a> {
        let name = proto.name.as_ref().unwrap();
        let num = proto.number.unwrap() as u32;

        let tspec = TypeSpec {
            typ: proto.r#type(),
            name: proto.name.clone(),
        };
        let ftype = match proto.label() {
            Label::Repeated => FieldType::Repeated {
                typ: tspec,
                packed: proto
                    .options
                    .as_ref()
                    .and_then(|opt| opt.packed)
                    .unwrap_or(false),
            },
            Label::Required => FieldType::Optional(tspec),
            Label::Optional
                if self.syntax == Syntax::Proto2
                    || proto.proto3_optional()
                    || tspec.typ == Type::Message =>
            {
                FieldType::Optional(tspec)
            }
            _ => FieldType::Single(tspec),
        };
        let oneof = proto.oneof_index.map(|i| oneofs[i as usize]);
        let default = proto.default_value.as_deref();

        Field {
            num,
            ftype,
            name,
            oneof,
            default,
            options: todo!(),
        }
    }

    fn tspec_rust_type(&self, tspec: &TypeSpec, options: &FieldOptions) -> TokenStream {
        match tspec.typ {
            Type::Int32 => quote! {i32},
            Type::Int64 => quote! {i64},
            Type::Uint32 => quote! {u32},
            Type::Uint64 => quote! {u64},
            Type::Sint32 => quote! {i32},
            Type::Sint64 => quote! {i64},
            Type::Fixed32 => quote! {u32},
            Type::Fixed64 => quote! {u64},
            Type::Sfixed32 => quote! {i32},
            Type::Sfixed64 => quote! {i64},
            Type::Float => quote! {f32},
            Type::Double => quote! {f64},
            Type::Bool => quote! {bool},
            Type::String => {
                let str_type = &self.config.string_type;
                let max_bytes = options.max_bytes.as_ref().unwrap();
                quote! { #str_type <#max_bytes> }
            }
            Type::Bytes => {
                let vec_type = &self.config.vec_type;
                let max_bytes = options.max_bytes.as_ref().unwrap();
                quote! { #vec_type <u8, #max_bytes> }
            }
            Type::Message | Type::Enum => self.resolve_ident(tspec.name.as_ref().unwrap()),
            Type::Group => panic!("Group records are deprecated and unsupported"),
        }
    }

    fn rust_type(&self, field_type: &FieldType, options: &FieldOptions) -> TokenStream {
        match field_type {
            FieldType::Map(k, v) => {
                let k = self.tspec_rust_type(k, options);
                let v = self.tspec_rust_type(v, options);
                let map_type = &self.config.map_type;
                let max_len = options.max_len.as_ref().unwrap();
                quote! { #map_type <#k, #v, #max_len> }
            }
            FieldType::Single(t) | FieldType::Optional(t) => self.tspec_rust_type(t, options),
            FieldType::Repeated { typ, .. } => {
                let vec_type = &self.config.vec_type;
                let max_len = options.max_len.as_ref().unwrap();
                let t = self.tspec_rust_type(typ, options);
                quote! { #vec_type <#t, #max_len> }
            }
            FieldType::Custom(t) => quote! {#t},
        }
    }

    fn field_decl(&self, field: &Field) -> TokenStream {
        let typ = self.rust_type(&field.ftype, &field.options);
        let name = field.name;
        quote! { #name : #typ, }
    }

    fn field_default(&self, field: &Field) -> TokenStream {
        let name = field.name;
        if let Some(default) = field.default {
            match field.ftype {
                FieldType::Single(ref t) | FieldType::Optional(ref t) => {
                    return match t.typ {
                        Type::String => todo!(),
                        Type::Bytes => todo!(),
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
        quote! { #name: Default::default(), }
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
}
