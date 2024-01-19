use crate::decode::{DecodeError, PbReader};

pub trait Message: Default {
    fn decode_update(&mut self, reader: &mut PbReader) -> Result<(), DecodeError>;

    fn decode(reader: &mut PbReader) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut this = Self::default();
        this.decode_update(reader)?;
        Ok(this)
    }
}
