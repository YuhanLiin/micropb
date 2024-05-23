use proc_macro2::Span;
use syn::Ident;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum IntSize {
    S8,
    S16,
    S32,
    S64,
}

impl IntSize {
    pub(crate) fn type_name(self, signed: bool) -> Ident {
        let t = match self {
            IntSize::S8 if signed => "i8",
            IntSize::S8 => "u8",
            IntSize::S16 if signed => "i16",
            IntSize::S16 => "u16",
            IntSize::S32 if signed => "i32",
            IntSize::S32 => "u32",
            IntSize::S64 if signed => "i64",
            IntSize::S64 => "u64",
        };
        Ident::new(t, Span::call_site())
    }
}

#[derive(Debug, Clone)]
pub enum CustomField {
    Type(String),
    Delegate(String),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum OptionalRepr {
    Hazzer,
    Option,
}

macro_rules! config_decl {
    ($($(#[$attr:meta])* $([$placeholder:ident])? $field:ident : $([$placeholder2:ident])? Option<$type:ty>,)+) => {
        #[non_exhaustive]
        #[derive(Debug, Clone, Default)]
        pub struct Config {
            $($(#[$attr])* pub $field: Option<$type>,)+
        }

        impl Config {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn merge(&mut self, other: &Self) {
                $(config_decl!(@merge $([$placeholder])? $field, self, other);)+
            }

            $(config_decl!(@setter $field: $([$placeholder2])? $type);)+
        }
    };

    (@merge $field:ident, $self:ident, $other:ident) => {
        if let Some(v) = &$other.$field {
            $self.$field = Some(v.clone());
        }
    };

    (@merge [no_inherit] $field:ident, $self:ident, $other:ident) => {
        $self.$field = $other.$field.clone();
    };

    (@setter $field:ident: [deref] $type:ty) => {
        pub fn $field(mut self, s: &str) -> Self {
            self.$field = Some(s.to_owned());
            self
        }
    };

    (@setter $field:ident: $type:ty) => {
        pub fn $field(mut self, val: $type) -> Self {
            self.$field = Some(val);
            self
        }
    };
}

config_decl! {
    // Field configs
    max_len: Option<u32>,
    max_bytes: Option<u32>,
    int_size: Option<IntSize>,
    field_attributes: [deref] Option<String>,
    boxed: Option<bool>,
    vec_type: [deref] Option<String>,
    string_type: [deref] Option<String>,
    map_type: [deref] Option<String>,
    optional_repr: Option<OptionalRepr>,
    unknown_handler: [deref] Option<String>,
    [no_inherit] custom_field: Option<CustomField>,
    [no_inherit] rename_field: [deref] Option<String>,

    // Type configs
    enum_int_size: Option<IntSize>,
    type_attributes: [deref] Option<String>,
    no_debug_derive: Option<bool>,

    // General configs
    skip: Option<bool>,
}

struct Attributes(Vec<syn::Attribute>);

impl syn::parse::Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(syn::Attribute::parse_outer)?))
    }
}

pub(crate) fn parse_attributes(s: &str) -> syn::Result<Vec<syn::Attribute>> {
    let attrs: Attributes = syn::parse_str(s)?;
    Ok(attrs.0)
}

impl Config {
    pub(crate) fn field_attr_parsed(&self) -> Result<Vec<syn::Attribute>, String> {
        let s = self.field_attributes.as_deref().unwrap_or("");
        parse_attributes(s).map_err(|e| {
            format!("Failed to parse field_attributes \"{s}\" as Rust attributes: {e}")
        })
    }

    pub(crate) fn type_attr_parsed(&self) -> Result<Vec<syn::Attribute>, String> {
        let s = self.type_attributes.as_deref().unwrap_or("");
        parse_attributes(s)
            .map_err(|e| format!("Failed to parse type_attributes \"{s}\" as Rust attributes: {e}"))
    }

    pub(crate) fn rust_field_name(&self, name: &str) -> Result<Ident, String> {
        let s = self.rename_field.as_deref().unwrap_or(name);
        syn::parse_str(s)
            .map_err(|e| format!("Failed to parse rename_field \"{s}\" as identifier: {e}"))
    }

    pub(crate) fn vec_type_parsed(&self) -> Result<Option<syn::Path>, String> {
        self.vec_type
            .as_ref()
            .map(|t| {
                syn::parse_str(t)
                    .map_err(|e| format!("Failed to parse vec_type \"{t}\" as type path: {e}"))
            })
            .transpose()
    }

    pub(crate) fn string_type_parsed(&self) -> Result<Option<syn::Path>, String> {
        self.string_type
            .as_ref()
            .map(|t| {
                syn::parse_str(t)
                    .map_err(|e| format!("Failed to parse string_type \"{t}\" as type path: {e}"))
            })
            .transpose()
    }

    pub(crate) fn map_type_parsed(&self) -> Result<Option<syn::Path>, String> {
        self.map_type
            .as_ref()
            .map(|t| {
                syn::parse_str(t)
                    .map_err(|e| format!("Failed to parse map_type \"{t}\" as type path: {e}"))
            })
            .transpose()
    }

    pub(crate) fn unknown_handler_parsed(&self) -> Result<Option<syn::Type>, String> {
        self.unknown_handler
            .as_ref()
            .map(|t| {
                syn::parse_str(t).map_err(|e| {
                    format!("Failed to parse unknown_handler \"{t}\" as Rust type: {e}")
                })
            })
            .transpose()
    }

    pub(crate) fn custom_field_parsed(
        &self,
    ) -> Result<Option<crate::generator::field::CustomField>, String> {
        let res = match &self.custom_field {
            Some(CustomField::Type(s)) => Some(crate::generator::field::CustomField::Type(
                syn::parse_str(s).map_err(|e| {
                    format!("Failed to parse custom field \"{s}\" as Rust type: {e}")
                })?,
            )),
            Some(CustomField::Delegate(s)) => Some(crate::generator::field::CustomField::Delegate(
                syn::parse_str(s).map_err(|e| {
                    format!("Failed to parse custom delegate \"{s}\" as identifier: {e}")
                })?,
            )),
            None => None,
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use quote::{format_ident, quote, ToTokens};

    use super::*;

    #[test]
    fn merge() {
        let mut mergee = Config::new()
            .rename_field("rename")
            .skip(true)
            .vec_type("vec")
            .string_type("str");
        let merger = Config::new().skip(false).vec_type("array");
        mergee.merge(&merger);

        assert!(!mergee.skip.unwrap());
        assert_eq!(mergee.vec_type.unwrap(), "array");
        assert_eq!(mergee.string_type.unwrap(), "str");
        // max_len was never set
        assert!(mergee.max_len.is_none());
        // rename_field gets overwritten unconditionally when merging
        assert!(mergee.rename_field.is_none());
    }

    #[test]
    fn parse() {
        let mut config = Config::new()
            .vec_type("heapless::Vec")
            .string_type("heapless::String")
            .map_type("Map")
            .type_attributes("#[derive(Hash)]");

        assert_eq!(
            config
                .vec_type_parsed()
                .unwrap()
                .to_token_stream()
                .to_string(),
            quote! { heapless::Vec }.to_string()
        );
        assert_eq!(
            config
                .string_type_parsed()
                .unwrap()
                .to_token_stream()
                .to_string(),
            quote! { heapless::String }.to_string()
        );
        assert_eq!(
            config
                .map_type_parsed()
                .unwrap()
                .to_token_stream()
                .to_string(),
            "Map"
        );
        let attrs = config.type_attr_parsed().unwrap();
        assert_eq!(
            quote! { #(#attrs)* }.to_string(),
            quote! { #[derive(Hash)] }.to_string()
        );

        let attrs = config.field_attr_parsed().unwrap();
        assert_eq!(quote! { #(#attrs)* }.to_string(), "");
        config.field_attributes = Some("#[default] #[delete]".to_owned());
        let attrs = config.field_attr_parsed().unwrap();
        assert_eq!(
            quote! { #(#attrs)* }.to_string(),
            quote! { #[default] #[delete] }.to_string()
        );

        assert_eq!(
            config.rust_field_name("name").unwrap(),
            format_ident!("name")
        );
        config.rename_field = Some("rename".to_string());
        assert_eq!(
            config.rust_field_name("name").unwrap(),
            format_ident!("rename")
        );

        config.custom_field = Some(CustomField::Type("Vec<u16, 4>".to_owned()));
        let crate::generator::field::CustomField::Type(typ) =
            config.custom_field_parsed().unwrap().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(
            typ.to_token_stream().to_string(),
            quote! { Vec<u16, 4> }.to_string()
        );

        config.custom_field = Some(CustomField::Delegate("name".to_owned()));
        let crate::generator::field::CustomField::Delegate(del) =
            config.custom_field_parsed().unwrap().unwrap()
        else {
            unreachable!()
        };
        assert_eq!(del, format_ident!("name"));
    }
}
