use crate::pathtree::PathTree;

#[derive(Debug, Clone, Copy)]
pub enum IntType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    Isize,
    Usize,
}

impl IntType {
    pub(crate) fn type_name(self) -> &'static str {
        match self {
            IntType::I8 => "i8",
            IntType::U8 => "u8",
            IntType::I16 => "i16",
            IntType::U16 => "u16",
            IntType::I32 => "i32",
            IntType::U32 => "u32",
            IntType::Isize => "isize",
            IntType::Usize => "usize",
        }
    }

    pub(crate) fn is_signed(self) -> bool {
        matches!(
            self,
            IntType::I8 | IntType::I16 | IntType::I32 | IntType::Isize
        )
    }
}

#[derive(Debug, Clone)]
pub enum CustomField {
    Type(String),
    Delegate(String),
}

macro_rules! config_decl {
    ($($(#[$attr:meta])* $field:ident $([$placeholder:ident])?: Option<$type:ty>,)+) => {
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
                $(config_decl!(@merge $field $([$placeholder])?, self, other);)+
            }

            $(config_decl!(@setter $field: $type);)+
        }
    };

    (@merge $field:ident, $self:ident, $other:ident) => {
        if let Some(v) = &$other.$field {
            $self.$field = Some(v.clone());
        }
    };

    (@merge $field:ident [no_inherit], $self:ident, $other:ident) => {
        $self.$field = $other.$field.clone();
    };

    (@setter $field:ident: String) => {
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
    fixed_len: Option<u32>,
    int_type: Option<IntType>,
    field_attributes: Option<String>,
    boxed: Option<bool>,
    vec_type: Option<String>,
    string_type: Option<String>,
    map_type: Option<String>,
    no_hazzer: Option<bool>,
    custom_field [no_inherit]: Option<CustomField>,
    rename_field [no_inherit]: Option<String>,

    // Type configs
    enum_int_type: Option<IntType>,
    type_attributes: Option<String>,
    hazzer_attributes: Option<String>,
    no_debug_derive: Option<bool>,

    // General configs
    skip: Option<bool>,
}

#[derive(Debug, Clone, Copy, Default)]
enum EncodeDecode {
    EncodeOnly,
    DecodeOnly,
    #[default]
    Both,
}

pub struct GenConfig {
    pub(crate) encode_decode: EncodeDecode,
    pub(crate) size_cache: bool,
    pub(crate) default_pkg_filename: String,
    pub(crate) micropb_path: String,
    pub(crate) strip_enum_prefix: bool,
    pub(crate) format: bool,

    pub(crate) field_configs: PathTree<Config>,
}
