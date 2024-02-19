use std::{borrow::Cow, cell::RefCell};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use protox::prost_reflect::{
    prost_types::{
        field_descriptor_proto::{Label, Type},
        DescriptorProto, FieldDescriptorProto,
    },
    Cardinality, DescriptorPool, EnumDescriptor, FieldDescriptor, FileDescriptor, Kind,
    MessageDescriptor, Syntax,
};
use quote::quote;

static DERIVE_MSG: &str = "#[derive(Debug, Clone, PartialEq)]";
static DERIVE_ENUM: &str = "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]";
static DERIVE_DEFAULT: &str = "#[derive(Default)]";
static REPR_ENUM: &str = "#[repr(transparent)]";

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
    micropb_path: String,
    strip_enum_prefix: bool,
    vec_type: String,
    string_type: String,
    map_type: String,
}

#[derive(Debug, Default)]
struct FieldOptions {
    max_bytes: Option<u32>,
    max_len: Option<u32>,
    kv_options: Option<(Box<FieldOptions>, Box<FieldOptions>)>,
}

struct Generator {
    config: GenConfig,
    syntax: Syntax,
    pkg_path: Vec<String>,
    type_path: RefCell<Vec<String>>,
}

impl Generator {
    fn generate_fdset(&mut self, fdset: DescriptorPool) {
        for file in fdset.files() {
            self.generate_fdproto(file);
        }
    }

    fn generate_fdproto(&mut self, fdproto: FileDescriptor) {
        let mut filename = fdproto.package_name();
        if filename.is_empty() {
            filename = &self.config.default_pkg_filename;
        }

        self.syntax = fdproto.syntax();
        self.pkg_path = fdproto
            .package_name()
            .split('.')
            .map(ToOwned::to_owned)
            .collect();

        let msgs = fdproto.messages().map(|m| self.generate_msg_type(m));
        let enums = fdproto.enums().map(|e| self.generate_enum_type(e));

        let code = quote! {
            #(#msgs)*
            #(#enums)*
        };
    }

    fn generate_enum_type(&self, enum_type: EnumDescriptor) -> TokenStream {
        let name = enum_type.name();
        let nums = enum_type.values().map(|v| v.number());
        let var_names = enum_type
            .values()
            .map(|v| v.name().to_case(Case::Pascal))
            .map(|v| self.strip_enum_prefix(&v, name).to_owned());
        let default_num = enum_type.default_value().number();

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

    fn generate_msg_type(&self, msg_type: MessageDescriptor) -> TokenStream {
        let name = msg_type.name();
        let msg_mod_name = format!("mod_{name}");
        let fields: Vec<_> = msg_type
            .fields()
            .filter(|f| f.containing_oneof().is_none())
            .collect();
        let oneofs: Vec<_> = msg_type
            .oneofs()
            .map(|oneof| (oneof.clone(), oneof.name().to_case(Case::Pascal)))
            .collect();

        self.type_path.borrow_mut().push(name.to_owned());
        let oneof_decls = oneofs.iter().map(|(oneof, oneof_type)| {
            let fields = oneof.fields().map(|f| self.field_decl(&f));
            quote! {
                #DERIVE_MSG
                pub enum #oneof_type {
                    #(#fields)*
                }
            }
        });
        let nested_msgs = msg_type
            .child_messages()
            .filter(|m| !m.is_map_entry())
            .map(|m| self.generate_msg_type(m));
        let nested_enums = msg_type.child_enums().map(|e| self.generate_enum_type(e));

        let msg_mod = quote! {
            pub mod #msg_mod_name {
                #(#oneof_decls)*
                #(#nested_msgs)*
                #(#nested_enums)*
            }
        };
        self.type_path.borrow_mut().pop();

        let msg_fields = fields.iter().cloned().map(|f| self.field_decl(&f));
        let opt_fields: Vec<_> = fields.iter().filter(|f| f.supports_presence()).collect();
        let (hazzer_name, hazzer_decl) = if !opt_fields.is_empty() {
            let (n, t) = self.generate_hazzer(name, &opt_fields);
            (Some(n), Some(t))
        } else {
            (None, None)
        };
        let hazzer_field = hazzer_name.as_ref().map(|n| quote! { pub has: #n, });
        let oneof_names = oneofs.iter().map(|(oneof, _)| oneof.name());
        let oneof_types = oneofs.iter().map(|(_, typ)| typ);

        let (derive_default, decl_default) = if fields
            .iter()
            .any(|f| f.field_descriptor_proto().default_value.is_some())
        {
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

        quote! {
            #msg_mod

            #hazzer_decl

            #DERIVE_MSG
            #derive_default
            pub struct #name {
                #(pub #msg_fields)*
                #(pub #oneof_names: Option<#msg_mod_name::#oneof_types>)*
                #hazzer_field
            }

            #decl_default
        }
    }

    fn generate_hazzer(&self, name: &str, fields: &[&FieldDescriptor]) -> (String, TokenStream) {
        let count = fields.len();
        let micropb_path = &self.config.micropb_path;
        let hazzer_name = format!("{name}Hazzer");

        let methods = fields.iter().enumerate().map(|(i, f)| {
            let fname = f.name();
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
            #DERIVE_MSG
            #DERIVE_DEFAULT
            pub struct #hazzer_name(#micropb_path::bitvec::BitArr!(for #count, in u8));

            impl #hazzer_name {
                #(#methods)*
            }
        };
        (hazzer_name, decl)
    }

    fn map_fields(&self, field: &FieldDescriptor) -> (FieldDescriptor, FieldDescriptor) {
        assert!(field.is_map());
        let Kind::Message(msg) = field.kind() else {
            unreachable!()
        };
        (msg.map_entry_key_field(), msg.map_entry_value_field())
    }

    fn kind_rust_type(&self, kind: Kind, options: &FieldOptions) -> TokenStream {
        match kind {
            Kind::Int32 => quote! {i32},
            Kind::Int64 => quote! {i64},
            Kind::Uint32 => quote! {u32},
            Kind::Uint64 => quote! {u64},
            Kind::Sint32 => quote! {i32},
            Kind::Sint64 => quote! {i64},
            Kind::Fixed32 => quote! {u32},
            Kind::Fixed64 => quote! {u64},
            Kind::Sfixed32 => quote! {i32},
            Kind::Sfixed64 => quote! {i64},
            Kind::Float => quote! {f32},
            Kind::Double => quote! {f64},
            Kind::Bool => quote! {bool},
            Kind::String => {
                let str_type = &self.config.string_type;
                let max_bytes = options.max_bytes.as_ref().unwrap();
                quote! { #str_type <#max_bytes> }
            }
            Kind::Bytes => {
                let vec_type = &self.config.vec_type;
                let max_bytes = options.max_bytes.as_ref().unwrap();
                quote! { #vec_type <u8, #max_bytes> }
            }
            Kind::Message(msg) => self.resolve_ident(msg.full_name()),
            Kind::Enum(en) => self.resolve_ident(en.full_name()),
        }
    }

    fn rust_type(&self, field_type: &FieldDescriptor, options: &FieldOptions) -> TokenStream {
        if field_type.is_group() {
            panic!("Groups not supported")
        } else if field_type.is_map() {
            let (k_opt, v_opt) = options.kv_options.as_ref().unwrap();
            let (kfield, vfield) = self.map_fields(field_type);
            let k = self.kind_rust_type(kfield.kind(), k_opt);
            let v = self.kind_rust_type(vfield.kind(), v_opt);
            let map_type = &self.config.map_type;
            let max_len = options.max_len.as_ref().unwrap();
            quote! { #map_type <#k, #v, #max_len> }
        } else if field_type.is_list() {
            let vec_type = &self.config.vec_type;
            let max_len = options.max_len.as_ref().unwrap();
            let t = self.kind_rust_type(field_type.kind(), options);
            quote! { #vec_type <#t, #max_len> }
        } else {
            self.kind_rust_type(field_type.kind(), options)
        }
    }

    fn field_decl(&self, field: &FieldDescriptor) -> TokenStream {
        let typ = self.rust_type(&field, todo!());
        let name = field.name();
        quote! { #name : #typ, }
    }

    fn field_default(&self, field: &FieldDescriptor) -> TokenStream {
        let name = field.name();
        let micropb_path = &self.config.micropb_path;
        if let Some(default) = field.field_descriptor_proto().default_value.as_ref() {
            if let Cardinality::Repeated = field.cardinality() {
                unreachable!("repeated and map fields shouldn't have custom defaults");
            } else {
                return match field.kind() {
                    Kind::String => {
                        let string = format!("\"{}\"", default.escape_default());
                        quote! { #name: #micropb_path::PbString::from_str(#string).expect("default string went over capacity"), }
                    }
                    Kind::Bytes => {
                        let bytes: String = unescape_c_escape_string(default)
                            .into_iter()
                            .flat_map(|b| core::ascii::escape_default(b).map(|c| c as char))
                            .collect();
                        let bstr = format!("b\"{bytes}\"");
                        quote! { #name: #micropb_path::PbVec::from_slice(#bstr).expect("default bytes went over capacity"), }
                    }
                    Kind::Message(_) => {
                        unreachable!("message fields shouldn't have custom defaults")
                    }
                    _ => quote! { #name: #default.into(), },
                };
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
