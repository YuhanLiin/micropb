#![no_std]

use core::ops::{AddAssign, BitOrAssign, Shl};

use bytes::Buf;

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

fn get_byte<B: Buf>(buf: &mut B) -> Result<u8, Error> {
    if !buf.has_remaining() {
        return Err(Error::UnexpectedEof);
    }
    Ok(buf.get_u8())
}

fn decode_varint<U: VarIntDecode, B: Buf>(buf: &mut B) -> Result<U, Error> {
    let b = get_byte(buf)?;
    let mut varint = U::from(b & !0x80);
    // Single byte case
    if b & 0x80 == 0 {
        return Ok(varint);
    }

    let mut bitpos = 7;
    loop {
        let b = get_byte(buf)?;
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

fn decode_uint32<B: Buf>(buf: &mut B) -> Result<u32, Error> {
    decode_varint::<u32, _>(buf)
}

fn decode_uint64<B: Buf>(buf: &mut B) -> Result<u64, Error> {
    decode_varint::<u64, _>(buf)
}

fn decode_int64<B: Buf>(buf: &mut B) -> Result<i64, Error> {
    decode_uint64(buf).map(|u| u as i64)
}

fn decode_int32<B: Buf>(buf: &mut B) -> Result<i32, Error> {
    decode_int64(buf).map(|u| u as i32)
}

fn decode_sint32<B: Buf>(buf: &mut B) -> Result<i32, Error> {
    decode_uint32(buf).map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
}

fn decode_sint64<B: Buf>(buf: &mut B) -> Result<i64, Error> {
    decode_uint64(buf).map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
}

fn decode_bool<B: Buf>(buf: &mut B) -> Result<bool, Error> {
    let b = buf.get_u8();
    Ok(b != 0)
}

fn decode_fixed<B: Buf, T, F: Fn(&mut B) -> T>(buf: &mut B, size: usize, f: F) -> Result<T, Error> {
    if buf.remaining() < size {
        return Err(Error::UnexpectedEof);
    }
    Ok(f(buf))
}

fn decode_fixed32<B: Buf>(buf: &mut B) -> Result<u32, Error> {
    decode_fixed(buf, 4, B::get_u32_le)
}

fn decode_fixed64<B: Buf>(buf: &mut B) -> Result<u64, Error> {
    decode_fixed(buf, 8, B::get_u64_le)
}

fn decode_sfixed32<B: Buf>(buf: &mut B) -> Result<i32, Error> {
    decode_fixed(buf, 4, B::get_i32_le)
}

fn decode_sfixed64<B: Buf>(buf: &mut B) -> Result<i64, Error> {
    decode_fixed(buf, 8, B::get_i64_le)
}

fn decode_float<B: Buf>(buf: &mut B) -> Result<f32, Error> {
    decode_fixed(buf, 4, B::get_f32_le)
}

fn decode_double<B: Buf>(buf: &mut B) -> Result<f64, Error> {
    decode_fixed(buf, 8, B::get_f64_le)
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

fn decode_tag<B: Buf>(buf: &mut B) -> Result<Tag, Error> {
    let u = decode_uint32(buf)?;
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
