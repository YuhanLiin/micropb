#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::{
    extension::{ExtensionRegistryDecode, ExtensionRegistryEncode, ExtensionRegistrySizeof},
    Tag,
};

pub trait Field {
    #[cfg(feature = "decode")]
    fn decode_field<R: PbRead>(
        &mut self,
        _tag: Tag,
        _decoder: &mut PbDecoder<R>,
    ) -> Result<(), DecodeError<R::Error>> {
        Ok(())
    }

    #[cfg(feature = "decode")]
    fn decode_field_ext<R: PbRead>(
        &mut self,
        _tag: Tag,
        _decoder: &mut PbDecoder<R>,
        _registry: &mut dyn ExtensionRegistryDecode<R>,
    ) -> Result<(), DecodeError<R::Error>> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn encode_field<W: PbWrite>(&self, _encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn encode_field_ext<W: PbWrite>(
        &self,
        _encoder: &mut PbEncoder<W>,
        _registry: &dyn ExtensionRegistryEncode<W>,
    ) -> Result<(), W::Error> {
        Ok(())
    }

    #[cfg(feature = "encode")]
    fn compute_field_size(&self) -> usize {
        0
    }

    #[cfg(feature = "encode")]
    fn compute_field_size_ext(&self, _registry: &dyn ExtensionRegistrySizeof) -> usize {
        0
    }
}
