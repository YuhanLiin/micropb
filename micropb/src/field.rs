#[cfg(feature = "decode")]
use crate::decode::{DecodeError, PbDecoder, PbRead};
#[cfg(feature = "encode")]
use crate::encode::{PbEncoder, PbWrite};
use crate::Tag;

#[cfg(feature = "decode")]
pub trait FieldDecode {
    fn decode_field<R: PbRead>(
        &mut self,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
    ) -> Result<bool, DecodeError<R::Error>>;
}

#[cfg(feature = "decode")]
impl<T: FieldDecode> FieldDecode for &mut T {
    fn decode_field<R: PbRead>(
        &mut self,
        tag: Tag,
        decoder: &mut PbDecoder<R>,
    ) -> Result<bool, DecodeError<R::Error>> {
        (*self).decode_field(tag, decoder)
    }
}

#[cfg(feature = "encode")]
pub trait FieldEncode {
    fn encode_field<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error>;

    fn compute_field_size(&self) -> usize;
}

#[cfg(feature = "encode")]
impl<T: FieldEncode> FieldEncode for &T {
    fn encode_field<W: PbWrite>(&self, encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        (*self).encode_field(encoder)
    }

    fn compute_field_size(&self) -> usize {
        (*self).compute_field_size()
    }
}
