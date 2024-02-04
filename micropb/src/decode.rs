use core::{
    mem::MaybeUninit,
    str::{from_utf8, Utf8Error},
};

use crate::{
    container::{PbString, PbVec},
    message::Message,
    misc::{
        maybe_uninit_slice_assume_init_ref, maybe_uninit_write_slice,
        maybe_ununit_array_assume_init,
    },
    Tag, VarInt, WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LEN, WIRE_TYPE_VARINT,
};

use never::Never;

#[derive(Debug, PartialEq)]
pub enum DecodeError<E> {
    VarIntLimit(u8),
    UnexpectedEof,
    Deprecation,
    BadWireType(u8),
    Utf8(Utf8Error),
    Capacity,
    WrongLen { expected: usize, actual: usize },
    Reader(E),
}

impl<E> From<Utf8Error> for DecodeError<E> {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

/// Implemented only for types decoded from fixed-size fields. These types must have no invalid
/// bit-patterns in their representation.
pub trait DecodeFixedSize: sealed::Sealed + Copy {}

impl DecodeFixedSize for u8 {}
impl DecodeFixedSize for u32 {}
impl DecodeFixedSize for i32 {}
impl DecodeFixedSize for u64 {}
impl DecodeFixedSize for i64 {}
impl DecodeFixedSize for f32 {}
impl DecodeFixedSize for f64 {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for u64 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

pub trait PbRead {
    type Error;

    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error>;

    fn pb_advance(&mut self, bytes: usize);
}

impl PbRead for &[u8] {
    type Error = Never;

    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error> {
        Ok(*self)
    }

    fn pb_advance(&mut self, bytes: usize) {
        *self = &self[bytes..];
    }
}

#[derive(Debug)]
pub struct PbDecoder<R: PbRead> {
    reader: R,
    idx: usize,
}

impl<R: PbRead> PbDecoder<R> {
    #[inline]
    pub fn new(reader: R) -> Self {
        Self { reader, idx: 0 }
    }

    pub fn bytes_read(&self) -> usize {
        self.idx
    }

    fn advance(&mut self, bytes: usize) {
        self.reader.pb_advance(bytes);
        self.idx += bytes;
    }

    #[inline]
    fn get_byte(&mut self) -> Result<u8, DecodeError<R::Error>> {
        let chunk = self.reader.pb_read_chunk().map_err(DecodeError::Reader)?;
        let b = chunk.first().copied().ok_or(DecodeError::UnexpectedEof)?;
        self.advance(1);
        Ok(b)
    }

    fn decode_varint<U: VarInt>(&mut self) -> Result<U, DecodeError<R::Error>> {
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

    pub fn decode_varint32(&mut self) -> Result<u32, DecodeError<R::Error>> {
        self.decode_varint::<u32>()
    }

    pub fn decode_varint64(&mut self) -> Result<u64, DecodeError<R::Error>> {
        self.decode_varint::<u64>()
    }

    pub fn decode_int64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_varint64().map(|u| u as i64)
    }

    pub fn decode_int32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_int64().map(|u| u as i32)
    }

    pub fn decode_sint32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_varint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    pub fn decode_sint64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_varint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    pub fn decode_bool(&mut self) -> Result<bool, DecodeError<R::Error>> {
        let b = self.get_byte()?;
        if b & 0x80 != 0 {
            return Err(DecodeError::VarIntLimit(1));
        }
        Ok(b != 0)
    }

    fn read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<(), DecodeError<R::Error>> {
        if buf.is_empty() {
            return Ok(());
        }

        let mut pos = 0;
        while {
            let remaining = &mut buf[pos..];
            let chunk = &self.reader.pb_read_chunk().map_err(DecodeError::Reader)?;
            if chunk.is_empty() {
                return Err(DecodeError::UnexpectedEof);
            }
            let n = chunk.len().min(remaining.len());
            maybe_uninit_write_slice(&mut remaining[..n], &chunk[..n]);
            self.advance(n);
            pos += n;
            pos < buf.len()
        } {}
        debug_assert_eq!(pos, buf.len());
        Ok(())
    }

    pub fn decode_fixed32(&mut self) -> Result<u32, DecodeError<R::Error>> {
        let mut data = [MaybeUninit::uninit(); 4];
        self.read_exact(&mut data)?;
        // SAFETY: read_exact is guaranteed to write to the whole buffer
        let data = unsafe { maybe_ununit_array_assume_init(data) };
        Ok(u32::from_le_bytes(data))
    }

    pub fn decode_fixed64(&mut self) -> Result<u64, DecodeError<R::Error>> {
        let mut data = [MaybeUninit::uninit(); 8];
        self.read_exact(&mut data)?;
        // SAFETY: read_exact is guaranteed to write to the whole buffer
        let data = unsafe { maybe_ununit_array_assume_init(data) };
        Ok(u64::from_le_bytes(data))
    }

    pub fn decode_sfixed32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_fixed32().map(|u| u as i32)
    }

    pub fn decode_sfixed64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_fixed64().map(|u| u as i64)
    }

    pub fn decode_float(&mut self) -> Result<f32, DecodeError<R::Error>> {
        self.decode_fixed32().map(f32::from_bits)
    }

    pub fn decode_double(&mut self) -> Result<f64, DecodeError<R::Error>> {
        self.decode_fixed64().map(f64::from_bits)
    }

    #[inline(always)]
    pub fn decode_tag(&mut self) -> Result<Tag, DecodeError<R::Error>> {
        self.decode_varint32().map(Tag)
    }

    pub fn decode_string<S: PbString>(
        &mut self,
        string: &mut S,
    ) -> Result<(), DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        string.pb_clear();
        string.pb_reserve(len);
        let spare_cap = string.pb_spare_cap();
        if spare_cap.len() < len {
            return Err(DecodeError::Capacity);
        }
        let target = &mut spare_cap[..len];
        self.read_exact(target)?;
        // SAFETY: read_exact guarantees that all bytes of target have been initialized
        from_utf8(unsafe { maybe_uninit_slice_assume_init_ref(target) })?;

        // SAFETY: read_exact guarantees that `len` bytes have been written into the string.
        // Also, we just checked the UTF-8 validity of the written bytes, so the string is valid.
        unsafe { string.pb_set_len(len) };
        Ok(())
    }

    pub fn decode_bytes<S: PbVec<u8>>(
        &mut self,
        bytes: &mut S,
    ) -> Result<(), DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        bytes.pb_clear();
        bytes.pb_reserve(len);
        let spare_cap = bytes.pb_spare_cap();
        if spare_cap.len() < len {
            return Err(DecodeError::Capacity);
        }
        self.read_exact(&mut spare_cap[..len])?;
        // SAFETY: read_exact guarantees that `len` bytes have been written into the buffer
        unsafe { bytes.pb_set_len(len) };
        Ok(())
    }

    fn decode_len_record<
        T,
        F: FnOnce(usize, usize, &mut Self) -> Result<T, DecodeError<R::Error>>,
    >(
        &mut self,
        decoder: F,
    ) -> Result<T, DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        let before = self.bytes_read();
        let val = decoder(len, before, self)?;
        let actual_len = self.bytes_read() - before;
        if actual_len != len {
            Err(DecodeError::WrongLen {
                expected: len,
                actual: actual_len,
            })
        } else {
            Ok(val)
        }
    }

    pub fn decode_packed<
        T: Copy,
        S: PbVec<T>,
        F: Fn(&mut Self) -> Result<T, DecodeError<R::Error>>,
    >(
        &mut self,
        vec: &mut S,
        decoder: F,
    ) -> Result<(), DecodeError<R::Error>> {
        self.decode_len_record(|len, before, this| {
            while this.bytes_read() - before < len {
                let val = decoder(this)?;
                vec.pb_push(val).map_err(|_| DecodeError::Capacity)?;
            }
            Ok(())
        })
    }

    #[cfg(target_endian = "little")]
    pub fn decode_packed_fixed<T: DecodeFixedSize, S: PbVec<T>>(
        &mut self,
        vec: &mut S,
    ) -> Result<(), DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        let elem_size = core::mem::size_of::<T>();
        let modulo = len % elem_size;
        // Length must be a multiple of elem_size
        if modulo > 0 {
            return Err(DecodeError::WrongLen {
                expected: len,
                // Previous multiple of elem_size
                actual: len - modulo,
            });
        }
        let elem_num = len / elem_size;
        vec.pb_reserve(elem_num);
        let spare_cap = vec.pb_spare_cap();
        if spare_cap.len() < elem_num {
            return Err(DecodeError::Capacity);
        }
        // SAFETY: Converting slice into uninitialized bytes is always valid. Moreover, we know
        // that `spare_cap` has equal or more than `elem_num` values, so it's size in bytes can't
        // be less than `len`, because `len` is equal to `elem_num * size_of<T>()`.
        let spare_bytes = unsafe {
            core::slice::from_raw_parts_mut(spare_cap.as_mut_ptr() as *mut MaybeUninit<u8>, len)
        };
        self.read_exact(spare_bytes)?;
        // SAFETY: We just wrote elem_num of elements into the spare space, so we can
        // increase the length by that much
        unsafe { vec.pb_set_len(vec.len() + elem_num) };
        Ok(())
    }

    pub fn decode_map_elem<
        K: Default,
        V: Default,
        UK: Fn(&mut K, &mut Self) -> Result<(), DecodeError<R::Error>>,
        UV: Fn(&mut V, &mut Self) -> Result<(), DecodeError<R::Error>>,
    >(
        &mut self,
        key_update: UK,
        val_update: UV,
    ) -> Result<Option<(K, V)>, DecodeError<R::Error>> {
        let mut key = None;
        let mut val = None;
        self.decode_len_record(|len, before, this| {
            while this.bytes_read() - before < len {
                let tag = this.decode_tag()?;
                match tag.field_num() {
                    1 => this.decode_optional(&mut key, &key_update)?,
                    2 => this.decode_optional(&mut val, &val_update)?,
                    _ => this.skip_wire_value(tag.wire_type())?,
                }
            }
            Ok(())
        })?;

        if let (Some(key), Some(val)) = (key, val) {
            Ok(Some((key, val)))
        } else {
            Ok(None)
        }
    }

    pub fn decode_optional<
        T: Default,
        U: Fn(&mut T, &mut Self) -> Result<(), DecodeError<R::Error>>,
    >(
        &mut self,
        optional: &mut Option<T>,
        update: U,
    ) -> Result<(), DecodeError<R::Error>> {
        let val = optional.get_or_insert_with(T::default);
        update(val, self)
    }

    pub fn decode_repeated_elem<
        T,
        S: PbVec<T>,
        F: Fn(&mut Self) -> Result<T, DecodeError<R::Error>>,
    >(
        &mut self,
        repeated: &mut S,
        decoder: F,
    ) -> Result<(), DecodeError<R::Error>> {
        let val = decoder(self)?;
        repeated.pb_push(val).map_err(|_| DecodeError::Capacity)
    }

    fn skip_varint(&mut self) -> Result<(), DecodeError<R::Error>> {
        for _ in 0..u64::BYTES {
            let b = self.get_byte()?;
            if b & 0x80 == 0 {
                return Ok(());
            }
        }
        Err(DecodeError::VarIntLimit(u64::BYTES))
    }

    fn skip_bytes(&mut self, bytes: usize) -> Result<(), DecodeError<R::Error>> {
        let mut total = 0;
        while total < bytes {
            let chunk = self.reader.pb_read_chunk().map_err(DecodeError::Reader)?;
            if chunk.is_empty() {
                return Err(DecodeError::UnexpectedEof);
            }
            let n = chunk.len().min(bytes - total);
            self.advance(n);
            total += n;
        }
        debug_assert_eq!(total, bytes);
        Ok(())
    }

    pub fn skip_wire_value(&mut self, wire_type: u8) -> Result<(), DecodeError<R::Error>> {
        match wire_type {
            WIRE_TYPE_VARINT => self.skip_varint()?,
            WIRE_TYPE_I64 => self.skip_bytes(8)?,
            WIRE_TYPE_LEN => {
                let len = self.decode_varint32()? as usize;
                self.skip_bytes(len)?;
            }
            3 | 4 => return Err(DecodeError::Deprecation),
            WIRE_TYPE_I32 => self.skip_bytes(4)?,
            w => return Err(DecodeError::BadWireType(w)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::{ArrayString, ArrayVec};

    use super::*;

    struct Multichunk<'a>(&'a [u8]);

    impl<'a> Multichunk<'a> {
        fn len(&self) -> usize {
            self.0.len()
        }

        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<'a> PbRead for Multichunk<'a> {
        type Error = Never;

        fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error> {
            let n = if self.len() % 2 == 0 { 2 } else { 1 };
            Ok(&self.0[..n.min(self.len())])
        }

        fn pb_advance(&mut self, bytes: usize) {
            self.0.pb_advance(bytes)
        }
    }

    macro_rules! assert_decode {
        (@testcase $expected:expr, $reader:expr, $($op:tt)+) => {
            let mut decoder = PbDecoder::new($reader);
            let total = decoder.reader.len();
            let res = decoder.$($op)+;
            println!("{} output = {res:?}", stringify!($reader));
            assert_eq!($expected, res);
            // Check that the reader is empty only when the decoding is successful
            if res.is_ok() {
                assert!(decoder.reader.is_empty());
            }
            // Check that # of bytes read is correct
            assert_eq!(decoder.bytes_read(), total - decoder.reader.len());
        };

        ($expected:expr, $arr:expr, $($op:tt)+) => {
            assert_decode!(@testcase $expected, $arr.as_slice(), $($op)+);
            assert_decode!(@testcase $expected, Multichunk($arr.as_slice()), $($op)+);
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
        // Last byte of input is partially truncated in the output
        assert_decode!(
            Ok(0b11110000000000000000000000000001),
            [0x81, 0x80, 0x80, 0x80, 0x7F],
            decode_varint32()
        );
        assert_decode!(
            Ok(u32::MAX),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
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
        assert_decode!(
            Ok(u64::MAX),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
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
        assert_decode!(
            Ok(i32::MIN),
            [0x80, 0x80, 0x80, 0x80, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            decode_int32()
        );
        assert_decode!(
            Ok(i64::MIN),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
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
    fn skip() {
        assert_decode!(
            Ok(()),
            [0x81, 0x80, 0x80, 0x80, 0x7F],
            skip_wire_value(WIRE_TYPE_VARINT)
        );

        assert_decode!(
            Ok(()),
            [0x12, 0x45, 0xE4, 0x90, 0x9C, 0xA1, 0xF5, 0xFF],
            skip_wire_value(WIRE_TYPE_I64)
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x12, 0x45, 0xE4, 0x90, 0x9C],
            skip_wire_value(WIRE_TYPE_I64)
        );

        assert_decode!(
            Ok(()),
            [0x9C, 0xA1, 0xF5, 0xFF],
            skip_wire_value(WIRE_TYPE_I32)
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0xF5, 0xFF],
            skip_wire_value(WIRE_TYPE_I32)
        );

        assert_decode!(
            Ok(()),
            [0x03, 0xEE, 0xAB, 0x56],
            skip_wire_value(WIRE_TYPE_LEN)
        );
        assert_decode!(
            Ok(()),
            [0x85, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
            skip_wire_value(WIRE_TYPE_LEN)
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x03, 0xAB, 0x56],
            skip_wire_value(WIRE_TYPE_LEN)
        );

        assert_decode!(Err(DecodeError::Deprecation), [], skip_wire_value(3));
        assert_decode!(Err(DecodeError::Deprecation), [], skip_wire_value(4));
        assert_decode!(Err(DecodeError::BadWireType(10)), [], skip_wire_value(10));
    }

    macro_rules! assert_decode_vec {
        (@testcase $pattern:pat $(if $guard:expr)?, $reader:expr, $func:ident ($container:ident $(, $($args:tt)+)?)) => {
            let mut decoder = PbDecoder::new($reader);
            let total = decoder.reader.len();
            let res = decoder.$func(&mut $container, $($($args)+)?).map(|_| $container.deref());
            println!("{} output = {res:?}", stringify!($reader));
            assert!(matches!(res, $pattern $(if $guard)?));
            // Check that the decoder is empty only when the decoding is successful
            if res.is_ok() {
                assert!(decoder.reader.is_empty());
            }
            // Check that # of bytes read is correct
            assert_eq!(decoder.bytes_read(), total - decoder.reader.len());
        };

        ($pattern:pat $(if $guard:expr)?, $arr:expr, $func:ident ($container:ident $(, $($args:tt)+)?)) => {
            assert_decode_vec!(@testcase $pattern $($guard)?, $arr.as_slice(), $func($container $(, $($args)+)?));
            assert_decode_vec!(@testcase $pattern $($guard)?, Multichunk($arr.as_slice()), $func($container $(, $($args)+)?));
        };

        ($pattern:pat $(if $guard:expr)?, $arr:expr, $func:ident ($container1:ident | $container2:ident $(, $($args:tt)+)?)) => {
            assert_decode_vec!(@testcase $pattern $($guard)?, $arr.as_slice(), $func($container1 $(, $($args)+)?));
            assert_decode_vec!(@testcase $pattern $($guard)?, Multichunk($arr.as_slice()), $func($container2 $(, $($args)+)?));
        };
    }

    macro_rules! container_test {
        ($test:ident, $name:ident, $container:ty, $fixed_cap:literal) => {
            #[test]
            fn $name() {
                $test::<$container>($fixed_cap);
            }
        };
    }

    fn string<S: PbString>(fixed_cap: bool) {
        let mut string = S::default();
        assert_decode_vec!(Ok(""), [0], decode_string(string));
        assert_decode_vec!(Ok("a"), [1, b'a'], decode_string(string));
        assert_decode_vec!(
            Ok("abcd"),
            [4, b'a', b'b', b'c', b'd'],
            decode_string(string)
        );
        assert_decode_vec!(Ok("Зд"), [4, 208, 151, 208, 180], decode_string(string));

        assert_decode_vec!(Err(DecodeError::UnexpectedEof), [], decode_string(string));
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [4, b'b', b'c', b'd'],
            decode_string(string)
        );
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [5, b'a', b'b', b'c', b'd', b'e'],
                decode_string(string)
            );
        }
        assert_decode_vec!(
            Err(DecodeError::Utf8(_)),
            [4, 0x80, 0x80, 0x80, 0x80],
            decode_string(string)
        );
    }

    container_test!(string, string_arrayvec, ArrayString::<4>, true);
    container_test!(string, string_heapless, heapless::String::<4>, true);
    container_test!(string, string_alloc, String, false);

    fn bytes<S: PbVec<u8>>(fixed_cap: bool) {
        let mut bytes = S::default();
        assert_decode_vec!(Ok(&[]), [0], decode_bytes(bytes));
        assert_decode_vec!(Ok(b"a"), [1, b'a'], decode_bytes(bytes));
        assert_decode_vec!(
            Ok(&[0x10, 0x20, 0x30]),
            [3, 0x10, 0x20, 0x30],
            decode_bytes(bytes)
        );

        assert_decode_vec!(Err(DecodeError::UnexpectedEof), [], decode_bytes(bytes));
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [4, 0x10, 0x20, 0x30, 0x40],
                decode_bytes(bytes)
            );
        }
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [3, 0x20, 0x30],
            decode_bytes(bytes)
        );
    }

    container_test!(bytes, bytes_arrayvec, ArrayVec::<_, 3>, true);
    container_test!(bytes, bytes_heapless, heapless::Vec::<_, 3>, true);
    container_test!(bytes, bytes_alloc, Vec<_>, false);

    fn packed<S: PbVec<u32>>(fixed_cap: bool) {
        let mut vec1 = S::default();
        let mut vec2 = S::default();
        assert_decode_vec!(
            Ok(&[]),
            [0],
            decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
        );
        assert_decode_vec!(
            Err(DecodeError::WrongLen {
                expected: 1,
                actual: 2
            }),
            [1, 0x90, 0x01],
            decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
        );
        // Reset vecs after an error
        vec1.pb_clear();
        vec2.pb_clear();

        assert_decode_vec!(
            Ok(&[150, 5]),
            [3, 0x96, 0x01, 0x05],
            decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
        );
        assert_decode_vec!(
            Ok(&[150, 5, 144, 512, 1]),
            [5, 0x90, 0x01, 0x80, 0x04, 0x01],
            decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
        );
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [1, 0x01],
                decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
            );
        }
    }

    container_test!(packed, packed_arrayvec, ArrayVec::<_, 5>, true);
    container_test!(packed, packed_heapless, heapless::Vec::<_, 5>, true);
    container_test!(packed, packed_alloc, Vec<_>, false);

    #[cfg(target_endian = "little")]
    fn packed_fixed<S: PbVec<u32>>(fixed_cap: bool) {
        let mut vec1 = S::default();
        let mut vec2 = S::default();
        assert_decode_vec!(Ok(&[]), [0], decode_packed_fixed(vec1 | vec2));
        assert_decode_vec!(
            Ok(&[0x04030201]),
            [4, 0x01, 0x02, 0x03, 0x04],
            decode_packed_fixed(vec1 | vec2)
        );
        assert_decode_vec!(
            Ok(&[0x04030201, 0x0D0C0B0A, 0x44332211]),
            [8, 0x0A, 0x0B, 0x0C, 0x0D, 0x11, 0x22, 0x33, 0x44],
            decode_packed_fixed(vec1 | vec2)
        );
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [4, 0x01, 0x02, 0x03, 0x04],
                decode_packed_fixed(vec1 | vec2)
            );
        }
        assert_decode_vec!(
            Err(DecodeError::WrongLen {
                expected: 1,
                actual: 0
            }),
            [1, 0x01],
            decode_packed_fixed(vec1 | vec2)
        );
    }

    #[cfg(target_endian = "little")]
    container_test!(packed_fixed, pf_arrayvec, ArrayVec::<_, 3>, true);
    #[cfg(target_endian = "little")]
    container_test!(packed_fixed, pf_heapless, heapless::Vec::<_, 3>, true);
    #[cfg(target_endian = "little")]
    container_test!(packed_fixed, pf_alloc, Vec<_>, false);

    /// Test decoding of a map element with varint32 key and string value
    macro_rules! assert_decode_map_elem {
        ($expected:expr, $arr:expr) => {
            assert_decode!(
                $expected,
                $arr,
                decode_map_elem(
                    |v, rd| rd.decode_varint32().map(|u| *v = u),
                    |v, rd| rd.decode_string::<ArrayString<5>>(v)
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

    #[test]
    fn map_elem_string_key() {
        assert_decode!(
            Ok(Some((
                ArrayString::from("ac").unwrap(),
                ArrayString::from("bd").unwrap()
            ))),
            [8, 0x0A, 2, b'a', b'c', 0x12, 2, b'b', b'd'],
            decode_map_elem(
                |v, rd| rd.decode_string::<ArrayString<5>>(v),
                |v, rd| rd.decode_string::<ArrayString<5>>(v)
            )
        );
    }
}
