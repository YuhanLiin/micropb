use core::{
    ops::{BitOrAssign, Shl},
    str::{from_utf8, Utf8Error},
};

use crate::{
    container::{PbString, PbVec},
    Tag, WireType,
};

#[derive(Debug)]
enum DecodeError {
    VarIntLimit(u8),
    UnexpectedEof,
    Deprecation,
    BadWireType(u8),
    Utf8(Utf8Error),
    Capacity,
}

impl From<Utf8Error> for DecodeError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

trait VarIntDecode: BitOrAssign + Shl<u8, Output = Self> + From<u8> + Copy {
    const BYTES: u8;
}

impl VarIntDecode for u32 {
    const BYTES: u8 = 5;
}

impl VarIntDecode for u64 {
    const BYTES: u8 = 10;
}

pub struct PbReader<'a> {
    buf: &'a [u8],
    idx: usize,
}

impl<'a> PbReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, idx: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.idx
    }

    #[inline]
    fn get_byte(&mut self) -> Result<u8, DecodeError> {
        if self.remaining() == 0 {
            return Err(DecodeError::UnexpectedEof);
        }
        let b = self.buf[self.idx];
        self.idx += 1;
        Ok(b)
    }

    fn decode_varint<U: VarIntDecode>(&mut self) -> Result<U, DecodeError> {
        let b = self.get_byte()?;
        let mut varint = U::from(b & !0x80);
        // Single byte case
        if b & 0x80 == 0 {
            return Ok(varint);
        }

        let mut bitpos = 7;
        for _ in 1..U::BYTES {
            let b = self.get_byte()?;
            // possible truncation in the last byte
            varint |= U::from(b & !0x80) << bitpos;
            if b & 0x80 == 0 {
                return Ok(varint);
            }
            bitpos += 7;
        }
        Err(DecodeError::VarIntLimit(U::BYTES))
    }

    fn decode_uint32(&mut self) -> Result<u32, DecodeError> {
        self.decode_varint::<u32>()
    }

    fn decode_uint64(&mut self) -> Result<u64, DecodeError> {
        self.decode_varint::<u64>()
    }

    fn decode_int64(&mut self) -> Result<i64, DecodeError> {
        self.decode_uint64().map(|u| u as i64)
    }

    fn decode_int32(&mut self) -> Result<i32, DecodeError> {
        self.decode_int64().map(|u| u as i32)
    }

    fn decode_sint32(&mut self) -> Result<i32, DecodeError> {
        self.decode_uint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    fn decode_sint64(&mut self) -> Result<i64, DecodeError> {
        self.decode_uint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        let b = self.get_byte()?;
        if b & 0x80 != 0 {
            return Err(DecodeError::VarIntLimit(1));
        }
        Ok(b != 0)
    }

    fn get_slice(&self, size: usize) -> Result<&[u8], DecodeError> {
        if self.remaining() < size {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(&self.buf[self.idx..self.idx + size])
    }

    fn decode_fixed32(&mut self) -> Result<u32, DecodeError> {
        let bytes = self.get_slice(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn decode_fixed64(&mut self) -> Result<u64, DecodeError> {
        let bytes = self.get_slice(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn decode_sfixed32(&mut self) -> Result<i32, DecodeError> {
        self.decode_fixed32().map(|u| u as i32)
    }

    fn decode_sfixed64(&mut self) -> Result<i64, DecodeError> {
        self.decode_fixed64().map(|u| u as i64)
    }

    fn decode_float(&mut self) -> Result<f32, DecodeError> {
        self.decode_fixed32().map(f32::from_bits)
    }

    fn decode_double(&mut self) -> Result<f64, DecodeError> {
        self.decode_fixed64().map(f64::from_bits)
    }

    fn decode_tag(&mut self) -> Result<Tag, DecodeError> {
        let u = self.decode_uint32()?;
        let field_num = u >> 3;
        let wire_type = match u & 0b111 {
            0 => WireType::Varint,
            1 => WireType::I64,
            2 => WireType::Len,
            3 | 4 => return Err(DecodeError::Deprecation),
            5 => WireType::I32,
            w => return Err(DecodeError::BadWireType(w as u8)),
        };
        Ok(Tag {
            field_num,
            wire_type,
        })
    }

    fn decode_len_slice(&mut self) -> Result<&[u8], DecodeError> {
        let len = self.decode_uint32()?;
        self.get_slice(len as usize)
    }

    fn decode_string<S: PbString>(&mut self, string: &mut S) -> Result<(), DecodeError> {
        let slice = self.decode_len_slice()?;
        let s = from_utf8(slice)?;
        string.write_str(s).map_err(|_| DecodeError::Capacity)
    }

    fn decode_bytes<S: PbVec<u8>>(&mut self, bytes: &mut S) -> Result<(), DecodeError> {
        let slice = self.decode_len_slice()?;
        bytes.write_slice(slice).map_err(|_| DecodeError::Capacity)
    }

    fn decode_packed<
        T: Copy,
        S: PbVec<T>,
        F: for<'b> Fn(&mut PbReader<'b>) -> Result<T, DecodeError>,
    >(
        &mut self,
        vec: &mut S,
        decoder: F,
    ) -> Result<(), DecodeError> {
        let mut reader = PbReader::new(self.decode_len_slice()?);
        while reader.remaining() > 0 {
            let val = decoder(&mut reader)?;
            vec.push(val).map_err(|_| DecodeError::Capacity)?;
        }
        Ok(())
    }

    fn skip_varint(&mut self) -> Result<(), DecodeError> {
        for _ in 0..u64::BYTES {
            let b = self.get_byte()?;
            if b & 0x80 == 0 {
                return Ok(());
            }
        }
        Err(DecodeError::VarIntLimit(u64::BYTES))
    }

    fn skip_wire_value(&mut self, wire_type: WireType) -> Result<(), DecodeError> {
        match wire_type {
            WireType::Varint => self.skip_varint()?,
            WireType::I64 => drop(self.get_slice(8)?),
            WireType::Len => drop(self.decode_len_slice()?),
            WireType::I32 => drop(self.get_slice(4)?),
        }
        Ok(())
    }
}
