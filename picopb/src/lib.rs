#![cfg_attr(not(test), no_std)]

pub mod container;
pub mod decode;
pub mod message;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LEN: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

#[derive(Debug, PartialEq)]
pub struct Tag {
    field_num: u32,
    wire_type: u8,
}

impl Tag {
    pub fn wire_type(&self) -> u8 {
        self.wire_type
    }

    pub fn field_num(&self) -> u32 {
        self.field_num
    }
}
