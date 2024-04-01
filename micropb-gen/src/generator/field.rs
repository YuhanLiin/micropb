use proc_macro2::{Literal, Span, TokenStream};
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, FieldDescriptorProto, Syntax,
};
use quote::{format_ident, quote};
use syn::Ident;

use crate::config::OptionalRepr;

use super::{type_spec::TypeSpec, CurrentConfig, Generator};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum CustomField {
    Type(syn::Type),
    Delegate(Ident),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum FieldType {
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
    Optional(TypeSpec, OptionalRepr),
    Repeated {
        typ: TypeSpec,
        packed: bool,
        type_path: syn::Path,
        max_len: Option<u32>,
    },
    Custom(CustomField),
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Field<'a> {
    pub(crate) num: u32,
    pub(crate) ftype: FieldType,
    pub(crate) name: &'a str,
    pub(crate) rust_name: Ident,
    pub(crate) default: Option<&'a str>,
    pub(crate) boxed: bool,
    pub(crate) attrs: Vec<syn::Attribute>,
}

impl<'a> Field<'a> {
    pub(crate) fn explicit_presence(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(..))
    }

    pub(crate) fn is_option(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Option))
    }

    pub(crate) fn is_hazzer(&self) -> bool {
        matches!(self.ftype, FieldType::Optional(_, OptionalRepr::Hazzer))
    }

    pub(crate) fn delegate(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Delegate(d)) = &self.ftype {
            Some(d)
        } else {
            None
        }
    }

    pub(crate) fn custom_type_field(&self) -> Option<&Ident> {
        if let FieldType::Custom(CustomField::Type(_)) = &self.ftype {
            Some(&self.rust_name)
        } else {
            None
        }
    }

    pub(crate) fn from_proto(
        proto: &'a FieldDescriptorProto,
        field_conf: &CurrentConfig,
        syntax: Syntax,
        map_msg: Option<&DescriptorProto>,
    ) -> Option<Self> {
        if field_conf.config.skip.unwrap_or(false) {
            return None;
        }

        let num = proto.number.unwrap() as u32;
        let name = proto.name();
        let rust_name = field_conf.config.rust_field_name(name);
        let boxed = field_conf.config.boxed.unwrap_or(false);

        let ftype = match (
            field_conf.config.custom_field_parsed(),
            map_msg,
            proto.label(),
        ) {
            (Some(t), _, _) => FieldType::Custom(t),

            (None, Some(map_msg), _) => {
                let key = TypeSpec::from_proto(&map_msg.field[0], &field_conf.next_conf("key"));
                let val = TypeSpec::from_proto(&map_msg.field[1], &field_conf.next_conf("value"));
                let type_name = field_conf.config.map_type_parsed().unwrap();
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

            (None, None, Label::Repeated) => FieldType::Repeated {
                typ: TypeSpec::from_proto(proto, &field_conf.next_conf("elem")),
                type_path: field_conf.config.vec_type_parsed().unwrap(),
                max_len: field_conf.config.max_len,
                packed: proto
                    .options
                    .as_ref()
                    .and_then(|opt| opt.packed)
                    .unwrap_or(false),
            },

            (None, None, Label::Required | Label::Optional)
                if syntax == Syntax::Proto2
                    || proto.proto3_optional()
                    || proto.r#type() == Type::Message =>
            {
                let repr = field_conf.config.optional_repr.unwrap_or(if boxed {
                    OptionalRepr::Option
                } else {
                    OptionalRepr::Hazzer
                });
                FieldType::Optional(TypeSpec::from_proto(proto, field_conf), repr)
            }

            (None, None, _) => FieldType::Single(TypeSpec::from_proto(proto, field_conf)),
        };
        let attrs = field_conf.config.field_attr_parsed();

        Some(Field {
            num,
            ftype,
            name,
            rust_name,
            default: proto.default_value.as_deref(),
            boxed,
            attrs,
        })
    }

    pub(crate) fn generate_rust_type(&self, gen: &Generator) -> TokenStream {
        let typ = match &self.ftype {
            FieldType::Map {
                key,
                val,
                type_path: type_name,
                max_len,
                ..
            } => {
                let k = key.generate_rust_type(gen);
                let v = val.generate_rust_type(gen);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_name <#k, #v #(, #max_len)* > }
            }

            FieldType::Single(t) | FieldType::Optional(t, _) => t.generate_rust_type(gen),

            FieldType::Repeated {
                typ,
                type_path,
                max_len,
                ..
            } => {
                let t = typ.generate_rust_type(gen);
                let max_len = max_len.map(Literal::u32_unsuffixed).into_iter();
                quote! { #type_path <#t #(, #max_len)* > }
            }

            FieldType::Custom(CustomField::Type(t)) => return quote! {#t},
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have a type")
            }
        };
        gen.wrapped_type(typ, self.boxed, self.is_option())
    }

    pub(crate) fn generate_field(&self, gen: &Generator) -> TokenStream {
        if let FieldType::Custom(CustomField::Delegate(_)) = self.ftype {
            return quote! {};
        }
        let typ = self.generate_rust_type(gen);
        let name = &self.rust_name;
        let attrs = &self.attrs;
        quote! { #(#attrs)* pub #name : #typ, }
    }

    pub(crate) fn generate_default(&self, gen: &Generator) -> TokenStream {
        match self.ftype {
            FieldType::Single(ref t) | FieldType::Optional(ref t, OptionalRepr::Hazzer) => {
                if let Some(default) = self.default {
                    let value = t.generate_default(default, gen);
                    return gen.wrapped_value(value, self.boxed, false);
                }
            }
            // Options don't use custom defaults, they should just default to None
            FieldType::Optional(_, OptionalRepr::Option) => {
                return quote! { ::core::option::Option::None }
            }
            FieldType::Custom(CustomField::Delegate(_)) => {
                unreachable!("delegate field cannot have default")
            }
            _ => {}
        }
        quote! { ::core::default::Default::default() }
    }

    pub(crate) fn generate_decode_branch(
        &self,
        gen: &Generator,
        tag: &Ident,
        decoder: &Ident,
    ) -> TokenStream {
        let fnum = self.num;
        let fname = &self.name;
        let mut_ref = Ident::new("mut_ref", Span::call_site());

        let decode_code = match &self.ftype {
            FieldType::Map { key, val, .. } => {
                let key_decode_expr = key.generate_decode_mut(gen, decoder, &mut_ref);
                let val_decode_expr = val.generate_decode_mut(gen, decoder, &mut_ref);
                quote! {
                    #decoder.decode_map_elem(
                        |#mut_ref, #decoder| { #key_decode_expr },
                        |#mut_ref, #decoder| { #val_decode_expr },
                    )?;
                }
            }

            FieldType::Single(tspec) | FieldType::Optional(tspec, OptionalRepr::Hazzer) => {
                let decode_expr = tspec.generate_decode_mut(gen, decoder, &mut_ref);
                let set_has = self.is_hazzer().then(|| {
                    let setter = format_ident!("set_{fname}");
                    quote! { self._has.#setter(true); }
                });
                quote! {
                    let #mut_ref = &mut self.#fname;
                    #decode_expr;
                    #set_has
                }
            }

            FieldType::Optional(tspec, OptionalRepr::Option) => {
                let decode_expr = tspec.generate_decode_mut(gen, decoder, &mut_ref);
                quote! {
                    let #mut_ref = &mut *self.#fname.get_or_insert_default();
                    #decode_expr;
                }
            }

            FieldType::Repeated { typ, .. } => {
                if let Some(val) = typ.generate_decode_val(decoder) {
                    // Type can be packed and is Copy, so we check the wire type to see if we can
                    // do packed decoding
                    quote! {
                        if #tag.wire_type() == WIRE_TYPE_LEN {
                            #decoder.decode_packed(&mut self.#fname, |#decoder| #val)?;
                        } else {
                            self.#fname.pb_push(#val).map_err(|_| ::micropb::DecodeError::Capacity)?;
                        }
                    }
                } else {
                    let decode_expr = typ.generate_decode_mut(gen, decoder, &mut_ref);
                    quote! {
                        self.#fname.pb_push(::core::default::Default::default()).map_err(|_| ::micropb::DecodeError::Capacity)?;
                        let #mut_ref = self.#fname.last_mut().unwrap();
                        #decode_expr;
                    }
                }
            }

            FieldType::Custom(CustomField::Type(_)) => {
                quote! { self.#fname.decode_field(#tag, #decoder)?; }
            }

            FieldType::Custom(CustomField::Delegate(field)) => {
                quote! { self.#field.decode_field(#tag, #decoder)?; }
            }
        };

        quote! {
            #fnum => { #decode_code }
        }
    }
}

#[cfg(test)]
pub(crate) fn make_test_field(num: u32, name: &str, boxed: bool, ftype: FieldType) -> Field {
    Field {
        num,
        ftype,
        name,
        rust_name: Ident::new(name, proc_macro2::Span::call_site()),
        default: None,
        boxed,
        attrs: vec![],
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use proc_macro2::Span;

    use crate::{
        config::{parse_attributes, Config, IntType},
        generator::type_spec::PbInt,
        pathtree::Node,
    };

    use super::*;

    fn field_proto(
        num: u32,
        name: &str,
        label: Option<Label>,
        proto3_opt: bool,
    ) -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: Some(name.to_owned()),
            number: Some(num as i32),
            label: label.map(|l| l.into()),
            r#type: Some(Type::Bool.into()),
            proto3_optional: Some(proto3_opt),
            ..Default::default()
        }
    }

    #[test]
    fn from_proto_skipped() {
        let config = Box::new(Config::new().skip(true));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(2, "field", None, false);
        assert!(Field::from_proto(&field, &field_conf, Syntax::Proto2, None).is_none());
    }

    #[test]
    fn from_proto_field() {
        let config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(2, "field", None, false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None).unwrap(),
            Field {
                num: 2,
                ftype: FieldType::Single(TypeSpec::Bool),
                name: "field",
                rust_name: Ident::new("field", Span::call_site()),
                default: None,
                boxed: false,
                attrs: vec![],
            }
        );

        // With some field configs
        let config = Box::new(
            Config::new()
                .boxed(true)
                .rename_field("renamed")
                .field_attributes("#[attr]"),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let mut field = field_proto(2, "field", None, false);
        field.default_value = Some("true".to_owned());
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None).unwrap(),
            Field {
                num: 2,
                ftype: FieldType::Single(TypeSpec::Bool),
                name: "field",
                rust_name: Ident::new("renamed", Span::call_site()),
                default: Some("true"),
                boxed: true,
                attrs: parse_attributes("#[attr]").unwrap(),
            }
        );
    }

    #[test]
    fn from_proto_field_type() {
        let config = Box::new(Config::new());
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(0, "field", None, false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .ftype,
            FieldType::Single(TypeSpec::Bool)
        );
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // Required fields are treated like optionals
        let field = field_proto(0, "field", Some(Label::Required), false);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // In proto3, if proto3_optional is set then field is optional
        let field = field_proto(0, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
        );

        // Boxed optionals should default to using Option instead of hazzers
        let config = Box::new(Config::new().boxed(true));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option)
        );

        // Explicitly set the optional_repr to Option, overriding the default
        let config = Box::new(Config::new().optional_repr(OptionalRepr::Option));
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option)
        );
    }

    #[test]
    fn from_proto_custom() {
        // Even if the field is boxed or optional, as long as we specify a custom field, those
        // other options are all ignored
        let config = Box::new(
            Config::new()
                .boxed(true)
                .custom_field(crate::config::CustomField::Type("Custom<false>".to_owned())),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Custom(CustomField::Type(syn::parse_str("Custom<false>").unwrap()))
        );

        let config = Box::new(
            Config::new()
                .boxed(true)
                .custom_field(crate::config::CustomField::Delegate("field".to_owned())),
        );
        let field_conf = CurrentConfig {
            node: None,
            config: Cow::Borrowed(&config),
        };
        let field = field_proto(1, "field", Some(Label::Optional), true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, None)
                .unwrap()
                .ftype,
            FieldType::Custom(CustomField::Delegate(syn::parse_str("field").unwrap()))
        );
    }

    #[test]
    fn from_proto_repeated() {
        // Repeated fields with custom element int type
        let config = Box::new(Config::new().max_len(21).vec_type("Vec"));
        let mut node = Node::default();
        *node.add_path(std::iter::once("elem")).value_mut() =
            Some(Box::new(Config::new().int_type(IntType::U8)));
        let field_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        let mut field = field_proto(0, "field", Some(Label::Repeated), false);
        field.r#type = Some(Type::Int32.into());
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntType::U8),
                packed: false,
                type_path: syn::parse_str("Vec").unwrap(),
                max_len: Some(21)
            }
        );
        field.options = Some(Default::default());
        field.options.as_mut().unwrap().packed = Some(true);
        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto3, None)
                .unwrap()
                .ftype,
            FieldType::Repeated {
                typ: TypeSpec::Int(PbInt::Int32, IntType::U8),
                packed: true,
                type_path: syn::parse_str("Vec").unwrap(),
                max_len: Some(21)
            }
        );
    }

    #[test]
    fn from_proto_map() {
        let config = Box::new(Config::new().map_type("std::Map"));
        let mut node = Node::default();
        *node.add_path(std::iter::once("key")).value_mut() =
            Some(Box::new(Config::new().int_type(IntType::U8)));
        *node.add_path(std::iter::once("value")).value_mut() =
            Some(Box::new(Config::new().string_type("std::String")));
        let field_conf = CurrentConfig {
            node: Some(&node),
            config: Cow::Borrowed(&config),
        };

        let mut key = field_proto(1, "key", Some(Label::Optional), false);
        key.r#type = Some(Type::Int32.into());
        let mut value = field_proto(1, "value", Some(Label::Optional), false);
        value.r#type = Some(Type::String.into());
        let mut map_elem = DescriptorProto {
            name: Some("MapElem".to_owned()),
            field: vec![key, value],
            extension: vec![],
            nested_type: vec![],
            enum_type: vec![],
            extension_range: vec![],
            oneof_decl: vec![],
            options: Some(Default::default()),
            reserved_range: vec![],
            reserved_name: vec![],
        };
        map_elem.options.as_mut().unwrap().map_entry = Some(true);
        let mut field = field_proto(0, "field", Some(Label::Repeated), false);
        field.r#type = Some(Type::Message.into());
        field.type_name = Some("MapElem".to_owned());

        assert_eq!(
            Field::from_proto(&field, &field_conf, Syntax::Proto2, Some(&map_elem))
                .unwrap()
                .ftype,
            FieldType::Map {
                key: TypeSpec::Int(PbInt::Int32, IntType::U8),
                val: TypeSpec::String {
                    type_path: syn::parse_str("std::String").unwrap(),
                    max_bytes: None
                },
                packed: false,
                type_path: syn::parse_str("std::Map").unwrap(),
                max_len: None
            }
        );
    }

    #[test]
    fn field_rust_type() {
        let gen = Generator::default();
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
            )
            .generate_rust_type(&gen)
            .to_string(),
            "bool"
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Repeated {
                    typ: TypeSpec::Message(".Message".to_owned()),
                    type_path: syn::parse_str("Vec").unwrap(),
                    max_len: None,
                    packed: true
                }
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { Vec<Message> }.to_string()
        );
        assert_eq!(
            make_test_field(
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
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { Vec<String<4>, 10> }.to_string()
        );
        assert_eq!(
            make_test_field(
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
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { std::HashMap<f32, u32> }.to_string()
        );
        assert_eq!(
            make_test_field(
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
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { std::HashMap<u32, f32, 8> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Custom(CustomField::Type(
                    syn::parse_str("custom::Type<true>").unwrap()
                ))
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { custom::Type<true> }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Optional(
                    TypeSpec::Message(".Config".to_owned()),
                    OptionalRepr::Option
                )
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { ::core::option::Option<::alloc::boxed::Box<Config> > }.to_string()
        );
        assert_eq!(
            make_test_field(
                0,
                "field",
                true,
                FieldType::Single(TypeSpec::Message(".Config".to_owned()))
            )
            .generate_rust_type(&gen)
            .to_string(),
            quote! { ::alloc::boxed::Box<Config> }.to_string()
        );
    }

    #[test]
    fn field_default() {
        let gen = Generator::default();
        // no special default
        assert_eq!(
            make_test_field(
                0,
                "field",
                false,
                FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer)
            )
            .generate_default(&gen)
            .to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            false,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { false as _ }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Hazzer),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::alloc::boxed::Box::new(false as _) }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Optional(TypeSpec::Bool, OptionalRepr::Option),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::option::Option::None }.to_string()
        );

        let mut field = make_test_field(
            0,
            "field",
            true,
            FieldType::Custom(CustomField::Type(syn::parse_str("Map").unwrap())),
        );
        field.default = Some("false");
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );

        let field = make_test_field(
            0,
            "field",
            true,
            FieldType::Repeated {
                typ: TypeSpec::Double,
                packed: false,
                type_path: syn::parse_str("Vec").unwrap(),
                max_len: None,
            },
        );
        assert_eq!(
            field.generate_default(&gen).to_string(),
            quote! { ::core::default::Default::default() }.to_string()
        );
    }
}
