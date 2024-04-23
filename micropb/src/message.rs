#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::{
    encode::{PbEncoder, PbWrite},
    size::SizeCache,
};

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

#[cfg(feature = "encode")]
pub trait MessageEncode {
    fn encode<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error>;

    fn encode_len_delimited<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        encoder.encode_varint32(self.compute_size() as u32)?;
        self.encode(encoder)
    }

    //fn encode_cached<W: PbWrite>(
    //&self,
    //encoder: &mut PbEncoder<W>,
    //encode_len: bool,
    //_cache: &dyn SizeCache,
    //) -> Result<(), W::Error> {
    //self.encode(encoder, encode_len)
    //}

    fn compute_size(&self) -> usize;

    fn compute_size_cached(&self, _cache: &mut dyn SizeCache) -> usize {
        self.compute_size()
    }
}
