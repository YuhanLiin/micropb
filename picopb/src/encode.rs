use num_traits::Zero;

use crate::{size::sizeof_packed_fixed, Tag, VarInt};

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

pub trait IsDefault {
    fn pb_is_default(&self) -> bool;
}

impl<T: Zero> IsDefault for T {
    fn pb_is_default(&self) -> bool {
        self.is_zero()
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

    pub fn encode_with_tag<T: IsDefault, F: FnMut(&mut Self, &T) -> Result<(), W::Error>>(
        &mut self,
        tag: &Tag,
        val: &T,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        // Implicit field presence, only encode if value is non-default
        if !val.pb_is_default() {
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
