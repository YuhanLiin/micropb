use crate::{size::sizeof_packed_fixed, ImplicitPresence, Tag, VarInt};

pub trait PbWrite {
    type Error;

    fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

impl<W: PbWrite> PbWrite for &mut W {
    type Error = W::Error;

    #[inline]
    fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        (*self).pb_write(data)
    }
}

pub struct PbEncoder<W: PbWrite> {
    writer: W,
}

impl<W: PbWrite> PbEncoder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    #[inline]
    fn encode_byte(&mut self, b: u8) -> Result<(), W::Error> {
        self.writer.pb_write(&[b])
    }

    fn encode_varint<U: VarInt>(&mut self, mut varint: U) -> Result<(), W::Error> {
        if varint <= From::from(0x7F) {
            return self.encode_byte(varint.as_());
        }

        while {
            let mut b = varint.as_() & 0x7F;
            varint = varint >> 7;
            let zero = varint.is_zero();
            if !zero {
                b |= 0x80;
            }
            self.encode_byte(b)?;
            !zero
        } {}

        Ok(())
    }

    pub fn encode_varint32(&mut self, u: u32) -> Result<(), W::Error> {
        self.encode_varint(u)
    }

    pub fn encode_varint64(&mut self, u: u64) -> Result<(), W::Error> {
        self.encode_varint(u)
    }

    pub fn encode_int32(&mut self, i: i32) -> Result<(), W::Error> {
        if i >= 0 {
            // Can avoid 64-bit operations if number if non-negative
            self.encode_varint32(i as u32)
        } else {
            self.encode_varint64(i as u64)
        }
    }

    pub fn encode_int64(&mut self, i: i64) -> Result<(), W::Error> {
        self.encode_varint64(i as u64)
    }

    pub fn encode_sint32(&mut self, i: i32) -> Result<(), W::Error> {
        self.encode_varint32(((i << 1) ^ (i >> 31)) as u32)
    }

    pub fn encode_sint64(&mut self, i: i64) -> Result<(), W::Error> {
        self.encode_varint64(((i << 1) ^ (i >> 63)) as u64)
    }

    pub fn encode_bool(&mut self, b: bool) -> Result<(), W::Error> {
        self.encode_byte(b as u8)
    }

    pub fn encode_fixed32(&mut self, u: u32) -> Result<(), W::Error> {
        self.writer.pb_write(&u.to_le_bytes())
    }

    pub fn encode_fixed64(&mut self, u: u64) -> Result<(), W::Error> {
        self.writer.pb_write(&u.to_le_bytes())
    }

    pub fn encode_sfixed32(&mut self, i: i32) -> Result<(), W::Error> {
        self.encode_fixed32(i as u32)
    }

    pub fn encode_sfixed64(&mut self, i: i64) -> Result<(), W::Error> {
        self.encode_fixed64(i as u64)
    }

    pub fn encode_float(&mut self, f: f32) -> Result<(), W::Error> {
        self.encode_fixed32(f.to_bits())
    }

    pub fn encode_double(&mut self, f: f64) -> Result<(), W::Error> {
        self.encode_fixed64(f.to_bits())
    }

    #[inline]
    pub fn encode_tag(&mut self, tag: &Tag) -> Result<(), W::Error> {
        self.encode_varint32(tag.varint())
    }

    pub fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.encode_varint32(bytes.len() as u32)?;
        self.writer.pb_write(bytes)
    }

    pub fn encode_string(&mut self, string: &str) -> Result<(), W::Error> {
        self.encode_bytes(string.as_bytes())
    }

    pub fn encode_packed_fixed<T: Copy>(&mut self, elems: &[T]) -> Result<(), W::Error> {
        // O(1) operation that gets total size of slice
        let len = sizeof_packed_fixed(elems);
        let bytes = unsafe { core::slice::from_raw_parts(elems.as_ptr() as *const u8, len) };
        self.encode_bytes(bytes)
    }

    pub fn encode_packed<T: Copy, F: FnMut(&mut Self, T) -> Result<(), W::Error>>(
        &mut self,
        elems: &[T],
        len: usize,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        self.encode_varint32(len as u32)?;
        for &e in elems {
            encoder(self, e)?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn encode_map_elem<
        K,
        V,
        EK: FnMut(&mut Self, &K) -> Result<(), W::Error>,
        EV: FnMut(&mut Self, &V) -> Result<(), W::Error>,
    >(
        &mut self,
        len: usize,
        key: &K,
        key_wtype: u8,
        val: &V,
        val_wtype: u8,
        mut key_encoder: EK,
        mut val_encoder: EV,
    ) -> Result<(), W::Error> {
        self.encode_varint32(len as u32)?;
        let key_tag = Tag::from_parts(1, key_wtype);
        let val_tag = Tag::from_parts(2, val_wtype);

        self.encode_tag(&key_tag)?;
        key_encoder(self, key)?;
        self.encode_tag(&val_tag)?;
        val_encoder(self, val)?;
        Ok(())
    }

    // Only for non-packed fields
    pub fn encode_repeated_with_tag<T, F: FnMut(&mut Self, T) -> Result<(), W::Error>>(
        &mut self,
        tag: &Tag,
        elems: impl Iterator<Item = T>,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        for e in elems {
            self.encode_tag(tag)?;
            encoder(self, e)?;
        }
        Ok(())
    }

    pub fn encode_with_tag<T: ImplicitPresence, F: FnMut(&mut Self, &T) -> Result<(), W::Error>>(
        &mut self,
        tag: &Tag,
        val: &T,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        // Implicit field presence, only encode if value is non-default
        if !val.pb_is_present() {
            self.encode_tag(tag)?;
            encoder(self, val)?;
        }
        Ok(())
    }

    pub fn encode_optional_with_tag<T, F: FnMut(&mut Self, &T) -> Result<(), W::Error>>(
        &mut self,
        tag: &Tag,
        val: &Option<T>,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        if let Some(val) = val {
            self.encode_tag(tag)?;
            encoder(self, val)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;

    use super::*;

    macro_rules! assert_encode {
        ($expected:expr, $encoder:ident.$($op:tt)+) => {
            $encoder.$($op)+.unwrap();
            assert_eq!($expected, $encoder.writer.as_slice());
            $encoder.writer.clear();
        }
    }

    #[test]
    fn varint32() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x01], encoder.encode_varint32(1));
        assert_encode!(&[0x00], encoder.encode_varint32(0));
        assert_encode!(&[0x96, 0x01], encoder.encode_varint32(150));
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_varint32(u32::MAX)
        );
        assert_encode!(
            &[0x95, 0x87, 0x14],
            encoder.encode_varint32(0b1010000001110010101)
        );
    }

    #[test]
    fn varint64() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x01], encoder.encode_varint64(1));
        assert_encode!(&[0x00], encoder.encode_varint64(0));
        assert_encode!(&[0x96, 0x01], encoder.encode_varint64(150));
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_varint64(u32::MAX as u64)
        );
        assert_encode!(
            &[0x95, 0x87, 0x14],
            encoder.encode_varint64(0b1010000001110010101)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_varint64(u64::MAX)
        );
    }

    #[test]
    fn int() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x01], encoder.encode_int32(1));
        assert_encode!(&[0x96, 0x01], encoder.encode_int32(150));
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_int32(-2)
        );
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_int64(-2)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_int32(-1)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_int64(-1)
        );
        assert_encode!(
            &[0x80, 0x80, 0x80, 0x80, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_int32(i32::MIN)
        );
        assert_encode!(
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            encoder.encode_int64(i64::MIN)
        );
    }

    #[test]
    fn sint32() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x00], encoder.encode_sint32(0));
        assert_encode!(&[0x01], encoder.encode_sint32(-1));
        assert_encode!(&[0x02], encoder.encode_sint32(1));
        assert_encode!(&[0x03], encoder.encode_sint32(-2));
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_sint32(0x7FFFFFFF)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_sint32(-0x80000000)
        );
    }

    #[test]
    fn sint64() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x00], encoder.encode_sint64(0));
        assert_encode!(&[0x01], encoder.encode_sint64(-1));
        assert_encode!(&[0x02], encoder.encode_sint64(1));
        assert_encode!(&[0x03], encoder.encode_sint64(-2));
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_sint64(0x7FFFFFFF)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encoder.encode_sint64(-0x80000000)
        );
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_sint64(0x7FFFFFFFFFFFFFFF)
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encoder.encode_sint64(-0x8000000000000000)
        );
    }

    #[test]
    fn bool() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x01], encoder.encode_bool(true));
        assert_encode!(&[0x00], encoder.encode_bool(false));
    }

    #[test]
    fn fixed() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0x00; 4], encoder.encode_fixed32(0));
        assert_encode!(
            &[0x12, 0x32, 0x98, 0xF4],
            encoder.encode_fixed32(0xF4983212)
        );
        assert_encode!(&[0x00; 8], encoder.encode_fixed64(0));
        assert_encode!(
            &[0x12, 0x32, 0x98, 0xF4, 0x3B, 0xAA, 0x50, 0x99],
            encoder.encode_fixed64(0x9950AA3BF4983212)
        );
        assert_encode!(
            &[0x12, 0x32, 0x98, 0xF4],
            encoder.encode_sfixed32(-0x0B67CDEE)
        );
    }

    #[test]
    fn float() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
        assert_encode!(&[0xC7, 0x46, 0xE8, 0xC1], encoder.encode_float(-29.03456));
        assert_encode!(
            &[0x5E, 0x09, 0x52, 0x2B, 0x83, 0x07, 0x3A, 0x40],
            encoder.encode_double(26.029345233467545)
        );
    }
}
