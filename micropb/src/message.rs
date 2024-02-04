#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
use crate::extension::{ExtensionRegistryDecode, ExtensionRegistryEncode, ExtensionRegistrySizeof};
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
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>>;

    fn decode_len_delimited<R: PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>> {
        decoder.decode_len_record(|len, _, decoder| self.decode(decoder, len, registry))
    }
}

#[cfg(feature = "encode")]
pub trait MessageEncode {
    fn encode<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        encode_len: bool,
        registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error>;

    fn encode_cached<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        encode_len: bool,
        _cache: &dyn SizeCache,
        registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error> {
        self.encode(encoder, encode_len, registry)
    }

    fn compute_size(&self, registry: Option<&dyn ExtensionRegistrySizeof>) -> usize;

    fn compute_size_cached(
        &self,
        _cache: &mut dyn SizeCache,
        registry: Option<&dyn ExtensionRegistrySizeof>,
    ) -> usize {
        self.compute_size(registry)
    }
}
