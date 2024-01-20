use crate::decode::{DecodeError, PbDecoder};

pub trait Message: Default {
    fn decode_update(&mut self, reader: &mut PbDecoder) -> Result<(), DecodeError>;

    fn decode(reader: &mut PbDecoder) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut this = Self::default();
        this.decode_update(reader)?;
        Ok(this)
    }
}
