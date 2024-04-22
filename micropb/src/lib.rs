#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use num_traits::{AsPrimitive, PrimInt};

pub mod container;
#[cfg(feature = "decode")]
pub mod decode;
#[cfg(feature = "encode")]
pub mod encode;
pub mod field;
pub mod message;
mod misc;
#[cfg(feature = "encode")]
pub mod size;

#[cfg(feature = "container-arrayvec")]
pub use ::arrayvec;
#[cfg(feature = "container-heapless")]
pub use ::heapless;

pub use ::bitvec::BitArr;

pub use container::{PbContainer, PbMap, PbString, PbVec};
#[cfg(feature = "decode")]
pub use decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
pub use encode::{PbEncoder, PbWrite};
#[cfg(feature = "decode")]
pub use field::FieldDecode;
#[cfg(feature = "encode")]
pub use field::FieldEncode;
#[cfg(feature = "decode")]
pub use message::MessageDecode;
#[cfg(feature = "encode")]
pub use message::MessageEncode;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LEN: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tag(u32);

impl Tag {
    #[inline]
    pub fn from_parts(field_num: u32, wire_type: u8) -> Self {
        debug_assert!(wire_type <= 7);
        Self((field_num << 3) | (wire_type as u32))
    }

    #[inline]
    pub fn wire_type(&self) -> u8 {
        (self.0 & 0b111) as u8
    }

    #[inline]
    pub fn field_num(&self) -> u32 {
        self.0 >> 3
    }

    #[inline]
    pub fn varint(&self) -> u32 {
        self.0
    }
}

trait VarInt: PrimInt + From<u8> + AsPrimitive<u8> {
    const BYTES: u8;
}

impl VarInt for u32 {
    const BYTES: u8 = 5;
}

impl VarInt for u64 {
    const BYTES: u8 = 10;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    Implicit,
    Explicit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag() {
        let tag = Tag::from_parts(5, 4);
        assert_eq!(tag.varint(), 0x2C);
        assert_eq!(tag.field_num(), 5);
        assert_eq!(tag.wire_type(), 4);

        let tag = Tag::from_parts(0, 0);
        assert_eq!(tag.varint(), 0);
        assert_eq!(tag.field_num(), 0);
        assert_eq!(tag.wire_type(), 0);
    }
}
