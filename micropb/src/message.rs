#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::extension::{ExtensionRegistryDecode, ExtensionRegistryEncode, ExtensionRegistrySizeof};

pub trait Message: Default {
    #[cfg(feature = "decode")]
    fn decode<R: PbRead>(
        &mut self,
        _reader: &mut PbDecoder<R>,
        _len: Option<usize>,
        _registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn encode<W: PbWrite>(
        &self,
        _writer: &mut PbEncoder<W>,
        _encode_len: bool,
        _registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn compute_size(&self, _registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
        0
    }
}
