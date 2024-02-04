#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::extension::{ExtensionRegistryDecode, ExtensionRegistryEncode, ExtensionRegistrySizeof};

#[cfg(feature = "decode")]
pub trait MessageDecode: Default {
    fn decode<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        len: Option<usize>,
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>>;

    fn decode_with_len<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>>;
}

#[cfg(feature = "encode")]
pub trait MessageEncode: Default {
    fn encode<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        encode_len: bool,
        registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error>;

    fn compute_size(&self, registry: Option<&dyn ExtensionRegistrySizeof>) -> usize;
}
