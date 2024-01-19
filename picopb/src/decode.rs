use core::{
    ops::{BitOrAssign, Shl},
    str::{from_utf8, Utf8Error},
};

use crate::{
    container::{PbString, PbVec},
    Tag,
};

#[derive(Debug)]
pub enum DecodeError {
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

type DecodeFn<T> = fn(&mut PbReader) -> Result<T, DecodeError>;

#[derive(Debug)]
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

    pub fn decode_uint32(&mut self) -> Result<u32, DecodeError> {
        self.decode_varint::<u32>()
    }

    pub fn decode_uint64(&mut self) -> Result<u64, DecodeError> {
        self.decode_varint::<u64>()
    }

    pub fn decode_int64(&mut self) -> Result<i64, DecodeError> {
        self.decode_uint64().map(|u| u as i64)
    }

    pub fn decode_int32(&mut self) -> Result<i32, DecodeError> {
        self.decode_int64().map(|u| u as i32)
    }

    pub fn decode_sint32(&mut self) -> Result<i32, DecodeError> {
        self.decode_uint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    pub fn decode_sint64(&mut self) -> Result<i64, DecodeError> {
        self.decode_uint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    pub fn decode_bool(&mut self) -> Result<bool, DecodeError> {
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

    pub fn decode_fixed32(&mut self) -> Result<u32, DecodeError> {
        let bytes = self.get_slice(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn decode_fixed64(&mut self) -> Result<u64, DecodeError> {
        let bytes = self.get_slice(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn decode_sfixed32(&mut self) -> Result<i32, DecodeError> {
        self.decode_fixed32().map(|u| u as i32)
    }

    pub fn decode_sfixed64(&mut self) -> Result<i64, DecodeError> {
        self.decode_fixed64().map(|u| u as i64)
    }

    pub fn decode_float(&mut self) -> Result<f32, DecodeError> {
        self.decode_fixed32().map(f32::from_bits)
    }

    pub fn decode_double(&mut self) -> Result<f64, DecodeError> {
        self.decode_fixed64().map(f64::from_bits)
    }

    #[inline(always)]
    pub fn decode_tag(&mut self) -> Result<Tag, DecodeError> {
        let u = self.decode_uint32()?;
        let field_num = u >> 3;
        let wire_type = (u | 0b111) as u8;
        Ok(Tag {
            field_num,
            wire_type,
        })
    }

    pub fn decode_len_slice(&mut self) -> Result<&[u8], DecodeError> {
        let len = self.decode_uint32()?;
        self.get_slice(len as usize)
    }

    pub fn decode_string<S: PbString>(&mut self, string: &mut S) -> Result<(), DecodeError> {
        let slice = self.decode_len_slice()?;
        let s = from_utf8(slice)?;
        string.write_str(s).map_err(|_| DecodeError::Capacity)
    }

    pub fn decode_bytes<S: PbVec<u8>>(&mut self, bytes: &mut S) -> Result<(), DecodeError> {
        let slice = self.decode_len_slice()?;
        bytes.write_slice(slice).map_err(|_| DecodeError::Capacity)
    }

    pub fn decode_packed<T: Copy, S: PbVec<T>>(
        &mut self,
        vec: &mut S,
        decoder: DecodeFn<T>,
    ) -> Result<(), DecodeError> {
        let mut reader = PbReader::new(self.decode_len_slice()?);
        while reader.remaining() > 0 {
            let val = decoder(&mut reader)?;
            vec.push(val).map_err(|_| DecodeError::Capacity)?;
        }
        Ok(())
    }

    pub fn decode_map_elem<
        K: Default,
        V: Default,
        UK: Fn(&mut Option<K>, &mut PbReader) -> Result<(), DecodeError>,
        UV: Fn(&mut Option<V>, &mut PbReader) -> Result<(), DecodeError>,
    >(
        &mut self,
        key_update: UK,
        val_update: UV,
    ) -> Result<Option<(K, V)>, DecodeError> {
        let mut reader = PbReader::new(self.decode_len_slice()?);
        let mut key = None;
        let mut val = None;
        while reader.remaining() > 0 {
            let tag = reader.decode_tag()?;
            match tag.field_num {
                1 => key_update(&mut key, &mut reader)?,
                2 => val_update(&mut val, &mut reader)?,
                _ => reader.skip_wire_value(&tag)?,
            }
        }

        if let (Some(key), Some(val)) = (key, val) {
            Ok(Some((key, val)))
        } else {
            Ok(None)
        }
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

    pub fn skip_wire_value(&mut self, tag: &Tag) -> Result<(), DecodeError> {
        match tag.wire_type {
            0 => self.skip_varint()?,
            1 => drop(self.get_slice(8)?),
            2 => drop(self.decode_len_slice()?),
            3 | 4 => return Err(DecodeError::Deprecation),
            5 => drop(self.get_slice(4)?),
            w => return Err(DecodeError::BadWireType(w)),
        }
        Ok(())
    }
}
