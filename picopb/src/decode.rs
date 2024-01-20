use core::str::{from_utf8, Utf8Error};

use crate::{
    container::{PbString, PbVec},
    Tag, VarInt, WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LEN, WIRE_TYPE_VARINT,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct PbDecoder<'a> {
    buf: &'a [u8],
    idx: usize,
}

impl<'a> PbDecoder<'a> {
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

    fn decode_varint<U: VarInt>(&mut self) -> Result<U, DecodeError> {
        let b = self.get_byte()?;
        let mut varint = From::from(b & !0x80);
        // Single byte case
        if b & 0x80 == 0 {
            return Ok(varint);
        }

        let mut bitpos = 7;
        for _ in 1..U::BYTES {
            let b = self.get_byte()?;
            // possible truncation in the last byte
            let u: U = From::from(b & !0x80);
            varint = varint | u << bitpos;
            if b & 0x80 == 0 {
                return Ok(varint);
            }
            bitpos += 7;
        }
        Err(DecodeError::VarIntLimit(U::BYTES))
    }

    pub fn decode_varint32(&mut self) -> Result<u32, DecodeError> {
        self.decode_varint::<u32>()
    }

    pub fn decode_varint64(&mut self) -> Result<u64, DecodeError> {
        self.decode_varint::<u64>()
    }

    pub fn decode_int64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint64().map(|u| u as i64)
    }

    pub fn decode_int32(&mut self) -> Result<i32, DecodeError> {
        self.decode_int64().map(|u| u as i32)
    }

    pub fn decode_sint32(&mut self) -> Result<i32, DecodeError> {
        self.decode_varint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    pub fn decode_sint64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    pub fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        let b = self.get_byte()?;
        if b & 0x80 != 0 {
            return Err(DecodeError::VarIntLimit(1));
        }
        Ok(b != 0)
    }

    fn get_slice(&mut self, size: usize) -> Result<&[u8], DecodeError> {
        if self.remaining() < size {
            return Err(DecodeError::UnexpectedEof);
        }
        let idx = self.idx;
        self.idx += size;
        Ok(&self.buf[idx..idx + size])
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
        let u = self.decode_varint32()?;
        let field_num = u >> 3;
        let wire_type = (u & 0b111) as u8;
        Ok(Tag {
            field_num,
            wire_type,
        })
    }

    pub fn decode_len_slice(&mut self) -> Result<&[u8], DecodeError> {
        let len = self.decode_varint32()?;
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

    pub fn decode_packed<T: Copy, S: PbVec<T>, F: Fn(&mut PbDecoder) -> Result<T, DecodeError>>(
        &mut self,
        vec: &mut S,
        decoder: F,
    ) -> Result<(), DecodeError> {
        let mut reader = PbDecoder::new(self.decode_len_slice()?);
        while reader.remaining() > 0 {
            let val = decoder(&mut reader)?;
            vec.push(val).map_err(|_| DecodeError::Capacity)?;
        }
        Ok(())
    }

    pub fn decode_map_elem<
        K,
        V,
        UK: Fn(&mut Option<K>, &mut PbDecoder) -> Result<(), DecodeError>,
        UV: Fn(&mut Option<V>, &mut PbDecoder) -> Result<(), DecodeError>,
    >(
        &mut self,
        key_update: UK,
        val_update: UV,
    ) -> Result<Option<(K, V)>, DecodeError> {
        let mut reader = PbDecoder::new(self.decode_len_slice()?);
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
            WIRE_TYPE_VARINT => self.skip_varint()?,
            WIRE_TYPE_I64 => drop(self.get_slice(8)?),
            WIRE_TYPE_LEN => drop(self.decode_len_slice()?),
            3 | 4 => return Err(DecodeError::Deprecation),
            WIRE_TYPE_I32 => drop(self.get_slice(4)?),
            w => return Err(DecodeError::BadWireType(w)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Deref;

    use arrayvec::{ArrayString, ArrayVec};

    use super::*;

    macro_rules! assert_decode {
        ($expected:expr, $arr:expr, $($op:tt)+) => {
            let mut reader = PbDecoder::new(&$arr);
            let res = reader.$($op)+;
            assert_eq!($expected, res);
            // Check that the reader is empty only when the decoding is successful
            if res.is_ok() {
                assert_eq!(reader.remaining(), 0);
            }
        };
    }

    #[test]
    fn varint32() {
        assert_decode!(Ok(5), [5], decode_varint32());
        assert_decode!(Ok(150), [0x96, 0x01], decode_varint32());
        assert_decode!(
            Ok(0b1010000001110010101),
            [0x95, 0x87, 0x14],
            decode_varint32()
        );
        // Last byte is partially truncated in the output
        assert_decode!(
            Ok(0b11110000000000000000000000000001),
            [0x81, 0x80, 0x80, 0x80, 0x7F],
            decode_varint32()
        );

        assert_decode!(Err(DecodeError::UnexpectedEof), [0x80], decode_varint32());
        assert_decode!(Err(DecodeError::UnexpectedEof), [], decode_varint32());
        assert_decode!(
            Err(DecodeError::VarIntLimit(5)),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            decode_varint32()
        );
    }

    #[test]
    fn varint64() {
        assert_decode!(Ok(5), [5], decode_varint64());
        assert_decode!(Ok(150), [0x96, 0x01], decode_varint64());
        // Last byte is partially truncated in the output
        assert_decode!(
            Ok(0b1000000000000000000000000000000000000000000000000000000000000001),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F],
            decode_varint64()
        );

        assert_decode!(Err(DecodeError::UnexpectedEof), [0x80], decode_varint64());
        assert_decode!(Err(DecodeError::UnexpectedEof), [], decode_varint64());
        assert_decode!(
            Err(DecodeError::VarIntLimit(10)),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            decode_varint64()
        );
    }

    #[test]
    fn skip_varint() {
        assert_decode!(Ok(()), [5], skip_varint());
        assert_decode!(Ok(()), [0x96, 0x01], skip_varint());
        assert_decode!(
            Ok(()),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F],
            skip_varint()
        );

        assert_decode!(Err(DecodeError::UnexpectedEof), [0x80], skip_varint());
        assert_decode!(Err(DecodeError::UnexpectedEof), [], skip_varint());
        assert_decode!(
            Err(DecodeError::VarIntLimit(10)),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            skip_varint()
        );
    }

    #[test]
    fn int() {
        assert_decode!(Ok(5), [5], decode_int32());
        assert_decode!(Ok(5), [5], decode_int64());

        // int32 is decoded as varint64, so big varints get casted down to 32 bits
        assert_decode!(
            Ok(0b00000000000000000000000000000001),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F],
            decode_int32()
        );
        assert_decode!(
            Ok(0b100000000000000000000000000000000000000000000000000000000000001),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xC0, 0x00],
            decode_int64()
        );

        assert_decode!(
            Ok(-2),
            [0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            decode_int32()
        );
        assert_decode!(
            Ok(-2),
            [0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            decode_int64()
        );
    }

    #[test]
    fn sint32() {
        assert_decode!(Ok(0), [0], decode_sint32());
        assert_decode!(Ok(-1), [1], decode_sint32());
        assert_decode!(Ok(1), [2], decode_sint32());
        assert_decode!(Ok(-2), [3], decode_sint32());
        assert_decode!(
            Ok(0x7FFFFFFF),
            [0xFE, 0xFF, 0xFF, 0xFF, 0x7F],
            decode_sint32()
        );
        assert_decode!(
            Ok(-0x80000000),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
            decode_sint32()
        );
        assert_decode!(
            Err(DecodeError::VarIntLimit(5)),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            decode_sint32()
        );
    }

    #[test]
    fn sint64() {
        assert_decode!(Ok(0), [0], decode_sint64());
        assert_decode!(Ok(-1), [1], decode_sint64());
        assert_decode!(
            Ok(0x7FFFFFFFFFFFFFFF),
            [0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
            decode_sint64()
        );
        assert_decode!(
            Ok(-0x8000000000000000),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
            decode_sint64()
        );
        assert_decode!(
            Err(DecodeError::VarIntLimit(10)),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            decode_sint64()
        );
    }

    #[test]
    fn bool() {
        assert_decode!(Ok(false), [0], decode_bool());
        assert_decode!(Ok(true), [1], decode_bool());
        assert_decode!(Ok(true), [0x3], decode_bool());
        assert_decode!(Err(DecodeError::VarIntLimit(1)), [0x80], decode_bool());
    }

    #[test]
    fn fixed() {
        assert_decode!(Err(DecodeError::UnexpectedEof), [0], decode_fixed32());
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22],
            decode_fixed32()
        );
        assert_decode!(Ok(0xF4983212), [0x12, 0x32, 0x98, 0xF4], decode_fixed32());

        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22, 0x32, 0x9A, 0xBB, 0x3C],
            decode_fixed64()
        );
        assert_decode!(
            Ok(0x9950AA3BF4983212),
            [0x12, 0x32, 0x98, 0xF4, 0x3B, 0xAA, 0x50, 0x99],
            decode_fixed64()
        );
    }

    #[test]
    fn sfixed() {
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22],
            decode_sfixed32()
        );
        assert_decode!(Ok(-0x0B67CDEE), [0x12, 0x32, 0x98, 0xF4], decode_sfixed32());

        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22, 0x32, 0x9A, 0xBB, 0x3C],
            decode_sfixed64()
        );
    }

    #[test]
    fn float() {
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22],
            decode_float()
        );
        assert_decode!(Ok(-29.03456), [0xC7, 0x46, 0xE8, 0xC1], decode_float());

        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x01, 0x43, 0x22, 0x32, 0x9A, 0xBB, 0x3C],
            decode_double()
        );
        assert_decode!(
            Ok(26.029345233467545),
            [0x5E, 0x09, 0x52, 0x2B, 0x83, 0x07, 0x3A, 0x40],
            decode_double()
        );
    }

    #[test]
    fn tag() {
        assert_decode!(
            Ok(Tag {
                field_num: 5,
                wire_type: 4
            }),
            [0x2C],
            decode_tag()
        );
        assert_decode!(
            Ok(Tag {
                field_num: 59,
                wire_type: 7
            }),
            [0xDF, 0x03],
            decode_tag()
        );
    }

    #[test]
    fn skip() {
        let mut tag = Tag {
            field_num: 1,
            wire_type: WIRE_TYPE_VARINT,
        };
        assert_decode!(
            Ok(()),
            [0x81, 0x80, 0x80, 0x80, 0x7F],
            skip_wire_value(&tag)
        );

        tag.wire_type = WIRE_TYPE_I64;
        assert_decode!(
            Ok(()),
            [0x12, 0x45, 0xE4, 0x90, 0x9C, 0xA1, 0xF5, 0xFF],
            skip_wire_value(&tag)
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x12, 0x45, 0xE4, 0x90, 0x9C],
            skip_wire_value(&tag)
        );

        tag.wire_type = WIRE_TYPE_I32;
        assert_decode!(Ok(()), [0x9C, 0xA1, 0xF5, 0xFF], skip_wire_value(&tag));
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0xF5, 0xFF],
            skip_wire_value(&tag)
        );

        tag.wire_type = WIRE_TYPE_LEN;
        assert_decode!(Ok(()), [0x03, 0xEE, 0xAB, 0x56], skip_wire_value(&tag));
        assert_decode!(
            Ok(()),
            [0x85, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
            skip_wire_value(&tag)
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x03, 0xAB, 0x56],
            skip_wire_value(&tag)
        );

        tag.wire_type = 3;
        assert_decode!(Err(DecodeError::Deprecation), [], skip_wire_value(&tag));
        tag.wire_type = 4;
        assert_decode!(Err(DecodeError::Deprecation), [], skip_wire_value(&tag));
        tag.wire_type = 10;
        assert_decode!(Err(DecodeError::BadWireType(10)), [], skip_wire_value(&tag));
    }

    macro_rules! assert_decode_vec {
        ($pattern:pat $(if $guard:expr)?, $arr:expr, $func:ident ($container:ident $(, $($args:tt)+)?)) => {
            let mut reader = PbDecoder::new(&$arr);
            let res = reader.$func(&mut $container, $($($args)+)?).map(|_| $container.deref());
            println!("Output = {res:?}");
            assert!(matches!(res, $pattern $(if $guard)?));
            // Check that the reader is empty only when the decoding is successful
            if res.is_ok() {
                assert_eq!(reader.remaining(), 0);
            }
        };
    }

    #[test]
    fn string() {
        let mut string = ArrayString::<4>::new();
        assert_decode_vec!(Ok(""), [0], decode_string(string));
        assert_decode_vec!(Ok("a"), [1, b'a'], decode_string(string));
        assert_decode_vec!(
            Ok("abcd"),
            [4, b'a', b'b', b'c', b'd'],
            decode_string(string)
        );

        assert_decode_vec!(Err(DecodeError::UnexpectedEof), [], decode_string(string));
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [5, b'a', b'b', b'c', b'd'],
            decode_string(string)
        );
        assert_decode_vec!(
            Err(DecodeError::Capacity),
            [5, b'a', b'b', b'c', b'd', b'e'],
            decode_string(string)
        );
        assert_decode_vec!(
            Err(DecodeError::Utf8(_)),
            [4, 0x80, 0x80, 0x80, 0x80],
            decode_string(string)
        );
    }

    #[test]
    fn bytes() {
        let mut bytes = ArrayVec::<u8, 3>::new();
        assert_decode_vec!(Ok(&[]), [0], decode_bytes(bytes));
        assert_decode_vec!(Ok(b"a"), [1, b'a'], decode_bytes(bytes));
        assert_decode_vec!(
            Ok(&[0x10, 0x20, 0x30]),
            [3, 0x10, 0x20, 0x30],
            decode_bytes(bytes)
        );

        assert_decode_vec!(Err(DecodeError::UnexpectedEof), [], decode_bytes(bytes));
        assert_decode_vec!(
            Err(DecodeError::Capacity),
            [4, 0x10, 0x20, 0x30, 0x40],
            decode_bytes(bytes)
        );
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [4, 0x10, 0x20, 0x30],
            decode_bytes(bytes)
        );
    }

    #[test]
    fn packed() {
        let mut vec = ArrayVec::<u32, 5>::new();
        assert_decode_vec!(Ok(&[]), [0], decode_packed(vec, |rd| rd.decode_varint32()));
        assert_decode_vec!(
            Ok(&[150, 5]),
            [3, 0x96, 0x01, 0x05],
            decode_packed(vec, |rd| rd.decode_varint32())
        );
        assert_decode_vec!(
            Ok(&[150, 5, 144, 512, 1]),
            [5, 0x90, 0x01, 0x80, 0x04, 0x01],
            decode_packed(vec, |rd| rd.decode_varint32())
        );
        assert_decode_vec!(
            Err(DecodeError::Capacity),
            [1, 0x01],
            decode_packed(vec, |rd| rd.decode_varint32())
        );
    }

    /// Test decoding of a map element with varint32 key and string value
    macro_rules! assert_decode_map_elem {
        ($expected:expr, $arr:expr) => {
            assert_decode!(
                $expected,
                $arr,
                decode_map_elem(
                    |opt, rd| rd.decode_varint32().map(|u| *opt = Some(u)),
                    |opt, rd| rd
                        .decode_string::<ArrayString<5>>(opt.get_or_insert_with(Default::default))
                )
            );
        };
    }

    #[test]
    fn map_elem() {
        assert_decode_map_elem!(Ok(None), [0]);
        // One key
        assert_decode_map_elem!(Ok(None), [2, 0x08, 0x01]);
        // Two keys
        assert_decode_map_elem!(Ok(None), [4, 0x08, 0x01, 0x08, 0x02]);
        // One value
        assert_decode_map_elem!(Ok(None), [3, 0x12, 1, b'a']);
        // Two values
        assert_decode_map_elem!(Ok(None), [6, 0x12, 1, b'a', 0x12, 1, b'c']);
        // Key and value
        assert_decode_map_elem!(
            Ok(Some((1, ArrayString::from("ac").unwrap()))),
            [6, 0x08, 0x01, 0x12, 2, b'a', b'c']
        );
        // Key and value, then an unknown tag which we ignore
        assert_decode_map_elem!(
            Ok(Some((1, ArrayString::from("ac").unwrap()))),
            [8, 0x08, 0x01, 0x12, 2, b'a', b'c', 0x28, 0x01]
        );
        // Value and key
        assert_decode_map_elem!(
            Ok(Some((1, ArrayString::from("ac").unwrap()))),
            [6, 0x12, 2, b'a', b'c', 0x08, 0x01]
        );
        // Overwrite value and key
        assert_decode_map_elem!(
            Ok(Some((2, ArrayString::from("x").unwrap()))),
            [11, 0x12, 2, b'a', b'c', 0x08, 0x01, 0x08, 0x02, 0x12, 1, b'x']
        );

        // Buffer too short
        assert_decode_map_elem!(Err(DecodeError::UnexpectedEof), []);
        assert_decode_map_elem!(Err(DecodeError::UnexpectedEof), [1]);
        assert_decode_map_elem!(Err(DecodeError::UnexpectedEof), [1, 0x08]);
        // Key and value, then an unknown tag with bad wire type
        assert_decode_map_elem!(
            Err(DecodeError::BadWireType(7)),
            [7, 0x08, 0x01, 0x12, 2, b'a', b'c', 0x07]
        );
    }
}
