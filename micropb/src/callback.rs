#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::{
    extension::{ExtensionRegistryDecode, ExtensionRegistryEncode, ExtensionRegistrySizeof},
    Tag,
};

#[cfg(feature = "decode")]
pub trait DecodeCallback: Default {
    fn decode_field<R: PbRead>(
        &mut self,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>>;
}

#[cfg(feature = "encode")]
pub trait EncodeCallback: Default {
    fn encode_field<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error>;

    fn compute_field_size(&self, registry: Option<&dyn ExtensionRegistrySizeof>) -> usize;
}
