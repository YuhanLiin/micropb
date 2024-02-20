use crate::pathtree::PathTree;

#[derive(Debug, Clone, Copy)]
pub enum IntType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
}

impl IntType {
    fn type_name(self) -> &'static str {
        match self {
            IntType::I8 => "i8",
            IntType::U8 => "u8",
            IntType::I16 => "i16",
            IntType::U16 => "u16",
            IntType::I32 => "i32",
            IntType::U32 => "u32",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldConfig {
    pub(crate) fixed_len: Option<u32>,
    pub(crate) int_type: Option<IntType>,
    pub(crate) custom_type: Option<String>,
    pub(crate) container_type: Option<String>,
    pub(crate) attributes: String,
    pub(crate) boxed: bool,
}

impl FieldConfig {
    pub fn merge(&mut self, other: &Self) {
        if let Some(v) = other.fixed_len {
            self.fixed_len = Some(v);
        }
        if let Some(v) = other.int_type {
            self.int_type = Some(v);
        }
        if let Some(v) = &other.custom_type {
            self.custom_type = Some(v.clone());
        }
        if let Some(v) = &other.container_type {
            self.container_type = Some(v.clone());
        }
        self.attributes += &other.attributes;
        if other.boxed {
            self.boxed = true;
        }
    }

    pub fn merge_clone(&self, other: &Self) -> Self {
        let mut c = self.clone();
        c.merge(other);
        c
    }

    pub fn fixed_len(mut self, len: u32) -> Self {
        self.fixed_len = Some(len);
        self
    }

    pub fn int_type(mut self, int_type: IntType) -> Self {
        self.int_type = Some(int_type);
        self
    }

    pub fn custom_type(mut self, type_name: &str) -> Self {
        self.custom_type = Some(type_name.to_owned());
        self
    }

    pub fn container_type(mut self, type_name: &str) -> Self {
        self.container_type = Some(type_name.to_owned());
        self
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = attributes.to_owned();
        self
    }

    pub fn boxed(mut self) -> Self {
        self.boxed = true;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeConfig {
    pub(crate) enum_int_type: Option<IntType>,
    pub(crate) attributes: String,
    pub(crate) skip_debug: bool,
}

impl TypeConfig {
    pub fn merge(&mut self, other: &Self) {
        if let Some(v) = other.enum_int_type {
            self.enum_int_type = Some(v);
        }
        self.attributes += &other.attributes;
        if other.skip_debug {
            self.skip_debug = true;
        }
    }

    pub fn merge_clone(&self, other: &Self) -> Self {
        let mut c = self.clone();
        c.merge(other);
        c
    }

    pub fn enum_int_type(mut self, enum_int_type: IntType) -> Self {
        self.enum_int_type = Some(enum_int_type);
        self
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = attributes.to_owned();
        self
    }

    pub fn skip_debug(mut self) -> Self {
        self.skip_debug = true;
        self
    }
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
    pub(crate) fixed_vec_type: String,
    pub(crate) fixed_string_type: String,
    pub(crate) fixed_map_type: String,
    pub(crate) alloc_vec_type: String,
    pub(crate) alloc_string_type: String,
    pub(crate) alloc_map_type: String,

    pub(crate) field_configs: PathTree<FieldConfig>,
    pub(crate) type_configs: PathTree<TypeConfig>,
}
