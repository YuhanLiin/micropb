use convert_case::{Case, Casing};
use micropb::size::{
    sizeof_len_record, sizeof_sint32, sizeof_sint64, sizeof_varint32, sizeof_varint64,
};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Ident, Lifetime};

use crate::{
    config::{IntSize, byte_string_type_parsed, contains_len_param},
    descriptor::{FieldDescriptorProto, FieldDescriptorProto_::Type},
    generator::{Context, field_error_str, sanitized_ident},
    utils::{find_lifetime_from_str, path_suffix, unescape_c_escape_string},
};

use super::CurrentConfig;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum PbInt {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Sfixed32,
    Sfixed64,
    Fixed32,
    Fixed64,
}

impl PbInt {
    fn is_signed(&self) -> bool {
        matches!(
            self,
            PbInt::Int32
                | PbInt::Int64
                | PbInt::Sint32
                | PbInt::Sint64
                | PbInt::Sfixed32
                | PbInt::Sfixed64
        )
    }

    fn generate_decode_func(&self, int_size: &IntSize) -> Ident {
        let func = match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => "decode_int64",
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => "decode_varint64",
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => "decode_sint64",
            PbInt::Fixed64 if matches!(int_size, IntSize::S64) => "decode_fixed64",
            PbInt::Sfixed64 if matches!(int_size, IntSize::S64) => "decode_sfixed64",

            PbInt::Int32 | PbInt::Int64 => "decode_int32",
            PbInt::Uint32 | PbInt::Uint64 => "decode_varint32",
            PbInt::Sint32 | PbInt::Sint64 => "decode_sint32",
            PbInt::Sfixed32 => "decode_sfixed32",
            PbInt::Fixed32 => "decode_fixed32",
            PbInt::Sfixed64 => "decode_sfixed64_as_32",
            PbInt::Fixed64 => "decode_fixed64_as_32",
        };
        Ident::new(func, Span::call_site())
    }

    fn generate_sizeof(&self, int_size: &IntSize, val_ref: &Ident) -> TokenStream {
        match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_int64(* #val_ref as _) }
            }
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_varint64(* #val_ref as _) }
            }
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => {
                quote! { ::micropb::size::sizeof_sint64(* #val_ref as _) }
            }

            PbInt::Int32 | PbInt::Int64 => {
                quote! { ::micropb::size::sizeof_int32(* #val_ref as _) }
            }
            PbInt::Uint32 | PbInt::Uint64 => {
                quote! { ::micropb::size::sizeof_varint32(* #val_ref as _) }
            }
            PbInt::Sint32 | PbInt::Sint64 => {
                quote! { ::micropb::size::sizeof_sint32(* #val_ref as _) }
            }

            PbInt::Sfixed32 => quote! { 4 },
            PbInt::Fixed32 => quote! { 4 },
            PbInt::Sfixed64 => quote! { 8 },
            PbInt::Fixed64 => quote! { 8 },
        }
    }

    fn generate_encode_func(&self, int_size: &IntSize) -> Ident {
        let func = match self {
            PbInt::Int64 if matches!(int_size, IntSize::S64) => "encode_int64",
            PbInt::Uint64 if matches!(int_size, IntSize::S64) => "encode_varint64",
            PbInt::Sint64 if matches!(int_size, IntSize::S64) => "encode_sint64",
            PbInt::Sfixed64 if matches!(int_size, IntSize::S64) => "encode_sfixed64",
            PbInt::Fixed64 if matches!(int_size, IntSize::S64) => "encode_fixed64",

            PbInt::Int32 | PbInt::Int64 => "encode_int32",
            PbInt::Uint32 | PbInt::Uint64 => "encode_varint32",
            PbInt::Sint32 | PbInt::Sint64 => "encode_sint32",
            PbInt::Sfixed32 => "encode_sfixed32",
            PbInt::Fixed32 => "encode_fixed32",
            PbInt::Sfixed64 => "encode_sfixed64_as_32",
            PbInt::Fixed64 => "encode_fixed64_as_32",
        };
        Ident::new(func, Span::call_site())
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum TypeSpec<'proto> {
    Message(&'proto str),
    Enum(&'proto str),
    Float,
    Double,
    Bool,
    Int(PbInt, IntSize),
    String {
        typestr: String,
        max_bytes: Option<u32>,
    },
    Bytes {
        typestr: String,
        max_bytes: Option<u32>,
    },
}

impl<'proto> TypeSpec<'proto> {
    pub(crate) fn find_lifetime(&self) -> Option<Lifetime> {
        match self {
            TypeSpec::Bytes {
                typestr: type_path, ..
            }
            | TypeSpec::String {
                typestr: type_path, ..
            } => find_lifetime_from_str(type_path),
            _ => None,
        }
    }

    fn max_size(&self) -> Result<usize, &'static str> {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => Ok(4),
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => Ok(8),
            TypeSpec::Bool => Ok(1),

            // negative VARINT values will always take up 10 bytes
            TypeSpec::Int(PbInt::Int32 | PbInt::Int64, _) => Ok(10),

            // positive VARINT size depends on the max size of the represented int type
            TypeSpec::Int(PbInt::Uint32, intsize) => Ok(sizeof_varint32(
                intsize.max_value().try_into().unwrap_or(u32::MAX),
            )),
            TypeSpec::Int(PbInt::Uint64, intsize) => Ok(sizeof_varint64(intsize.max_value())),
            TypeSpec::Int(PbInt::Sint32, intsize) => Ok(sizeof_sint32(
                intsize.min_value().try_into().unwrap_or(i32::MAX),
            )),
            TypeSpec::Int(PbInt::Sint64, intsize) => Ok(sizeof_sint64(intsize.min_value())),

            TypeSpec::Bytes { max_bytes, .. } | TypeSpec::String { max_bytes, .. } => max_bytes
                .map(|max| sizeof_len_record(max as usize))
                .ok_or("unbounded string or bytes"),

            // Will be handled later
            TypeSpec::Message(..) | TypeSpec::Enum(..) => Ok(0),
        }
    }

    pub(crate) fn fixed_size(&self) -> Option<usize> {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => Some(4),
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => Some(8),
            TypeSpec::Bool => Some(1),
            _ => None,
        }
    }

    pub(crate) fn is_copy(&self, ctx: &Context<'proto>) -> bool {
        match self {
            TypeSpec::Message(name) => ctx
                .graph
                .get_message(name)
                .map(|msg| msg.is_copy)
                .unwrap_or(false),

            TypeSpec::Enum(_)
            | TypeSpec::Float
            | TypeSpec::Double
            | TypeSpec::Bool
            | TypeSpec::Int(..) => true,

            TypeSpec::String { .. } | TypeSpec::Bytes { .. } => false,
        }
    }

    pub(crate) fn from_proto(
        proto: &'proto FieldDescriptorProto,
        type_conf: &CurrentConfig,
    ) -> Result<Self, String> {
        let conf = &type_conf.config;
        let res = match proto.r#type {
            Type::Group => return Err("Group fields are unsupported".to_owned()),
            Type::Double => TypeSpec::Double,
            Type::Float => TypeSpec::Float,
            Type::Bool => TypeSpec::Bool,
            Type::String => {
                let typestr = conf
                    .string_type
                    .clone()
                    .ok_or_else(|| "string_type not configured".to_owned())?;
                TypeSpec::String {
                    max_bytes: conf.max_bytes.filter(|_| contains_len_param(&typestr)),
                    typestr,
                }
            }
            Type::Bytes => {
                let typestr = conf
                    .bytes_type
                    .clone()
                    .ok_or_else(|| "bytes_type not configured".to_owned())?;
                TypeSpec::Bytes {
                    max_bytes: conf.max_bytes.filter(|_| contains_len_param(&typestr)),
                    typestr,
                }
            }
            Type::Message => TypeSpec::Message(&proto.type_name),
            Type::Enum => TypeSpec::Enum(&proto.type_name),
            Type::Uint32 => TypeSpec::Int(PbInt::Uint32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Int64 => TypeSpec::Int(PbInt::Int64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Uint64 => TypeSpec::Int(PbInt::Uint64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Int32 => TypeSpec::Int(PbInt::Int32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Fixed64 => TypeSpec::Int(PbInt::Fixed64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Fixed32 => TypeSpec::Int(PbInt::Fixed32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sfixed32 => TypeSpec::Int(PbInt::Sfixed32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sfixed64 => TypeSpec::Int(PbInt::Sfixed64, conf.int_size.unwrap_or(IntSize::S64)),
            Type::Sint32 => TypeSpec::Int(PbInt::Sint32, conf.int_size.unwrap_or(IntSize::S32)),
            Type::Sint64 => TypeSpec::Int(PbInt::Sint64, conf.int_size.unwrap_or(IntSize::S64)),
            t => return Err(format!("Unknown type specifier {}", t.0)),
        };
        Ok(res)
    }

    pub(crate) fn generate_rust_type(&self, ctx: &Context<'proto>) -> Result<TokenStream, String> {
        let res = match self {
            TypeSpec::Int(pbint, itype) => {
                let typ = itype.type_name(pbint.is_signed());
                quote! { #typ }
            }
            TypeSpec::Float => quote! {f32},
            TypeSpec::Double => quote! {f64},
            TypeSpec::Bool => quote! {bool},
            TypeSpec::String { typestr, max_bytes } => {
                let ty = byte_string_type_parsed(typestr, *max_bytes)?;
                quote! { #ty }
            }
            TypeSpec::Bytes { typestr, max_bytes } => {
                let ty = byte_string_type_parsed(typestr, *max_bytes)?;
                quote! { #ty }
            }

            TypeSpec::Message(tname) => {
                let rust_type = ctx.resolve_type_name(tname);
                if let Some(lifetime) = ctx
                    .graph
                    .get_message(tname)
                    .and_then(|m| m.lifetime.as_ref())
                {
                    quote! { #rust_type<#lifetime> }
                } else {
                    quote! { #rust_type }
                }
            }
            TypeSpec::Enum(tname) => {
                let rust_type = ctx.resolve_type_name(tname);
                quote! { #rust_type }
            }
        };
        Ok(res)
    }

    pub(crate) fn generate_max_size(
        &self,
        ctx: &Context<'proto>,
        msg_name: &'proto str,
        fname: &'proto str,
    ) -> TokenStream {
        match self {
            TypeSpec::Message(tname) => {
                let rust_type = ctx.resolve_type_name(tname);
                return quote! { ::micropb::const_map!(<#rust_type as ::micropb::MessageEncode>::MAX_SIZE, |size| ::micropb::size::sizeof_len_record(size)) };
            }
            TypeSpec::Enum(tname) => {
                let rust_type = ctx.resolve_type_name(tname);
                return quote! { ::core::result::Result::Ok(#rust_type::_MAX_SIZE) };
            }
            _ => (),
        }

        self.max_size()
            .map(Literal::usize_suffixed)
            .map(|lit| quote! {::core::result::Result::Ok(#lit)})
            .unwrap_or_else(|err| {
                let err = field_error_str(&ctx.pkg, msg_name, fname, err);
                quote! {::core::result::Result::<usize, &'static str>::Err(#err)}
            })
    }

    pub(crate) fn generate_default(
        &self,
        default: &str,
        ctx: &Context<'proto>,
    ) -> Result<TokenStream, String> {
        let out = match self {
            TypeSpec::String { max_bytes, .. } => match *max_bytes {
                Some(max_bytes) if default.len() > max_bytes as usize => {
                    return Err(format!(
                        "String field is limited to {max_bytes} bytes, but its default value is {} bytes",
                        default.len()
                    ));
                }
                _ => quote! { ::core::convert::TryFrom::try_from(#default).unwrap_or_default() },
            },

            TypeSpec::Bytes { max_bytes, .. } => {
                let bytes = unescape_c_escape_string(default);
                let default_bytes = Literal::byte_string(&bytes);
                match *max_bytes {
                    Some(max_bytes) if bytes.len() > max_bytes as usize => {
                        return Err(format!(
                            "Bytes field is limited to {max_bytes} bytes, but its default value is {} bytes",
                            bytes.len()
                        ));
                    }
                    _ => {
                        quote! { ::core::convert::TryFrom::try_from(#default_bytes.as_slice()).unwrap_or_default() }
                    }
                }
            }

            TypeSpec::Message(..) => {
                unreachable!("message fields shouldn't have custom defaults")
            }

            TypeSpec::Enum(tpath) => {
                let enum_path = ctx.resolve_type_name(tpath);
                let enum_name = sanitized_ident(&path_suffix(tpath).to_case(Case::Pascal));
                let variant = ctx.enum_variant_name(default, &enum_name);
                quote! { #enum_path::#variant }
            }

            _ => {
                let default: TokenStream =
                    syn::parse_str(default).expect("default value tokenization error");
                quote! { #default as _ }
            }
        };
        Ok(out)
    }

    pub(crate) fn wire_type(&self) -> u8 {
        match self {
            TypeSpec::Float | TypeSpec::Int(PbInt::Fixed32 | PbInt::Sfixed32, _) => {
                micropb::WIRE_TYPE_I32
            }
            TypeSpec::Double | TypeSpec::Int(PbInt::Fixed64 | PbInt::Sfixed64, _) => {
                micropb::WIRE_TYPE_I64
            }
            TypeSpec::Enum(_)
            | TypeSpec::Bool
            | TypeSpec::Int(
                PbInt::Int32
                | PbInt::Int64
                | PbInt::Uint32
                | PbInt::Uint64
                | PbInt::Sint32
                | PbInt::Sint64,
                _,
            ) => micropb::WIRE_TYPE_VARINT,
            TypeSpec::Message(..) | TypeSpec::String { .. } | TypeSpec::Bytes { .. } => {
                micropb::WIRE_TYPE_LEN
            }
        }
    }

    pub(crate) fn generate_implicit_presence_check(&self, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(..) => quote! {},
            TypeSpec::Enum(_) => quote! { if #val_ref.0 != 0 },
            TypeSpec::Float | TypeSpec::Double => quote! { if *#val_ref != 0.0 },
            TypeSpec::Bool => quote! { if *#val_ref },
            TypeSpec::Int(_, _) => quote! { if *#val_ref != 0 },
            TypeSpec::String { .. } => quote! { if !#val_ref.is_empty() },
            TypeSpec::Bytes { .. } => quote! { if !#val_ref.is_empty() },
        }
    }

    /// Generate decode value expressions (Result<T, DecodeError>) for "packable" types
    pub(crate) fn generate_decode_val(
        &self,
        ctx: &Context<'proto>,
        decoder: &Ident,
    ) -> Option<TokenStream> {
        match self {
            TypeSpec::Float => Some(quote! { #decoder.decode_float() }),
            TypeSpec::Double => Some(quote! { #decoder.decode_double() }),
            TypeSpec::Bool => Some(quote! { #decoder.decode_bool() }),
            TypeSpec::Int(pbint, int_size) => {
                let func = pbint.generate_decode_func(int_size);
                Some(quote! { #decoder.#func() })
            }
            // Enum is actually packable due to https://github.com/protocolbuffers/protobuf/issues/15480
            TypeSpec::Enum(tpath) => {
                let enum_path = ctx.resolve_type_name(tpath);
                Some(quote! { #decoder.decode_int32().map(|n| #enum_path(n as _)) })
            }
            _ => None,
        }
    }

    pub(crate) fn generate_decode_mut(
        &self,
        ctx: &Context<'proto>,
        implicit_presence: bool,
        decoder: &Ident,
        mut_ref: &Ident,
    ) -> Result<TokenStream, String> {
        let presence = if implicit_presence {
            "Implicit"
        } else {
            "Explicit"
        };
        let presence_ident = Ident::new(presence, Span::call_site());

        let tok = match self {
            TypeSpec::Message(..) => quote! { #mut_ref.decode_len_delimited(#decoder)?; },
            TypeSpec::Enum(_)
            | TypeSpec::Float
            | TypeSpec::Double
            | TypeSpec::Bool
            | TypeSpec::Int(..) => {
                let val_expr = self
                    .generate_decode_val(ctx, decoder)
                    .expect("ints should be packable");
                let setter = if implicit_presence {
                    let val_ref = Ident::new("val_ref", Span::call_site());
                    let presence_check = self.generate_implicit_presence_check(&val_ref);
                    quote! {
                        let #val_ref = &val;
                        #presence_check {
                            *#mut_ref = val as _;
                        }
                    }
                } else {
                    quote! { *#mut_ref = val as _; }
                };
                quote! {
                    let val = #val_expr?;
                    #setter
                }
            }
            TypeSpec::String { .. } => {
                quote! { #decoder.decode_string(#mut_ref, ::micropb::Presence::#presence_ident)?; }
            }
            TypeSpec::Bytes { .. } => {
                quote! { #decoder.decode_bytes(#mut_ref, ::micropb::Presence::#presence_ident)?; }
            }
        };
        Ok(tok)
    }

    pub(crate) fn is_cached(&self, ctx: &Context<'proto>) -> bool {
        if let TypeSpec::Message(tname) = self {
            ctx.params.cache_extern_types || !ctx.params.extern_paths.contains_key(*tname)
        } else {
            false
        }
    }

    pub(crate) fn generate_cache_type(&self, ctx: &Context<'proto>) -> Option<TokenStream> {
        if let TypeSpec::Message(tname) = self
            && (ctx.params.cache_extern_types || !ctx.params.extern_paths.contains_key(*tname))
        {
            let cache_name = (*tname).to_owned() + "._Cache";
            let cache_type = ctx.resolve_type_name(&cache_name);
            Some(cache_type)
        } else {
            None
        }
    }

    pub(crate) fn generate_sizeof(&self, _ctx: &Context<'proto>, val_ref: &Ident) -> TokenStream {
        match self {
            TypeSpec::Message(..) => {
                quote! { ::micropb::size::sizeof_len_record(#val_ref.compute_size()) }
            }
            TypeSpec::Enum(_) => quote! { ::micropb::size::sizeof_int32(#val_ref.0 as _) },
            TypeSpec::Float => quote! { 4 },
            TypeSpec::Double => quote! { 8 },
            TypeSpec::Bool => quote! { 1 },
            TypeSpec::Int(pbint, int_size) => pbint.generate_sizeof(int_size, val_ref),
            TypeSpec::String { .. } => {
                quote! { ::micropb::size::sizeof_len_record(#val_ref.len()) }
            }
            TypeSpec::Bytes { .. } => quote! { ::micropb::size::sizeof_len_record(#val_ref.len()) },
        }
    }

    pub(crate) fn generate_encode_expr(
        &self,
        _ctx: &Context<'proto>,
        encoder: &Ident,
        val_ref: &Ident,
    ) -> TokenStream {
        match self {
            TypeSpec::Message(..) => quote! { #val_ref.encode_len_delimited(#encoder) },
            TypeSpec::Enum(_) => quote! { #encoder.encode_int32(#val_ref.0 as _) },
            TypeSpec::Float => quote! { #encoder.encode_float(* #val_ref) },
            TypeSpec::Double => quote! { #encoder.encode_double(* #val_ref) },
            TypeSpec::Bool => quote! { #encoder.encode_bool(* #val_ref) },
            TypeSpec::Int(pbint, int_size) => {
                let func = pbint.generate_encode_func(int_size);
                quote! { #encoder.#func(* #val_ref as _) }
            }
            TypeSpec::String { .. } => quote! { #encoder.encode_string(#val_ref) },
            TypeSpec::Bytes { .. } => quote! { #encoder.encode_bytes(#val_ref) },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{config::Config, generator::make_ctx};

    use super::*;

    #[test]
    fn max_size() {
        assert_eq!(TypeSpec::Float.max_size(), Ok(4));
        assert_eq!(TypeSpec::Double.max_size(), Ok(8));
        assert_eq!(TypeSpec::Bool.max_size(), Ok(1));
        assert_eq!(TypeSpec::Int(PbInt::Int32, IntSize::S8).max_size(), Ok(10));
        assert_eq!(TypeSpec::Int(PbInt::Int64, IntSize::S8).max_size(), Ok(10));
        assert_eq!(TypeSpec::Int(PbInt::Fixed32, IntSize::S8).max_size(), Ok(4));
        assert_eq!(TypeSpec::Int(PbInt::Fixed64, IntSize::S8).max_size(), Ok(8));

        // uint types
        assert_eq!(TypeSpec::Int(PbInt::Uint32, IntSize::S8).max_size(), Ok(2));
        assert_eq!(TypeSpec::Int(PbInt::Uint32, IntSize::S32).max_size(), Ok(5));
        assert_eq!(TypeSpec::Int(PbInt::Uint32, IntSize::S64).max_size(), Ok(5));
        assert_eq!(TypeSpec::Int(PbInt::Uint64, IntSize::S16).max_size(), Ok(3));
        assert_eq!(TypeSpec::Int(PbInt::Uint64, IntSize::S32).max_size(), Ok(5));
        assert_eq!(
            TypeSpec::Int(PbInt::Uint64, IntSize::S64).max_size(),
            Ok(10)
        );

        // sint types
        assert_eq!(TypeSpec::Int(PbInt::Sint32, IntSize::S16).max_size(), Ok(3));
        assert_eq!(TypeSpec::Int(PbInt::Sint32, IntSize::S32).max_size(), Ok(5));
        assert_eq!(TypeSpec::Int(PbInt::Sint32, IntSize::S64).max_size(), Ok(5));
        assert_eq!(TypeSpec::Int(PbInt::Sint64, IntSize::S16).max_size(), Ok(3));
        assert_eq!(TypeSpec::Int(PbInt::Sint64, IntSize::S32).max_size(), Ok(5));
        assert_eq!(
            TypeSpec::Int(PbInt::Sint64, IntSize::S64).max_size(),
            Ok(10)
        );

        assert_eq!(
            TypeSpec::String {
                typestr: "test".to_owned(),
                max_bytes: Some(12)
            }
            .max_size(),
            Ok(13)
        );
        assert_eq!(
            TypeSpec::String {
                typestr: "test".to_owned(),
                max_bytes: None
            }
            .max_size(),
            Err("unbounded string or bytes")
        );

        assert_eq!(
            TypeSpec::Bytes {
                typestr: "test".to_owned(),
                max_bytes: Some(12)
            }
            .max_size(),
            Ok(13)
        );
        assert_eq!(
            TypeSpec::Bytes {
                typestr: "test".to_owned(),
                max_bytes: None
            }
            .max_size(),
            Err("unbounded string or bytes")
        );
    }

    fn field_proto(typ: Type, type_name: &str) -> FieldDescriptorProto {
        let mut f = FieldDescriptorProto::default();
        f.set_name("name".to_owned());
        f.set_number(0);
        f.set_type(typ);
        f.set_type_name(type_name.to_owned());
        f
    }

    #[test]
    fn from_proto() {
        let mut config = Box::new(
            Config::new()
                .string_type("string::String<$N>")
                .bytes_type("vec::Vec<u8, $N>")
                .max_bytes(10),
        );

        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Float, ""), &type_conf).unwrap(),
            TypeSpec::Float
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Double, ""), &type_conf).unwrap(),
            TypeSpec::Double
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bool, ""), &type_conf).unwrap(),
            TypeSpec::Bool
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::String, ""), &type_conf).unwrap(),
            TypeSpec::String {
                typestr: "string::String<$N>".to_owned(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bytes, ""), &type_conf).unwrap(),
            TypeSpec::Bytes {
                typestr: "vec::Vec<u8, $N>".to_owned(),
                max_bytes: Some(10)
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Message, ".msg.Message"), &type_conf).unwrap(),
            TypeSpec::Message(".msg.Message")
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Enum, ".Enum"), &type_conf).unwrap(),
            TypeSpec::Enum(".Enum")
        );

        config.string_type = Some("string::String".to_owned());
        config.bytes_type = Some("Bytes".to_owned());
        config.max_bytes = None;
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::String, ""), &type_conf).unwrap(),
            TypeSpec::String {
                typestr: "string::String".to_owned(),
                max_bytes: None
            }
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Bytes, ""), &type_conf).unwrap(),
            TypeSpec::Bytes {
                typestr: "Bytes".to_owned(),
                max_bytes: None
            }
        );
    }

    #[test]
    fn from_proto_num() {
        let mut config = Box::new(Config::new());
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Sint32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Sint32, IntSize::S32)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Int64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Int64, IntSize::S64)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Fixed32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Fixed32, IntSize::S32)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Uint64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Uint64, IntSize::S64)
        );

        config.int_size = Some(IntSize::S8);
        let type_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Sint32, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Sint32, IntSize::S8)
        );
        assert_eq!(
            TypeSpec::from_proto(&field_proto(Type::Uint64, ""), &type_conf).unwrap(),
            TypeSpec::Int(PbInt::Uint64, IntSize::S8)
        );
    }

    #[test]
    fn tspec_default() {
        let ctx = make_ctx();
        assert_eq!(
            TypeSpec::Bool
                .generate_default("true", &ctx)
                .unwrap()
                .to_string(),
            quote! { true as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Bool
                .generate_default("false", &ctx)
                .unwrap()
                .to_string(),
            quote! { false as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Float
                .generate_default("0.1", &ctx)
                .unwrap()
                .to_string(),
            quote! { 0.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Double
                .generate_default("-4.1", &ctx)
                .unwrap()
                .to_string(),
            quote! { -4.1 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::Int(PbInt::Int32, IntSize::S8)
                .generate_default("-99", &ctx)
                .unwrap()
                .to_string(),
            quote! { -99 as _ }.to_string()
        );
        assert_eq!(
            TypeSpec::String {
                typestr: "Vec".to_owned(),
                max_bytes: None
            }
            .generate_default("abc\n\tddd", &ctx)
            .unwrap()
            .to_string(),
            quote! { ::core::convert::TryFrom::try_from("abc\n\tddd").unwrap_or_default() }
                .to_string()
        );
        assert_eq!(
            TypeSpec::Bytes {
                typestr: "Vec".to_owned(),
                max_bytes: None
            }
            .generate_default("abc\\n\\t\\a\\xA0ddd", &ctx)
            .unwrap()
            .to_string(),
            quote! { ::core::convert::TryFrom::try_from(b"abc\n\t\x07\xA0ddd".as_slice()).unwrap_or_default() }
                .to_string()
        );
    }
}
