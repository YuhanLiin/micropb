#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};

#[cfg(feature = "decode")]
pub trait MessageDecode {
    fn decode<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        len: usize,
    ) -> Result<(), DecodeError<R::Error>>;

    fn decode_len_delimited<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>> {
        decoder.decode_len_record(|len, _, decoder| self.decode(decoder, len))
    }
}

#[cfg(feature = "decode")]
impl<T: MessageDecode> MessageDecode for &mut T {
    fn decode<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        len: usize,
    ) -> Result<(), DecodeError<R::Error>> {
        (*self).decode(decoder, len)
    }

    fn decode_len_delimited<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>> {
        (*self).decode_len_delimited(decoder)
    }
}

#[cfg(feature = "encode")]
pub trait MessageEncode {
    fn encode<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error>;

    fn encode_len_delimited<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        encoder.encode_varint32(self.compute_size() as u32)?;
        self.encode(encoder)
    }

    fn compute_size(&self) -> usize;
}

#[cfg(feature = "encode")]
impl<T: MessageEncode> MessageEncode for &T {
    fn encode<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        (*self).encode(encoder)
    }

    fn compute_size(&self) -> usize {
        (*self).compute_size()
    }

    fn encode_len_delimited<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        (*self).encode_len_delimited(encoder)
    }
}
