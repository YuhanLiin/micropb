#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};

pub trait Message: Default {
    #[cfg(feature = "decode")]
    fn decode<R: PbRead>(
        &mut self,
        _reader: &mut PbDecoder<R>,
        _len: Option<usize>,
    ) -> Result<(), DecodeError<R::Error>> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn encode<W: PbWrite>(
        &self,
        _writer: &mut PbEncoder<W>,
        _encode_len: bool,
    ) -> Result<(), W::Error> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn compute_size(&self) -> usize {
        0
    }
}
