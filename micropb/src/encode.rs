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

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
pub struct PbWriter<W>(pub W);

#[cfg(feature = "std")]
impl<W: std::io::Write> PbWrite for PbWriter<W> {
    type Error = std::io::Error;

    fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(data)
    }
}

pub struct PbEncoder<W: PbWrite> {
    writer: W,
    bytes_written: usize,
}

impl<W: PbWrite> PbEncoder<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            bytes_written: 0,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.writer.pb_write(bytes)?;
        self.bytes_written += bytes.len();
        Ok(())
    }

    #[inline]
    fn encode_byte(&mut self, b: u8) -> Result<(), W::Error> {
        self.write(&[b])
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
        self.write(&u.to_le_bytes())
    }

    pub fn encode_fixed64(&mut self, u: u64) -> Result<(), W::Error> {
        self.write(&u.to_le_bytes())
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
    pub fn encode_tag(&mut self, tag: Tag) -> Result<(), W::Error> {
        self.encode_varint32(tag.varint())
    }

    pub fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.encode_varint32(bytes.len() as u32)?;
        self.write(bytes)
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
        len: usize,
        elems: &[T],
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
        K: ?Sized,
        V: ?Sized,
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

        self.encode_tag(key_tag)?;
        key_encoder(self, key)?;
        self.encode_tag(val_tag)?;
        val_encoder(self, val)?;
        Ok(())
    }

    // Only for non-packed fields
    pub fn encode_repeated_with_tag<T, F: FnMut(&mut Self, T) -> Result<(), W::Error>>(
        &mut self,
        tag: Tag,
        elems: impl Iterator<Item = T>,
        mut encoder: F,
    ) -> Result<(), W::Error> {
        for e in elems {
            self.encode_tag(tag)?;
            encoder(self, e)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;

    use crate::{size::*, WIRE_TYPE_LEN, WIRE_TYPE_VARINT};

    use super::*;

    macro_rules! assert_encode {
        ($expected:expr, $encode:ident( $($arg:expr),+ ), $sizeof:ident) => {
            let mut encoder = PbEncoder::new(ArrayVec::<_, 20>::new());
            encoder.$encode($($arg),+).unwrap();
            assert_eq!($expected, encoder.writer.as_slice());
            assert_eq!(encoder.bytes_written, encoder.writer.len());
            assert_eq!(encoder.writer.len(), $sizeof($($arg),+));
        }
    }

    macro_rules! assert_encode_nosize {
        ($expected:expr, $encode:ident( $($arg:expr),+ )) => {
            let mut encoder = PbEncoder::new(ArrayVec::<_, 20>::new());
            encoder.$encode($($arg),+).unwrap();
            assert_eq!(encoder.bytes_written, encoder.writer.len());
            assert_eq!($expected, encoder.writer.as_slice());
        }
    }

    #[test]
    fn varint32() {
        assert_encode!(&[0x01], encode_varint32(1), sizeof_varint32);
        assert_encode!(&[0x00], encode_varint32(0), sizeof_varint32);
        assert_encode!(&[0x96, 0x01], encode_varint32(150), sizeof_varint32);
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_varint32(u32::MAX),
            sizeof_varint32
        );
        assert_encode!(
            &[0x95, 0x87, 0x14],
            encode_varint32(0b1010000001110010101),
            sizeof_varint32
        );
    }

    #[test]
    fn varint64() {
        assert_encode!(&[0x01], encode_varint64(1), sizeof_varint64);
        assert_encode!(&[0x00], encode_varint64(0), sizeof_varint64);
        assert_encode!(&[0x96, 0x01], encode_varint64(150), sizeof_varint64);
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_varint64(u32::MAX as u64),
            sizeof_varint64
        );
        assert_encode!(
            &[0x95, 0x87, 0x14],
            encode_varint64(0b1010000001110010101),
            sizeof_varint64
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_varint64(u64::MAX),
            sizeof_varint64
        );
    }

    #[test]
    fn int() {
        assert_encode!(&[0x01], encode_int32(1), sizeof_int32);
        assert_encode!(&[0x96, 0x01], encode_int32(150), sizeof_int32);
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_int32(-2),
            sizeof_int32
        );
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_int64(-2),
            sizeof_int32
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_int32(-1),
            sizeof_int32
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_int64(-1),
            sizeof_int64
        );
        assert_encode!(
            &[0x80, 0x80, 0x80, 0x80, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_int32(i32::MIN),
            sizeof_int32
        );
        assert_encode!(
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            encode_int64(i64::MIN),
            sizeof_int64
        );
    }

    #[test]
    fn sint32() {
        assert_encode!(&[0x00], encode_sint32(0), sizeof_sint32);
        assert_encode!(&[0x01], encode_sint32(-1), sizeof_sint32);
        assert_encode!(&[0x02], encode_sint32(1), sizeof_sint32);
        assert_encode!(&[0x03], encode_sint32(-2), sizeof_sint32);
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_sint32(0x7FFFFFFF),
            sizeof_sint32
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_sint32(-0x80000000),
            sizeof_sint32
        );
    }

    #[test]
    fn sint64() {
        assert_encode!(&[0x00], encode_sint64(0), sizeof_sint64);
        assert_encode!(&[0x01], encode_sint64(-1), sizeof_sint64);
        assert_encode!(&[0x02], encode_sint64(1), sizeof_sint64);
        assert_encode!(&[0x03], encode_sint64(-2), sizeof_sint64);
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_sint64(0x7FFFFFFF),
            sizeof_sint64
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
            encode_sint64(-0x80000000),
            sizeof_sint64
        );
        assert_encode!(
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_sint64(0x7FFFFFFFFFFFFFFF),
            sizeof_sint64
        );
        assert_encode!(
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            encode_sint64(-0x8000000000000000),
            sizeof_sint64
        );
    }

    #[test]
    fn bool() {
        assert_encode_nosize!(&[0x01], encode_bool(true));
        assert_encode_nosize!(&[0x00], encode_bool(false));
    }

    #[test]
    fn fixed() {
        assert_encode_nosize!(&[0x00; 4], encode_fixed32(0));
        assert_encode_nosize!(&[0x12, 0x32, 0x98, 0xF4], encode_fixed32(0xF4983212));
        assert_encode_nosize!(&[0x00; 8], encode_fixed64(0));
        assert_encode_nosize!(
            &[0x12, 0x32, 0x98, 0xF4, 0x3B, 0xAA, 0x50, 0x99],
            encode_fixed64(0x9950AA3BF4983212)
        );
        assert_encode_nosize!(&[0x12, 0x32, 0x98, 0xF4], encode_sfixed32(-0x0B67CDEE));
    }

    #[test]
    fn float() {
        assert_encode_nosize!(&[0xC7, 0x46, 0xE8, 0xC1], encode_float(-29.03456));
        assert_encode_nosize!(
            &[0x5E, 0x09, 0x52, 0x2B, 0x83, 0x07, 0x3A, 0x40],
            encode_double(26.029345233467545)
        );
    }

    #[test]
    fn bytes_string() {
        assert_encode_nosize!(&[0], encode_bytes(b""));
        assert_encode_nosize!(&[5, b'a', b'b', b'c', b'd', b'e'], encode_bytes(b"abcde"));

        assert_encode_nosize!(&[0], encode_string(""));
        assert_encode_nosize!(&[3, b'a', b'b', b'c'], encode_string("abc"));
        assert_encode_nosize!(&[4, 208, 151, 208, 180], encode_string("Зд"));
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn packed_fixed() {
        assert_encode_nosize!([0], encode_packed_fixed(&[0u32; 0]));
        assert_encode_nosize!(
            [4, 0x1, 0x0, 0x0, 0x1],
            encode_packed_fixed(&[true, false, false, true])
        );
        assert_encode_nosize!(
            [8, 0x1, 0x0, 0x0, 0x0, 0x6, 0x0, 0x0, 0x0],
            encode_packed_fixed(&[1u32, 6u32])
        );
    }

    #[test]
    fn packed() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 20>::new());
        let len = sizeof_packed(&[0u32; 0], sizeof_varint32);
        encoder
            .encode_packed(len, &[0u32; 0], PbEncoder::encode_varint32)
            .unwrap();
        assert_eq!([0], encoder.writer.as_slice());
        assert_eq!(encoder.bytes_written, encoder.writer.len());
        assert_eq!(1, sizeof_len_record(len));

        let mut encoder = PbEncoder::new(ArrayVec::<_, 20>::new());
        let len = sizeof_packed(&[1, 156], sizeof_varint32);
        encoder
            .encode_packed(len, &[1, 156], PbEncoder::encode_varint32)
            .unwrap();
        assert_eq!([3, 0x01, 0x9C, 0x01], encoder.writer.as_slice());
        assert_eq!(encoder.bytes_written, encoder.writer.len());
        assert_eq!(4, sizeof_len_record(len));
    }

    macro_rules! assert_encode_map_elem {
        ($expected:expr, $key:expr, $val:expr) => {
            let mut encoder = PbEncoder::new(ArrayVec::<_, 20>::new());
            let len = sizeof_map_elem(
                $key,
                $val,
                |v| sizeof_varint32(*v),
                |s| sizeof_len_record(s.len()),
            );
            encoder
                .encode_map_elem(
                    len,
                    $key,
                    WIRE_TYPE_VARINT,
                    $val,
                    WIRE_TYPE_LEN,
                    |wr, v| wr.encode_varint32(*v),
                    |wr, s| wr.encode_string(s),
                )
                .unwrap();
            assert_eq!($expected, encoder.writer.as_slice());
            assert_eq!(encoder.bytes_written, encoder.writer.len());
            assert_eq!($expected.len(), sizeof_len_record(len));
        };
    }

    #[test]
    fn map_elem() {
        assert_encode_map_elem!([6, 0x08, 0x01, 0x12, 2, b'a', b'c'], &1, "ac");
        assert_encode_map_elem!([5, 0x08, 0x02, 0x12, 1, b'x'], &2, "x");
        assert_encode_map_elem!(
            [8, 0x08, 0x0B, 0x12, 4, b'c', b'd', b'e', b'f'],
            &11,
            "cdef"
        );
        assert_encode_map_elem!([5, 0x08, 0x96, 0x01, 0x12, 0], &150, "");
    }

    macro_rules! assert_encode_with_tag {
        ($expected:expr, $encode:ident($($args:tt)+), $sizeof:ident) => {
            let mut encoder = PbEncoder::new(ArrayVec::<_, 10>::new());
            encoder
                .$encode($($args)+, |wr, v| wr.encode_varint32(*v))
                .unwrap();
            assert_eq!($expected, encoder.writer.as_slice());
            assert_eq!(encoder.bytes_written, encoder.writer.len());
            assert_eq!(
                $expected.len(),
                $sizeof($($args)+, |v| sizeof_varint32(*v))
            );
        };
    }

    #[test]
    fn repeated_with_tag() {
        let tag = Tag::from_parts(1, WIRE_TYPE_VARINT);
        assert_encode_with_tag!(
            b"",
            encode_repeated_with_tag(tag, core::iter::empty::<&u32>()),
            sizeof_repeated_with_tag
        );
        assert_encode_with_tag!(
            [0x08, 0x01, 0x08, 0x96, 0x01],
            encode_repeated_with_tag(tag, [1, 150].iter()),
            sizeof_repeated_with_tag
        );
    }

    #[test]
    fn repeated_map_elems() {
        let mut encoder = PbEncoder::new(ArrayVec::<_, 30>::new());
        let mut len_cache = ArrayVec::<_, 3>::new();

        let tag = Tag::from_parts(8, WIRE_TYPE_LEN);
        let elems = &[("x", true), ("xy", false), ("xyz", true)];
        let len = sizeof_repeated_with_tag(tag, elems.iter(), |(k, v)| {
            let kvlen = sizeof_map_elem(k, v, |s| sizeof_len_record(s.len()), |_| 1);
            len_cache.push(kvlen);
            sizeof_len_record(kvlen)
        });

        let mut cached = len_cache.iter().copied();
        encoder
            .encode_repeated_with_tag(tag, elems.iter(), |wr, (k, v)| {
                wr.encode_map_elem(
                    cached.next().unwrap(),
                    k,
                    WIRE_TYPE_LEN,
                    v,
                    WIRE_TYPE_VARINT,
                    |wr, s| wr.encode_string(s),
                    |wr, b| wr.encode_bool(*b),
                )
            })
            .unwrap();

        assert_eq!(
            // 1st byte is tag, 2nd is length of the pair, rest are the payload of the pair
            [
                0x42, 5, 0x0A, 1, b'x', 0x10, 0x01, // first key-value pair
                0x42, 6, 0x0A, 2, b'x', b'y', 0x10, 0x00, // second key-value pair
                0x42, 7, 0x0A, 3, b'x', b'y', b'z', 0x10, 0x01 // third key-value pair
            ],
            encoder.writer.as_slice()
        );
        assert_eq!(encoder.bytes_written, encoder.writer.len());
        assert_eq!(24, len);
    }
}
