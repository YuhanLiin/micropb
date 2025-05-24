use core::{
    mem::MaybeUninit,
    str::{from_utf8, Utf8Error},
};

use crate::{
    container::{PbString, PbVec},
    misc::{
        maybe_uninit_slice_assume_init_ref, maybe_uninit_write_slice,
        maybe_ununit_array_assume_init,
    },
    MessageDecode, Presence, Tag, WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LEN, WIRE_TYPE_VARINT,
};

use never::Never;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
/// Protobuf [decoder](PbDecoder) error.
///
/// The error is parametrized by the underlying reader's error type `E`. Most of the error variants
/// use simple enums to minimize the memory footprint.
pub enum DecodeError<E> {
    /// Varint exceeded max length of 10 bytes
    VarIntLimit,
    /// Reader encountered EOF in the middle of decoding
    UnexpectedEof,
    /// Encountered deprecated wire type
    Deprecation,
    /// Unknown Protobuf wire type encountered
    UnknownWireType,
    /// Field number of 0, which is not allowed
    ZeroField,
    /// Custom field decoding returned false for field number that should be recognized
    CustomField,
    /// Decoded string is not valid UTF8
    Utf8,
    /// Exceeded capcity of fixed container for `string`, `bytes`, repeated, or `map` field
    Capacity,
    /// Actual length of length-delimited record differs from value of length prefix
    WrongLen,
    /// Error returned from reader
    Reader(E),
}

impl<E> From<Utf8Error> for DecodeError<E> {
    fn from(_: Utf8Error) -> Self {
        Self::Utf8
    }
}

/// A reader from which Protobuf data is read, similar to [`std::io::BufRead`].
///
/// Like [`std::io::BufRead`], this trait assumes that the reader uses an underlying buffer.
/// [`PbDecoder`] uses this trait as the interface to decode incoming Protobuf messages.
pub trait PbRead {
    /// I/O error returned on read failure.
    type Error;

    /// Returns the internal buffer, filling it with more data if necessary.
    ///
    /// This call does not consume the underlying buffer, so calling it consecutively may yield the
    /// same contents. As such, this call must be followed by a [`pb_advance`](Self::pb_advance)
    /// call with the number of bytes that are "consumed" from the returned buffer, to ensure that
    /// consumed bytes won't be returned again.
    ///
    /// Empty buffer is returned if and only if the underlying reader has reached EOF.
    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error>;

    /// Consumes `bytes` from the underlying buffer.
    ///
    /// This function should be called after [`pb_read_chunk`](Self::pb_read_chunk). It advances
    /// the internal buffer by `bytes` so that those bytes won't be returned from future calls to
    /// [`pb_read_chunk`](Self::pb_read_chunk). This function doesn't perform I/O, so it's
    /// infallible.
    ///
    /// The `bytes` should not exceed the length of the buffer returned from
    /// [`pb_read_chunk`](Self::pb_read_chunk). Otherwise, the behaviour is implementation-defined.
    fn pb_advance(&mut self, bytes: usize);

    /// Try to read exactly the number of bytes needed to fill `buf`.
    ///
    /// Returns the number of bytes read, which will be at most the size of `buf`. If the return is
    /// less than `buf`, then the reader reached EOF before filling `buf`. This function will
    /// advance the reader by the amount of bytes read, so no need to call
    /// [`pb_advance`](Self::pb_advance).
    fn pb_read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Self::Error> {
        let mut pos = 0;
        loop {
            let remaining = buf.get_mut(pos..).unwrap_or(&mut []);
            if remaining.is_empty() {
                break;
            }
            let chunk = &self.pb_read_chunk()?;
            if chunk.is_empty() {
                return Ok(pos);
            }
            let n = maybe_uninit_write_slice(remaining, chunk);
            self.pb_advance(n);
            pos += n;
        }

        debug_assert_eq!(pos, buf.len());
        Ok(pos)
    }
}

impl<T: PbRead> PbRead for &mut T {
    type Error = T::Error;

    #[inline]
    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error> {
        (*self).pb_read_chunk()
    }

    #[inline]
    fn pb_advance(&mut self, bytes: usize) {
        (*self).pb_advance(bytes)
    }

    #[inline]
    fn pb_read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Self::Error> {
        (*self).pb_read_exact(buf)
    }
}

impl PbRead for &[u8] {
    type Error = Never;

    #[inline]
    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error> {
        Ok(*self)
    }

    #[inline]
    fn pb_advance(&mut self, bytes: usize) {
        *self = self.get(bytes..).unwrap_or(&[])
    }

    #[inline]
    fn pb_read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Self::Error> {
        let n = maybe_uninit_write_slice(buf, self);
        self.pb_advance(n);
        Ok(n)
    }
}

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
/// Adapter that implements [`PbRead`] for all implementers of [`std::io::BufRead`], allowing the
/// decoder to read from `std` readers.
pub struct StdReader<R>(pub R);

#[cfg(feature = "std")]
impl<R: std::io::BufRead> PbRead for StdReader<R> {
    type Error = std::io::Error;

    #[inline]
    fn pb_read_chunk(&mut self) -> Result<&[u8], Self::Error> {
        self.0.fill_buf()
    }

    #[inline]
    fn pb_advance(&mut self, bytes: usize) {
        self.0.consume(bytes)
    }
}

#[derive(Debug)]
/// Decoder that reads Protobuf bytes and decodes them into Rust types.
///
/// Main interface for decoding Protobuf messages. Reads bytes from an underlying [`PbRead`]
/// instance.
///
/// # Example
///
/// Decoding a Protobuf message:
/// ```no_run
/// use micropb::{PbRead, PbDecoder, MessageDecode, DecodeError};
///
/// # #[derive(Default)]
/// # struct ProtoMessage;
/// # impl micropb::MessageDecode for ProtoMessage {
/// #   fn decode<R: PbRead>(&mut self, decoder: &mut PbDecoder<R>, len: usize) -> Result<(), micropb::DecodeError<R::Error>> { todo!() }
/// # }
///
/// let data = [0x08, 0x96, 0x01];
/// // Slices implement `PbRead` out of the box
/// let mut decoder = PbDecoder::new(data.as_slice());
///
/// let mut message = ProtoMessage::default();
/// message.decode(&mut decoder, data.len())?;
/// # Ok::<(), DecodeError<never::Never>>(())
/// ```
///
/// # Reducing Code Size
///
/// To prevent multiple monomorphizations and increased code size, make sure you instantiate
/// `PbDecoder` with only one reader type across the whole application. If multiple readers need to
/// be supported, wrap them in an enum or use a trait object.
///
/// Likewise, try to only use one set of string, vec, and map types across the application to
/// reduce code size.
pub struct PbDecoder<R: PbRead> {
    reader: R,
    idx: usize,
    /// If this flag is set, then the decoder will never report a capacity error when decoding
    /// repeated fields. When the container is filled, the decoder will instead ignore excess
    /// elements on the wire. The decoder will still report capacity errors when decoding `bytes`
    /// and `string` values that exceed their fixed containers.
    pub ignore_repeated_cap_err: bool,
}

impl<R: PbRead> PbDecoder<R> {
    #[inline]
    /// Construct a new decoder from a [`PbRead`].
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            idx: 0,
            ignore_repeated_cap_err: false,
        }
    }

    #[inline]
    /// Transform the decoder into the underlying reader.
    pub fn into_reader(self) -> R {
        self.reader
    }

    #[inline]
    /// Get reference to underlying reader.
    pub fn as_reader(&self) -> &R {
        &self.reader
    }

    #[inline]
    /// Get the number of bytes that the decoder has consumed from the reader.
    pub fn bytes_read(&self) -> usize {
        self.idx
    }

    #[inline]
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

    /// Decode an `uint32`.
    pub fn decode_varint32(&mut self) -> Result<u32, DecodeError<R::Error>> {
        let b = self.get_byte()?;
        // Single byte case
        if b & 0x80 == 0 {
            return Ok(b as u32);
        }

        let mut varint: u32 = b as u32 & !0x80;
        let mut bitpos = 7;
        for i in 1..10 {
            let b = self.get_byte()?;
            // Take the first 5 bytes into account, but ignore the later 5 bytes since they're
            // going to be truncated anyways
            if i < 5 {
                let u = b & !0x80;
                varint |= (u as u32) << bitpos;
                bitpos += 7;
            }
            if b & 0x80 == 0 {
                return Ok(varint);
            }
        }
        Err(DecodeError::VarIntLimit)
    }

    #[cfg(feature = "enable-64bit")]
    /// Decode an `uint64`.
    pub fn decode_varint64(&mut self) -> Result<u64, DecodeError<R::Error>> {
        let b = self.get_byte()?;
        // Single byte case
        if b & 0x80 == 0 {
            return Ok(b as u64);
        }

        let mut varint: u64 = b as u64 & !0x80;
        let mut bitpos = 7;
        for _ in 1..10 {
            let b = self.get_byte()?;
            let u = b & !0x80;
            varint |= (u as u64) << bitpos;
            bitpos += 7;
            if b & 0x80 == 0 {
                return Ok(varint);
            }
        }
        Err(DecodeError::VarIntLimit)
    }

    #[inline]
    #[cfg(feature = "enable-64bit")]
    /// Decode an `int64`.
    pub fn decode_int64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_varint64().map(|u| u as i64)
    }

    #[inline]
    /// Decode an `int32`.
    pub fn decode_int32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_varint32().map(|u| u as i32)
    }

    #[inline]
    /// Decode an `sint32`.
    pub fn decode_sint32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_varint32()
            .map(|u| ((u >> 1) as i32) ^ -((u & 1) as i32))
    }

    #[inline]
    #[cfg(feature = "enable-64bit")]
    /// Decode an `sint64`.
    pub fn decode_sint64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_varint64()
            .map(|u| ((u >> 1) as i64) ^ -((u & 1) as i64))
    }

    #[inline]
    /// Decode a `bool`.
    pub fn decode_bool(&mut self) -> Result<bool, DecodeError<R::Error>> {
        Ok(self.decode_varint32()? != 0)
    }

    fn read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<(), DecodeError<R::Error>> {
        let bytes_read = self
            .reader
            .pb_read_exact(buf)
            .map_err(DecodeError::Reader)?;
        self.idx += bytes_read;

        if bytes_read < buf.len() {
            return Err(DecodeError::UnexpectedEof);
        }
        Ok(())
    }

    #[inline]
    /// Decode a `fixed32`.
    pub fn decode_fixed32(&mut self) -> Result<u32, DecodeError<R::Error>> {
        let mut data = [MaybeUninit::uninit(); 4];
        self.read_exact(&mut data)?;
        // SAFETY: read_exact is guaranteed to write to the whole buffer
        let data = unsafe { maybe_ununit_array_assume_init(data) };
        Ok(u32::from_le_bytes(data))
    }

    #[inline]
    #[cfg(feature = "enable-64bit")]
    /// Decode a `fixed64`.
    pub fn decode_fixed64(&mut self) -> Result<u64, DecodeError<R::Error>> {
        let mut data = [MaybeUninit::uninit(); 8];
        self.read_exact(&mut data)?;
        // SAFETY: read_exact is guaranteed to write to the whole buffer
        let data = unsafe { maybe_ununit_array_assume_init(data) };
        Ok(u64::from_le_bytes(data))
    }

    #[inline]
    /// Decode a `fixed64` but keep only the lower 32 bits.
    ///
    /// Avoids 64-bit operations for `fixed64` if only the lower bits are needed. This can have
    /// performance benefits on 32-bit architectures.
    pub fn decode_fixed64_as_32(&mut self) -> Result<u32, DecodeError<R::Error>> {
        let n = self.decode_fixed32()?;
        self.skip_bytes(4)?;
        Ok(n)
    }

    #[inline]
    /// Decode a `sfixed32`.
    pub fn decode_sfixed32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        self.decode_fixed32().map(|u| u as i32)
    }

    #[inline]
    #[cfg(feature = "enable-64bit")]
    /// Decode a `sfixed64`.
    pub fn decode_sfixed64(&mut self) -> Result<i64, DecodeError<R::Error>> {
        self.decode_fixed64().map(|u| u as i64)
    }

    #[inline]
    /// Decode a `sfixed64` but keep only the lower 32 bits.
    ///
    /// Avoids 64-bit operations for `sfixed64` if only the lower bits are needed. This can have
    /// performance benefits on 32-bit architectures.
    pub fn decode_sfixed64_as_32(&mut self) -> Result<i32, DecodeError<R::Error>> {
        let n = self.decode_sfixed32()?;
        self.skip_bytes(4)?;
        Ok(n)
    }

    #[inline]
    /// Decode a `float`.
    pub fn decode_float(&mut self) -> Result<f32, DecodeError<R::Error>> {
        self.decode_fixed32().map(f32::from_bits)
    }

    #[inline]
    /// Decode a `double`.
    pub fn decode_double(&mut self) -> Result<f64, DecodeError<R::Error>> {
        let mut data = [MaybeUninit::uninit(); 8];
        self.read_exact(&mut data)?;
        // SAFETY: read_exact is guaranteed to write to the whole buffer
        let data = unsafe { maybe_ununit_array_assume_init(data) };
        Ok(f64::from_le_bytes(data))
    }

    #[inline(always)]
    /// Decode a Protobuf tag.
    pub fn decode_tag(&mut self) -> Result<Tag, DecodeError<R::Error>> {
        self.decode_varint32().map(Tag)
    }

    #[inline]
    fn read_into_buf<'a>(
        &mut self,
        buf: &'a mut [MaybeUninit<u8>],
        len: usize,
    ) -> Result<&'a [u8], DecodeError<R::Error>> {
        if buf.len() < len {
            return Err(DecodeError::Capacity);
        }
        let target = &mut buf[..len];
        self.read_exact(target)?;
        // SAFETY: read_exact guarantees that all bytes of target have been initialized
        Ok(unsafe { maybe_uninit_slice_assume_init_ref(target) })
    }

    /// Decode a `string` into a [`PbString`] container.
    ///
    /// The string container's existing contents will be replaced by the string decoded from the
    /// wire. However, if `presence` is implicit and the new string is empty, the existing string
    /// will remain unchanged.
    ///
    /// # Errors
    ///
    /// If the length of the string on the wire exceeds the fixed capacity of the string container,
    /// return [`DecodeError::Capacity`]. If the string on the wire if not UTF-8, return
    /// [`DecodeError::Utf8`].
    pub fn decode_string<S: PbString>(
        &mut self,
        string: &mut S,
        presence: Presence,
    ) -> Result<(), DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        // With implicit presence, ignore empty strings
        if len == 0 && presence == Presence::Implicit {
            return Ok(());
        }

        string.pb_clear();
        string.pb_reserve(len);
        let spare_cap = string.pb_spare_cap();
        let written = match self.read_into_buf(spare_cap, len) {
            Ok(w) => w,
            Err(e) => {
                // Clear UTF8 errors for fixed-len String
                string.pb_clear();
                return Err(e);
            }
        };

        // Check UTF8 validity
        if let Err(e) = from_utf8(written) {
            // Clear UTF8 errors for fixed-len String
            string.pb_clear();
            return Err(e.into());
        }
        // SAFETY: read_into_buf guarantees that `len` bytes have been written into the string.
        // Also, we just checked the UTF-8 validity of the written bytes, so the string is valid.
        unsafe { string.pb_set_len(len) };
        Ok(())
    }

    /// Decode a `bytes` into a [`PbVec<u8>`](crate::PbVec<u8>) container.
    ///
    /// The byte container's existing contents will be replaced by the bytes decoded from the
    /// wire. However, if `presence` is implicit and the new bytes is empty, the existing container
    /// will remain unchanged.
    ///
    /// # Errors
    ///
    /// If the length of the bytes on the wire exceeds the fixed capacity of the byte container,
    /// return [`DecodeError::Capacity`].
    pub fn decode_bytes<S: PbVec<u8>>(
        &mut self,
        bytes: &mut S,
        presence: Presence,
    ) -> Result<(), DecodeError<R::Error>> {
        let len = self.decode_varint32()? as usize;
        // With implicit presence, ignore empty strings
        if len == 0 && presence == Presence::Implicit {
            return Ok(());
        }

        bytes.pb_clear();
        bytes.pb_reserve(len);
        let spare_cap = bytes.pb_spare_cap();
        self.read_into_buf(spare_cap, len)?;
        // SAFETY: read_into_buf guarantees that `len` bytes have been written into the buffer
        unsafe { bytes.pb_set_len(len) };
        Ok(())
    }

    pub(crate) fn decode_len_record<
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
            Err(DecodeError::WrongLen)
        } else {
            Ok(val)
        }
    }

    /// Decode a repeated packed field and append the elements to a [`PbVec`] container.
    ///
    /// The `decoder` callback determines how each element is decoded from the wire. If the number
    /// of elements on the wire exceeds the remaining fixed capacity of the container and the
    /// `ignore_repeated_cap_err` flag is not set, return [`DecodeError::Capacity`].
    pub fn decode_packed<
        T: Copy,
        S: PbVec<T>,
        F: Fn(&mut Self) -> Result<T, DecodeError<R::Error>>,
    >(
        &mut self,
        vec: &mut S,
        decoder: F,
    ) -> Result<(), DecodeError<R::Error>> {
        let ignore_repeated_cap_err = self.ignore_repeated_cap_err;
        self.decode_len_record(|len, before, this| {
            while this.bytes_read() - before < len {
                let val = decoder(this)?;
                if let (Err(_), false) = (vec.pb_push(val), ignore_repeated_cap_err) {
                    return Err(DecodeError::Capacity);
                }
            }
            Ok(())
        })
    }

    //#[cfg(target_endian = "little")]
    //pub fn decode_packed_fixed<T: DecodeFixedSize, S: PbVec<T>>(
    //&mut self,
    //vec: &mut S,
    //) -> Result<(), DecodeError<R::Error>> {
    //let len = self.decode_varint32()? as usize;
    //let elem_size = core::mem::size_of::<T>();
    //let modulo = len % elem_size;
    //// Length must be a multiple of elem_size
    //if modulo > 0 {
    //return Err(DecodeError::WrongLen {
    //expected: len,
    //// Previous multiple of elem_size
    //actual: len - modulo,
    //});
    //}
    //let elem_num = len / elem_size;
    //vec.pb_reserve(elem_num);
    //let spare_cap = vec.pb_spare_cap();
    //if spare_cap.len() < elem_num {
    //return Err(DecodeError::Capacity);
    //}
    //// SAFETY: Converting slice into uninitialized bytes is always valid. Moreover, we know
    //// that `spare_cap` has equal or more than `elem_num` values, so its size in bytes can't
    //// be less than `len`, because `len` is equal to `elem_num * size_of<T>()`.
    //let spare_bytes = unsafe {
    //core::slice::from_raw_parts_mut(spare_cap.as_mut_ptr() as *mut MaybeUninit<u8>, len)
    //};
    //self.read_exact(spare_bytes)?;
    //// SAFETY: We just wrote elem_num of elements into the spare space, so we can
    //// increase the length by that much
    //unsafe { vec.pb_set_len(vec.len() + elem_num) };
    //Ok(())
    //}

    /// Decode a Protobuf map key-value pair from the decoder.
    ///
    /// According the the Protobuf spec, the key-value pair is formatted as a Protobuf message with
    /// the key in field 1 and the value in field 2. Other field numbers are ignored.
    ///
    /// The `key_update` and `val_update` callbacks are expected to decode the key and value
    /// respectively. If either key or value field is not found, return `None`.
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
                    1 => key_update(key.get_or_insert_with(K::default), this)?,
                    2 => val_update(val.get_or_insert_with(V::default), this)?,
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

    fn skip_varint(&mut self) -> Result<(), DecodeError<R::Error>> {
        for _ in 0..10 {
            let b = self.get_byte()?;
            if b & 0x80 == 0 {
                return Ok(());
            }
        }
        Err(DecodeError::VarIntLimit)
    }

    /// Consume some bytes from the reader.
    ///
    /// If reader reached EOF before the specified number of bytes are skipped, return
    /// [`DecodeError::UnexpectedEof`].
    pub fn skip_bytes(&mut self, bytes: usize) -> Result<(), DecodeError<R::Error>> {
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

    /// Skip the next Protobuf value/payload on the wire.
    ///
    /// The type of the Protobuf payload is determined by `wire_type`, which must be a valid
    /// Protobuf wire type. This is mainly used to skip unknown fields.
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
            _ => return Err(DecodeError::UnknownWireType),
        }
        Ok(())
    }

    /// Decode a new message from the wire.
    pub fn decode_message<M: MessageDecode + Default>(
        &mut self,
        len: usize,
    ) -> Result<M, DecodeError<R::Error>> {
        let mut msg = M::default();
        msg.decode(self, len)?;
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Range;

    use arrayvec::{ArrayString, ArrayVec};

    use crate::{FixedLenArray, FixedLenString};

    use super::*;

    struct Multichunk<'a>(&'a [u8]);

    impl Multichunk<'_> {
        fn len(&self) -> usize {
            self.0.len()
        }

        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl PbRead for Multichunk<'_> {
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
            Ok(1),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00],
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
            Err(DecodeError::VarIntLimit),
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
            Err(DecodeError::VarIntLimit),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            skip_varint()
        );
    }

    #[test]
    fn int() {
        assert_decode!(Ok(5), [5], decode_int32());
        assert_decode!(Ok(5), [5], decode_int64());

        // big varints get casted down to 32 bits
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
            Ok(-1),
            [0x81, 0x80, 0x80, 0x80, 0x80, 0x00],
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
            Err(DecodeError::VarIntLimit),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            decode_sint64()
        );
    }

    #[test]
    fn bool() {
        assert_decode!(Ok(false), [0], decode_bool());
        assert_decode!(Ok(true), [1], decode_bool());
        assert_decode!(Ok(true), [0x3], decode_bool());
        assert_decode!(Ok(false), [0x80, 0x00], decode_bool());
        assert_decode!(Ok(true), [0x80, 0x01], decode_bool());
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
    fn fixed_64_as_32() {
        assert_decode!(
            Ok(0xF4983212),
            [0x12, 0x32, 0x98, 0xF4, 0x12, 0x34, 0x00, 0x00],
            decode_fixed64_as_32()
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x12, 0x32, 0x98, 0xF4, 0x12, 0x34, 0x00],
            decode_fixed64_as_32()
        );
        assert_decode!(
            Ok(-0x0B67CDEE),
            [0x12, 0x32, 0x98, 0xF4, 0xFF, 0xFF, 0x00, 0x00],
            decode_sfixed64_as_32()
        );
        assert_decode!(
            Err(DecodeError::UnexpectedEof),
            [0x12, 0x32, 0x98, 0xF4],
            decode_sfixed64_as_32()
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
        assert_decode!(Err(DecodeError::UnknownWireType), [], skip_wire_value(10));
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
        ($test:ident, $name:ident, $container:ty, $fixed_cap:literal, $fixed_len:literal) => {
            #[test]
            fn $name() {
                $test::<$container>($fixed_cap, $fixed_len);
            }
        };
    }

    fn string<S: PbString + Default>(fixed_cap: bool, fixed_len: bool) {
        let mut string = S::default();
        if fixed_len {
            assert_decode_vec!(
                Ok("\0\0\0\0"),
                [0],
                decode_string(string, Presence::Explicit)
            );
        } else {
            assert_decode_vec!(Ok(""), [0], decode_string(string, Presence::Explicit));
        }
        if fixed_len {
            assert_decode_vec!(
                Ok("a\0\0\0"),
                [1, b'a'],
                decode_string(string, Presence::Implicit)
            );
        } else {
            assert_decode_vec!(
                Ok("a"),
                [1, b'a'],
                decode_string(string, Presence::Implicit)
            );
        }
        assert_decode_vec!(
            Ok("abcd"),
            [4, b'a', b'b', b'c', b'd'],
            decode_string(string, Presence::Explicit)
        );
        assert_decode_vec!(
            Ok("Зд"),
            [4, 208, 151, 208, 180],
            decode_string(string, Presence::Implicit)
        );
        if !fixed_len {
            assert_decode_vec!(Ok("Зд"), [0], decode_string(string, Presence::Implicit));
        }

        string.pb_clear();
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [],
            decode_string(string, Presence::Explicit)
        );
        if fixed_len {
            assert_eq!(string.deref(), "\0\0\0\0");
        }
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [4, b'b', b'c', b'd'],
            decode_string(string, Presence::Explicit)
        );
        if fixed_len {
            assert_eq!(string.deref(), "\0\0\0\0");
        }
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [5, b'a', b'b', b'c', b'd', b'e'],
                decode_string(string, Presence::Explicit)
            );
        }
        if fixed_len {
            assert_eq!(string.deref(), "\0\0\0\0");
        }
        assert_decode_vec!(
            Err(DecodeError::Utf8),
            [4, 0x80, 0x80, 0x80, 0x80],
            decode_string(string, Presence::Explicit)
        );
        if fixed_len {
            assert_eq!(string.deref(), "\0\0\0\0");
        }
    }

    container_test!(string, string_arrayvec, ArrayString::<4>, true, false);
    container_test!(string, string_heapless, heapless::String::<4>, true, false);
    container_test!(string, string_alloc, String, false, false);
    container_test!(string, string_fixed_len, FixedLenString::<4>, true, true);

    fn bytes<S: PbVec<u8> + Default>(fixed_cap: bool, fixed_len: bool) {
        let mut bytes = S::default();
        if fixed_len {
            assert_decode_vec!(Ok(&[0, 0, 0]), [0], decode_bytes(bytes, Presence::Explicit));
            assert_decode_vec!(
                Ok(b"a\0\0"),
                [1, b'a'],
                decode_bytes(bytes, Presence::Implicit)
            );
        } else {
            assert_decode_vec!(Ok(&[]), [0], decode_bytes(bytes, Presence::Explicit));
            assert_decode_vec!(Ok(b"a"), [1, b'a'], decode_bytes(bytes, Presence::Implicit));
        }
        assert_decode_vec!(
            Ok(&[0x10, 0x20, 0x30]),
            [3, 0x10, 0x20, 0x30],
            decode_bytes(bytes, Presence::Explicit)
        );
        assert_decode_vec!(
            Ok(&[0x10, 0x20, 0x30]),
            [0],
            decode_bytes(bytes, Presence::Implicit)
        );

        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [],
            decode_bytes(bytes, Presence::Explicit)
        );
        if fixed_cap {
            assert_decode_vec!(
                Err(DecodeError::Capacity),
                [4, 0x10, 0x20, 0x30, 0x40],
                decode_bytes(bytes, Presence::Explicit)
            );
        }
        assert_decode_vec!(
            Err(DecodeError::UnexpectedEof),
            [3, 0x20, 0x30],
            decode_bytes(bytes, Presence::Explicit)
        );
    }

    container_test!(bytes, bytes_arrayvec, ArrayVec::<_, 3>, true, false);
    container_test!(bytes, bytes_heapless, heapless::Vec::<_, 3>, true, false);
    container_test!(bytes, bytes_alloc, Vec<_>, false, false);
    container_test!(bytes, bytes_fixed, FixedLenArray<u8, 3>, true, true);

    fn packed<S: PbVec<u32> + Default>(fixed_cap: bool, _fixed_len: bool) {
        let mut vec1 = S::default();
        let mut vec2 = S::default();
        assert_decode_vec!(
            Ok(&[]),
            [0],
            decode_packed(vec1 | vec2, |rd| rd.decode_varint32())
        );
        assert_decode_vec!(
            Err(DecodeError::WrongLen),
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

    container_test!(packed, packed_arrayvec, ArrayVec::<_, 5>, true, false);
    container_test!(packed, packed_heapless, heapless::Vec::<_, 5>, true, false);
    container_test!(packed, packed_alloc, Vec<_>, false, false);

    //#[cfg(target_endian = "little")]
    //fn packed_fixed<S: PbVec<u32>>(fixed_cap: bool) {
    //let mut vec1 = S::default();
    //let mut vec2 = S::default();
    //assert_decode_vec!(Ok(&[]), [0], decode_packed_fixed(vec1 | vec2));
    //assert_decode_vec!(
    //Ok(&[0x04030201]),
    //[4, 0x01, 0x02, 0x03, 0x04],
    //decode_packed_fixed(vec1 | vec2)
    //);
    //assert_decode_vec!(
    //Ok(&[0x04030201, 0x0D0C0B0A, 0x44332211]),
    //[8, 0x0A, 0x0B, 0x0C, 0x0D, 0x11, 0x22, 0x33, 0x44],
    //decode_packed_fixed(vec1 | vec2)
    //);
    //if fixed_cap {
    //assert_decode_vec!(
    //Err(DecodeError::Capacity),
    //[4, 0x01, 0x02, 0x03, 0x04],
    //decode_packed_fixed(vec1 | vec2)
    //);
    //}
    //assert_decode_vec!(
    //Err(DecodeError::WrongLen {
    //expected: 1,
    //actual: 0
    //}),
    //[1, 0x01],
    //decode_packed_fixed(vec1 | vec2)
    //);
    //}

    //#[cfg(target_endian = "little")]
    //container_test!(packed_fixed, pf_arrayvec, ArrayVec::<_, 3>, true);
    //#[cfg(target_endian = "little")]
    //container_test!(packed_fixed, pf_heapless, heapless::Vec::<_, 3>, true);
    //#[cfg(target_endian = "little")]
    //container_test!(packed_fixed, pf_alloc, Vec<_>, false);

    /// Test decoding of a map element with varint32 key and string value
    macro_rules! assert_decode_map_elem {
        ($expected:expr, $arr:expr) => {
            assert_decode!(
                $expected,
                $arr,
                decode_map_elem(
                    |v, rd| rd.decode_varint32().map(|u| *v = u),
                    |v, rd| rd.decode_string::<ArrayString<5>>(v, Presence::Explicit)
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
            Err(DecodeError::UnknownWireType),
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
                |v, rd| rd.decode_string::<ArrayString<5>>(v, Presence::Explicit),
                |v, rd| rd.decode_string::<ArrayString<5>>(v, Presence::Explicit)
            )
        );
    }

    // The following proptests exercise unsafe code in micropb. Since we only care about catching
    // UB, we only need to run the tests under miri.
    #[cfg(miri)]
    mod r#unsafe {
        use super::*;
        use proptest::prelude::*;

        pub(crate) fn bytes_strat(sizes: Range<usize>) -> impl Strategy<Value = Vec<u8>> {
            proptest::collection::vec(
                proptest::num::u8::ANY,
                proptest::collection::size_range(sizes),
            )
        }

        macro_rules! check_decode {
            (@testcase $expect_ok:expr, $reader:expr, $($op:tt)+) => {
                let mut decoder = PbDecoder::new($reader);
                let res = decoder.$($op)+;
                if $expect_ok {
                    assert!(res.is_ok());
                } else {
                    assert!(res.is_err());
                }
            };

            ($expect_ok:expr, $arr:expr, $($op:tt)+) => {
                check_decode!(@testcase $expect_ok, $arr.as_slice(), $($op)+);
                check_decode!(@testcase $expect_ok, Multichunk($arr.as_slice()), $($op)+);
            };
        }

        fn check_string(mut string: impl PbString + Clone, data: Vec<u8>) {
            let mut string_cl = string.clone();
            let mut decoder = PbDecoder::new(data.as_slice());
            let _ = decoder.decode_string(&mut string, Presence::Implicit);
            let mut decoder = PbDecoder::new(Multichunk(data.as_slice()));
            let _ = decoder.decode_string(&mut string_cl, Presence::Explicit);
        }

        fn check_bytes(mut bytes: impl PbVec<u8> + Clone, data: Vec<u8>) {
            let mut bytes_cl = bytes.clone();
            let mut decoder = PbDecoder::new(data.as_slice());
            let _ = decoder.decode_bytes(&mut bytes, Presence::Implicit);
            let mut decoder = PbDecoder::new(Multichunk(data.as_slice()));
            let _ = decoder.decode_bytes(&mut bytes_cl, Presence::Explicit);
        }

        proptest! {
            // Need this for miri to work
            #![proptest_config(proptest::test_runner::Config {
                failure_persistence: None,
                ..Default::default()
            })]
            #[test]
            fn proptest_fixed32(data in bytes_strat(0..6)) {
                check_decode!(data.len() >= 4, data, decode_fixed32());
            }
            #[test]
            fn proptest_fixed64(data in bytes_strat(2..10)) {
                check_decode!(data.len() >= 8, data, decode_fixed64());
            }
            #[test]
            fn proptest_float(data in bytes_strat(0..6)) {
                check_decode!(data.len() >= 4, data, decode_float());
            }
            #[test]
            fn proptest_double(data in bytes_strat(2..10)) {
                check_decode!(data.len() >= 8, data, decode_double());
            }

            #[test]
            fn proptest_string(data in bytes_strat(4..32)) {
                check_string(String::new(), data);
            }
            #[test]
            fn proptest_arraystring(data in bytes_strat(4..32)) {
                check_string(ArrayString::<16>::new(), data);
            }
            #[test]
            fn proptest_hlstring(data in bytes_strat(4..32)) {
                check_string(heapless::String::<16>::new(), data);
            }
            #[test]
            fn proptest_fixedstring(data in bytes_strat(4..32)) {
                check_string(FixedLenString::<16>::default(), data);
            }

            #[test]
            fn proptest_vec(data in bytes_strat(4..32)) {
                check_bytes(Vec::new(), data);
            }
            #[test]
            fn proptest_arrayvec(data in bytes_strat(4..32)) {
                check_bytes(ArrayVec::<u8, 16>::new(), data);
            }
            #[test]
            fn proptest_hlvec(data in bytes_strat(4..32)) {
                check_bytes(heapless::Vec::<u8, 16>::new(), data);
            }
            #[test]
            fn proptest_fixedarray(data in bytes_strat(4..32)) {
                check_bytes(FixedLenArray::<u8, 16>::default(), data);
            }
        }
    }
}
