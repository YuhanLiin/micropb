#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};

#[cfg(feature = "decode")]
pub trait MessageDecode: Default {
    fn decode_update<R: PbRead>(
        &mut self,
        reader: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>>;

    fn decode<R: PbRead>(reader: &mut PbDecoder<R>) -> Result<Self, DecodeError<R::Error>>
    where
        Self: Sized,
    {
        let mut this = Self::default();
        this.decode_update(reader)?;
        Ok(this)
    }
}

#[cfg(feature = "encode")]
pub trait MessageEncode: Default {
    fn encode<W: PbWrite>(&self, writer: &mut PbEncoder<W>) -> Result<(), W::Error>;

    fn compute_size(&self) -> usize;
}
