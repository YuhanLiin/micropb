#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::Tag;

#[cfg(feature = "decode")]
pub trait DecodeCallback: Default {
    fn decode_fields<R: PbRead>(
        &mut self,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>>;
}

#[cfg(feature = "encode")]
pub trait EncodeCallback: Default {
    fn encode_fields<W: PbWrite>(&self, encoder: PbEncoder<W>) -> Result<(), W::Error>;

    fn compute_fields_size(&self) -> usize;
}
