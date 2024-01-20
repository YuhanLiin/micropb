use crate::VarInt;

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

pub struct PbWriter<W: PbWrite> {
    writer: W,
}

impl<W: PbWrite> PbWriter<W> {
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
        self.encode_varint32(((i as u32) << 1) ^ ((i as u32) >> 31))
    }

    pub fn encode_sint64(&mut self, i: i64) -> Result<(), W::Error> {
        self.encode_varint64(((i as u64) << 1) ^ ((i as u64) >> 31))
    }

    pub fn encode_bool(&mut self, b: bool) -> Result<(), W::Error> {
        self.encode_byte(b as u8)
    }
}
