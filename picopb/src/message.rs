use crate::decode::{DecodeError, PbDecoder, PbRead};

pub trait Message: Default {
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
