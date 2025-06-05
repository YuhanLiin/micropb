//! Configuration options for Protobuf types and fields.

use std::borrow::Cow;

use proc_macro2::{Span, TokenStream};
use syn::Ident;

use crate::generator::sanitized_ident;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
/// Sizes of integer types
pub enum IntSize {
    /// 8-bit int
    S8,
    /// 16-bit int
    S16,
    /// 32-bit int
    S32,
    /// 64-bit int
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

    pub(crate) fn max_value(self) -> u64 {
        match self {
            IntSize::S8 => u8::MAX as u64,
            IntSize::S16 => u16::MAX as u64,
            IntSize::S32 => u32::MAX as u64,
            IntSize::S64 => u64::MAX,
        }
    }

    pub(crate) fn min_value(self) -> i64 {
        match self {
            IntSize::S8 => i8::MIN as i64,
            IntSize::S16 => i16::MIN as i64,
            IntSize::S32 => i32::MIN as i64,
            IntSize::S64 => i64::MIN,
        }
    }
}

#[derive(Debug, Clone)]
/// Customize encoding and decoding behaviour for a generated field
pub enum CustomField {
    /// Fully-qualified type name that replaces the generated type of the field.
    ///
    /// This type must implement `FieldEncode` and `FieldDecode`.
    Type(String),
    /// Name of the other field that this field will delegate to.
    ///
    /// The delegated field must have [`CustomField::Type`] configured. It will handle the decoding
    /// and encoding of this field's wire value.
    Delegate(String),
}

impl CustomField {
    /// Constructs a [`CustomField::Type`]
    pub fn from_type(s: &str) -> Self {
        Self::Type(s.to_owned())
    }

    /// Constructs a [`CustomField::Delegate`]
    pub fn from_delegate(s: &str) -> Self {
        Self::Delegate(s.to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
/// Representation of optional fields in the generated code
pub enum OptionalRepr {
    /// Presence of optional field is tracked in a separate bitfield called a hazzer.
    ///
    /// Default for non-boxed fields.
    Hazzer,
    /// Optional field is wrapped in `Option`
    ///
    /// Default for boxed fields.
    Option,
}

macro_rules! config_decl {
    ($($(#[$doc:meta])* $([$placeholder:ident])? $field:ident : $([$placeholder2:ident])? Option<$type:ty>,)+) => {
        #[non_exhaustive]
        #[derive(Debug, Clone, Default)]
        /// Configuration that changes how the code generator handles Protobuf types and fields.
        /// See [`configure`](crate::Generator::configure) for how configurations are applied.
        ///
        /// Configuration fields are set by chaining builder methods:
        /// ```no_run
        /// # use micropb_gen::Config;
        /// Config::new().boxed(true).max_len(12).vec_type("MyVec");
        /// ```
        pub struct Config {
            $(pub(crate) $field: Option<$type>,)+
        }

        impl Config {
            /// Create new config
            pub fn new() -> Self {
                Self::default()
            }

            pub(crate) fn merge(&mut self, other: &Self) {
                $(config_decl!(@merge $([$placeholder])? $field, self, other);)+
            }

            $(config_decl!(@setter $(#[$doc])* $field: $([$placeholder2])? $type);)+
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

    (@setter $(#[$doc:meta])* $field:ident: [deref] $type:ty) => {
        $(#[$doc])*
        pub fn $field(mut self, s: &str) -> Self {
            self.$field = Some(s.to_owned());
            self
        }
    };

    (@setter $(#[$doc:meta])* $field:ident: $type:ty) => {
        $(#[$doc])*
        pub fn $field(mut self, val: $type) -> Self {
            self.$field = Some(val);
            self
        }
    };
}

config_decl! {
    // Field configs

    /// Max number of elements for fixed-capacity repeated and `map` fields.
    ///
    /// This should only be set if [`vec_type`](Config::vec_type) or [`map_type`](Config::map_type)
    /// is a fix-capacity container, because `max_len` will be used as the 2nd type parameter of
    /// the container in the generated code.
    ///
    /// For example, if `vec_type` is `ArrayVec` and `max_len` is 5, then the generated container
    /// type will be `ArrayVec<_, 5>`.
    max_len: Option<u32>,

    /// Max number of bytes for fixed-capacity `string` and `bytes` fields.
    ///
    /// Like with [`max_len`](Config::max_len), this should only be set if
    /// [`string_type`](Config::string_type) or [`vec_type`](Config::vec_type) is a fix-capacity
    /// container, because `max_bytes` will be used as the 2nd type parameter of the container in
    /// the generated code.
    max_bytes: Option<u32>,

    /// Override the integer type of integer fields such as `int32` or `fixed64`.
    ///
    /// Change the integer fields to be 8, 16, 32, or 64 bytes. If the integer type is smaller than
    /// the value on the wire, the value will be truncated to fit.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::IntSize};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Set type of int32 to `i8`
    /// gen.configure(".Message.int32_field", Config::new().int_size(IntSize::S8));
    /// // Set type of uint32 to `u64`
    /// gen.configure(".Message.uint32_field", Config::new().int_size(IntSize::S64));
    /// ```
    ///
    /// # Avoiding 64-bit operations
    /// Setting a 64-bit int field such as `int64` or `sint64` to >=32 bits makes the code
    /// generator use 32-bit operations on that field instead of 64-bit operations. This can have
    /// performance benefits on some 32-bit platforms. Setting all int fields to >=32 bits allows
    /// `micropb`'s `enable-64bits` feature flag to be turned off, disabling 64-bit operations
    /// altogether.
    int_size: Option<IntSize>,

    /// Set attributes for message fields.
    ///
    /// The attribute string will be placed before matched fields. The string must be in the syntax
    /// of 0 or more Rust attributes.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Set field attribute
    /// gen.configure(".Message.foo", Config::new().field_attributes("#[serde(skip)]"));
    /// // Unset field attribute
    /// gen.configure(".Message.foo", Config::new().field_attributes(""));
    /// ```
    ///
    /// # Special cases
    /// - If applied to an oneof field, the attributes are applied to the oneof field of the
    /// message struct.
    /// - If applied to an oneof variant, the attributes are applied to the oneof enum variant in
    /// the oneof enum definition.
    /// - If applied to the `._has` suffix, the attributes are applied to the hazzer field of the
    /// message struct.
    /// - If applied to the `._unknown` suffix, the attributes are applied to the unknown handler
    /// of the message struct.
    field_attributes: [deref] Option<String>,

    /// Wrap the field in a `Box`.
    ///
    /// If the field is already wrapped in `Option`, then the field will be of type
    /// `Option<Box<_>>`.
    ///
    /// This config not apply to elements of repeated and `map` fields.
    boxed: Option<bool>,

    /// Container type that's generated for repeated fields.
    ///
    /// For decoding, the provided type must implement `PbVec<T>`. For encoding, the type must
    /// dereference into `[T]`, where `T` is the type of the element. Moreover, the type must
    /// implement `Default` in order to generate default values.
    ///
    /// If the provided type contains the sequence `$N`, it will be substituted for the value of
    /// [`max_bytes`](Config::max_bytes) if it's set for this field. Similarly, the sequence `$T`
    /// will be substituted for the type of the repeated element.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::IntSize};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // assuming that .pkg.Message.list is a repeated field of booleans:
    ///
    /// // repeated field configured to `Vec<bool>` (dynamic-capacity)
    /// gen.configure(".pkg.Message.list", Config::new().vec_type("Vec<$T>"));
    /// // repeated field configured to `arrayvec::ArrayVec<bool, 5>` (fixed-capacity)
    /// gen.configure(".pkg.Message.list", Config::new().vec_type("arrayvec::ArrayVec<$T, $N>").max_len(5));
    /// ```
    vec_type: [deref] Option<String>,

    /// Container type that's generated for `string` fields.
    ///
    /// For decoding, the provided type must implement `PbString`. For encoding, the type must
    /// dereference to `str`. Moreover, the type must implement `Default + TryFrom<&str>` in order
    /// to generate default values.
    ///
    /// If the provided type contains the sequence `$N`, it will be substituted for the value of
    /// [`max_bytes`](Config::max_bytes) if it's set for this field.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // `string` field configured to `String` (dynamic-capacity)
    /// gen.configure(".pkg.Message.string_field", Config::new().string_type("String"));
    /// // `string` field configured to `ArrayString<4>` (fixed-capacity)
    /// gen.configure(".pkg.Message.string_field", Config::new().string_type("ArrayString<$N>").max_bytes(4));
    /// ```
    string_type: [deref] Option<String>,

    /// Container type that's generated for `bytes` fields.
    ///
    /// For decoding, the provided type must implement `PbBytes`. For encoding, the type must
    /// dereference to `[u8]`. Moreover, the type must implement `Default + TryFrom<&[u8]>` in
    /// order to generate default values.
    ///
    /// If the provided type contains the sequence `$N`, it will be substituted for the value of
    /// [`max_bytes`](Config::max_bytes) if it's set for this field.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // `bytes` field configured to `Vec<u8>` (dynamic-capacity)
    /// gen.configure(".pkg.Message.string_field", Config::new().string_type("Vec<u8>"));
    /// // `bytes` field configured to `Vec<u8, 4>` (fixed-capacity)
    /// gen.configure(".pkg.Message.string_field", Config::new().string_type("Vec<u8, $N>").max_bytes(4));
    /// ```
    bytes_type: [deref] Option<String>,

    /// Container type that's generated for `map` fields.
    ///
    /// For decoding, the provided type must implement `PbMap`. For encoding, the type must
    /// implement `IntoIterator<Item = (&K, &V)>` for `&T`. Moreover, the type must implement
    /// `Default` in order to generate default values.
    ///
    /// If the provided type contains the sequence `$N`, it will be substituted for the value of
    /// [`max_bytes`](Config::max_bytes) if it's set for this field. Similarly, the sequences `$K`
    /// and `$V` will be substituted for the types of the map key and value respectively.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::IntSize};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // assume that .pkg.Message.map_field is a `map<int32, float>`:
    ///
    /// // `map` field configured to `BTreeMap<i32, f32>` (dynamic-capacity)
    /// gen.configure(".pkg.Message.map_field", Config::new().map_type("BTreeMap<$K, $V>"));
    /// // `map` field configured to `FnvIndexMap<i32, f32, 4>` (fixed-capacity)
    /// gen.configure(".pkg.Message.map_field", Config::new().map_type("FnvIndexMap<$K, $V, $N>").max_len(4));
    /// ```
    map_type: [deref] Option<String>,

    /// Determine how optional fields are represented.
    ///
    /// Presence of optional fields is tracked by either a bitfield in the message struct called a
    /// hazzer, or by the `Option` type. By default, non-boxed fields use hazzers and boxed fields
    /// use `Option`. This behaviour can be customized by setting this option.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::OptionalRepr};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // `optional1: T` with bitfield entry (default unboxed behaviour)
    /// gen.configure(".Message.optional1", Config::new().optional_repr(OptionalRepr::Hazzer));
    /// // `optional2: Option<T>`
    /// gen.configure(".Message.optional2", Config::new().optional_repr(OptionalRepr::Option));
    /// // `optional3: Box<T>` with bitfield entry
    /// gen.configure(".Message.optional3", Config::new().boxed(true)
    ///                                         .optional_repr(OptionalRepr::Hazzer));
    /// // `optional4: Option<Box<T>>` (default boxed behaviour)
    /// gen.configure(".Message.optional4", Config::new().boxed(true)
    ///                                         .optional_repr(OptionalRepr::Option));
    /// ```
    optional_repr: Option<OptionalRepr>,

    /// Replace generated field with an user-provided type. See
    /// [`CustomField`](crate::config::CustomField) for more info.
    ///
    /// Substitute a user-provided type as the type of the field. The encoding and decoding
    /// behaviour will also be user-provided, so the custom type must implement `FieldEncode` and
    /// `FieldDecode` and correctly handle the field's wire representation.
    ///
    /// Alternatively, a field can be set to "delegate" to another custom field for encoding and
    /// decoding. In that case, the field won't be generated at all, and its wire value will be
    /// handled by the delegated field.
    ///
    /// This configuration applies to normal field and oneof fields, but won't be applied to
    /// `oneof` variants.
    ///
    /// # Interaction with other configs
    /// Setting this config option overrides every other config option that affects the field's
    /// generated type, including `optional_repr`, `int_size`, and `boxed` (but not
    /// `field_attributes`). If the field is optional, then the custom type is responsible for
    /// tracking field presence, since custom fields aren't tracked by the hazzer.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config, config::CustomField};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Make the generator generate `foo: crate::CustomHandler` for field `foo`
    /// gen.configure(
    ///     ".Message.foo",
    ///     Config::new().custom_field(CustomField::from_type("crate::CustomHandler"))
    /// );
    /// // Decoding and encoding of `bar` will also be handled by the `CustomHandler` assigned to `foo`
    /// gen.configure(
    ///     ".Message.bar",
    ///     Config::new().custom_field(CustomField::from_delegate("foo"))
    /// );
    /// ```
    custom_field: Option<CustomField>,

    /// Rename a field in the generated Rust struct.
    ///
    /// Instead of the protobuf field name, use a different name for the generated field and its
    /// accessors. Applies to normal fields as well as oneofs and oneof variants.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // `super` can't be a field identifier, so we need to rename it
    /// gen.configure(".Message.super", Config::new().rename_field("super_"));
    /// // The oneof field will be renamed to `oneof`, and the oneof type will be `Oneof`
    /// gen.configure(".Message.my_oneof", Config::new().rename_field("oneof"));
    /// ```
    ///
    /// # Note
    /// This configuration is only applied to the path passed to
    /// [`configure`](crate::Generator::configure). It is not propagated to "children" paths.
    [no_inherit] rename_field: [deref] Option<String>,

    /// Override the max size of the field on the wire.
    ///
    /// Instead of calculating the max size of the field, the generator will use this value instead
    /// when determining the max size of the entire message. This is useful for fields with
    /// "unbounded" size, such as `Vec` fields and recursive fields. Applies to normal fields,
    /// oneof fields, and oneof variants.
    encoded_max_size: Option<usize>,

    /// Specify lifetime parameter of a message field.
    ///
    /// If message type `Inner` has fields with a lifetime, its message struct will be generated
    /// with that lifetime parameter. However, if another message type `Outer` has `Inner` as its
    /// field, then that field must specify `field_lifetime` so that the lifetime is included in
    /// the field declaration.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // `Inner` now has a lifetime param
    /// gen.configure(".Inner.field", Config::new().string_type("MyString<'a>"));
    /// // Make sure inner is declared as `inner: Inner<'a>`
    /// // Will also automatically add the lifetime param to declaration of `Outer`
    /// gen.configure(".Outer.inner", Config::new().field_lifetime("'a"));
    /// ```
    field_lifetime: [deref] Option<String>,

    // Type configs

    /// Override the integer size of Protobuf enums.
    ///
    /// Change the integer fields to be `i8`, `i16`, `i32`, or `i64`. If the integer type is
    /// smaller than the value on the wire, the value will be truncated to fit.
    enum_int_size: Option<IntSize>,

    /// Set attributes for generated types, such as messages and enums.
    ///
    /// The attribute string will be placed before type definitions. The string must be in the
    /// syntax of 0 or more Rust attributes.
    ///
    /// # Example
    /// ```no_run
    /// # use micropb_gen::{Generator, Config};
    /// # let mut gen = micropb_gen::Generator::new();
    /// // Set 2 type attributes for Message
    /// gen.configure(".Message", Config::new().type_attributes("#[derive(Eq)] #[MyDerive]"));
    /// // Unset type attributes for Message
    /// gen.configure(".Message", Config::new().type_attributes(""));
    /// ```
    ///
    /// # Special cases
    /// - If applied to an oneof field, the attributes are applied to the oneof enum type
    /// definition inside the message.
    /// - If applied to the `._has` suffix, the attributes are applied to the hazzer type
    /// definition inside the message.
    type_attributes: [deref] Option<String>,

    /// Disable generating `Debug` trait derives for message types.
    no_debug_impl: Option<bool>,

    /// Disable generating `Default` trait impl for message types.
    ///
    /// This can cause compile errors if decoding logic is being generated, because decoding
    /// repeated and `map` fields requires the elements to implement `Default`.
    no_default_impl: Option<bool>,

    /// Disable generating `PartialEq` trait impl for message types.
    no_partial_eq_impl: Option<bool>,

    /// Disable generating `Clone` trait derives for message types.
    no_clone_impl: Option<bool>,

    /// Add a custom handler on a message struct for handling unknown fields.
    ///
    /// When decoding a message, unknown fields are skipped by default. If a message has
    /// `unknown_handler` configured to a type name, a field of that type named `_unknown` will be
    /// added to the message struct. This field will handle decoding of all unknown fields and will
    /// also be encoded, so the handler type must implement `FieldEncode` and `FieldDecode`,
    /// like with [`custom_field`](Config::custom_field).
    ///
    /// # Note
    /// This configuration is only applied to the path passed to
    /// [`configure`](crate::Generator::configure). It is not propagated to "children" paths.
    [no_inherit] unknown_handler: [deref] Option<String>,

    // General configs

    /// Skip generating a type or field
    ///
    /// If applied to message or enum, the whole type definition will be skipped. If applied to a
    /// field, it won't be included in the message struct.
    skip: Option<bool>,
}

impl Config {
    /// Ensure proper handling of a recursive field by boxing it and hardcoding its max size to 0
    ///
    /// Combination of [`Self::boxed`] and [`Self::encoded_max_size`].
    pub fn recursive_field(self) -> Self {
        self.boxed(true).encoded_max_size(0)
    }
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

    pub(crate) fn rust_field_name(&self, name: &str) -> Result<(String, Ident), String> {
        if let Some(s) = &self.rename_field {
            // expect user-supplied names to not require sanitization
            Ok((
                s.to_owned(),
                syn::parse_str(s).map_err(|e| {
                    format!("Failed to parse rename_field \"{s}\" as identifier: {e}")
                })?,
            ))
        } else {
            Ok((name.to_owned(), sanitized_ident(name)))
        }
    }

    pub(crate) fn string_type_parsed(&self, n: Option<u32>) -> Result<Option<syn::Type>, String> {
        self.string_type
            .as_ref()
            .map(|t| {
                let typestr = substitute_param(t.into(), "$N", n);
                syn::parse_str(&typestr).map_err(|e| {
                    format!("Failed to parse string_type \"{typestr}\" as type path: {e}")
                })
            })
            .transpose()
    }

    pub(crate) fn bytes_type_parsed(&self, n: Option<u32>) -> Result<Option<syn::Type>, String> {
        self.bytes_type
            .as_ref()
            .map(|t| {
                let typestr = substitute_param(t.into(), "$N", n);
                syn::parse_str(&typestr).map_err(|e| {
                    format!("Failed to parse bytes_type \"{typestr}\" as type path: {e}")
                })
            })
            .transpose()
    }

    pub(crate) fn vec_type_parsed(
        &self,
        t: TokenStream,
        n: Option<u32>,
    ) -> Result<Option<syn::Type>, String> {
        self.vec_type
            .as_ref()
            .map(|typestr| {
                let typestr = substitute_param(typestr.into(), "$N", n);
                let typestr = substitute_param(typestr, "$T", Some(t));
                syn::parse_str(&typestr).map_err(|e| {
                    format!("Failed to parse vec_type \"{typestr}\" as type path: {e}")
                })
            })
            .transpose()
    }

    pub(crate) fn map_type_parsed(
        &self,
        k: TokenStream,
        v: TokenStream,
        n: Option<u32>,
    ) -> Result<Option<syn::Type>, String> {
        self.map_type
            .as_ref()
            .map(|t| {
                let typestr = substitute_param(t.into(), "$N", n);
                let typestr = substitute_param(typestr, "$K", Some(k));
                let typestr = substitute_param(typestr, "$V", Some(v));
                syn::parse_str(&typestr).map_err(|e| {
                    format!("Failed to parse map_type \"{typestr}\" as type path: {e}")
                })
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

    pub(crate) fn field_lifetime_parsed(&self) -> Result<Option<syn::Lifetime>, String> {
        self.field_lifetime
            .as_ref()
            .map(|l| {
                syn::parse_str(l)
                    .map_err(|e| format!("Failed to parse \"{l}\" as Rust lifetime: {e}"))
            })
            .transpose()
    }
}

fn substitute_param<'a>(
    typestr: Cow<'a, str>,
    pat: &str,
    t: Option<impl ToString>,
) -> Cow<'a, str> {
    if let Some(t) = t {
        if typestr.find(pat).is_some() {
            let t = t.to_string();
            return typestr.replace(pat, &t).into();
        }
    }
    typestr
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
            .vec_type("heapless::Vec<$T, $N>")
            .string_type("heapless::String<$N>")
            .map_type("Map<$K, $V, $N>")
            .bytes_type("Bytes")
            .type_attributes("#[derive(Hash)]");

        assert_eq!(
            config
                .vec_type_parsed(quote! {u8}, Some(5))
                .unwrap()
                .to_token_stream()
                .to_string(),
            quote! { heapless::Vec<u8, 5> }.to_string()
        );
        assert_eq!(
            config
                .string_type_parsed(Some(12))
                .unwrap()
                .to_token_stream()
                .to_string(),
            quote! { heapless::String<12> }.to_string()
        );
        assert_eq!(
            config
                .map_type_parsed(quote! {u32}, quote! {bool}, Some(14))
                .unwrap()
                .to_token_stream()
                .to_string(),
            quote! { Map<u32, bool, 14> }.to_string()
        );
        assert_eq!(
            config
                .bytes_type_parsed(None)
                .unwrap()
                .to_token_stream()
                .to_string(),
            "Bytes"
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
            ("name".to_owned(), format_ident!("r#name"))
        );
        config.rename_field = Some("rename".to_string());
        assert_eq!(
            config.rust_field_name("name").unwrap(),
            ("rename".to_owned(), format_ident!("rename"))
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
