#![no_std]

use core::ops::{AddAssign, BitOrAssign, Shl};

enum Error {
    VarIntLimit(u8),
    UnexpectedEof,
    Deprecation,
    BadWireType(u8),
}

trait VarIntDecode: BitOrAssign + Shl<u8, Output = Self> + From<u8> + Copy {
    const BITS: u8;
    const BYTES: u8 = Self::BITS / 7;
}

impl VarIntDecode for u32 {
    const BITS: u8 = 35;
}

impl VarIntDecode for u64 {
    const BITS: u8 = 70;
}

pub struct ProtoReader<'a> {
    buf: &'a [u8],
    idx: usize,
}

impl<'a> ProtoReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, idx: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.idx
    }

    #[inline]
    fn get_byte(&mut self) -> Result<u8, Error> {
        if self.remaining() == 0 {
            return Err(Error::UnexpectedEof);
        }
        let b = self.buf[self.idx];
        self.idx += 1;
        Ok(b)
    }

    fn decode_varint<U: VarIntDecode>(&mut self) -> Result<U, Error> {
        let b = self.get_byte()?;
        let mut varint = U::from(b & !0x80);
        // Single byte case
        if b & 0x80 == 0 {
            return Ok(varint);
        }

        let mut bitpos = 7;
        loop {
            let b = self.get_byte()?;
            // possible truncation in the last byte
            varint |= U::from(b & !0x80) << bitpos;
            if b & 0x80 == 0 {
                return Ok(varint);
            }
            bitpos += 7;
            if bitpos >= U::BITS {
                return Err(Error::VarIntLimit(U::BYTES));
            }
        }
    }

    fn decode_uint32(&mut self) -> Result<u32, Error> {
        self.decode_varint::<u32>()
    }

    fn decode_uint64(&mut self) -> Result<u64, Error> {
        self.decode_varint::<u64>()
    }

    fn decode_int64(&mut self) -> Result<i64, Error> {
        self.decode_uint64().map(|u| u as i64)
    }

    fn decode_int32(&mut self) -> Result<i32, Error> {
        self.decode_int64().map(|u| u as i32)
    }

    fn decode_sint32(&mut self) -> Result<i32, Error> {
        self.decode_uint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    fn decode_sint64(&mut self) -> Result<i64, Error> {
        self.decode_uint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    fn decode_bool(&mut self) -> Result<bool, Error> {
        let b = self.get_byte()?;
        Ok(b != 0)
    }

    fn get_slice(&self, size: usize) -> Result<&[u8], Error> {
        if self.remaining() < size {
            return Err(Error::UnexpectedEof);
        }
        Ok(&self.buf[self.idx..self.idx + size])
    }

    fn decode_fixed32(&mut self) -> Result<u32, Error> {
        let bytes = self.get_slice(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn decode_fixed64(&mut self) -> Result<u64, Error> {
        let bytes = self.get_slice(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn decode_sfixed32(&mut self) -> Result<i32, Error> {
        self.decode_fixed32().map(|u| u as i32)
    }

    fn decode_sfixed64(&mut self) -> Result<i64, Error> {
        self.decode_fixed64().map(|u| u as i64)
    }

    fn decode_float(&mut self) -> Result<f32, Error> {
        self.decode_fixed32().map(f32::from_bits)
    }

    fn decode_double(&mut self) -> Result<f64, Error> {
        self.decode_fixed64().map(f64::from_bits)
    }

    fn decode_tag(&mut self) -> Result<Tag, Error> {
        let u = self.decode_uint32()?;
        let field_num = u >> 3;
        let wire_type = match u & 0b111 {
            0 => WireType::Varint,
            1 => WireType::I64,
            2 => WireType::Len,
            3 | 4 => return Err(Error::Deprecation),
            5 => WireType::I32,
            w => return Err(Error::BadWireType(w as u8)),
        };
        Ok(Tag {
            field_num,
            wire_type,
        })
    }
}

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
