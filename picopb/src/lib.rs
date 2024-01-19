#![no_std]

pub mod container;
pub mod decode;
pub mod message;

#[derive(Debug)]
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
