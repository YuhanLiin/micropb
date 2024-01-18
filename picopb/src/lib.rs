#![no_std]

pub mod container;
pub mod decode;

enum WireType {
    Varint,
    I64,
    Len,
    I32,
}

struct Tag {
    field_num: u32,
    wire_type: WireType,
}
