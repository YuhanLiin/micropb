use core::any::{Any, TypeId};

use micropb::{
    extension::{
        ExtensionField, ExtensionId, ExtensionRegistry, ExtensionRegistryDecode,
        ExtensionRegistryEncode, ExtensionRegistrySizeof, RegistryError,
    },
    field::{FieldDecode, FieldEncode},
    size::{sizeof_tag, sizeof_varint32},
    static_extension_registry, DecodeError, PbDecoder, PbEncoder, PbRead, PbWrite, Tag,
};

#[derive(Debug, Default)]
struct NumMsg {}

#[derive(Debug, Default)]
struct RecursiveMsg {
    ext: Option<ExtensionId>,
}

#[derive(Debug, Default)]
struct NumExtension1(u32);
impl ExtensionField for NumExtension1 {
    const FIELD_NUM: u32 = 1;
    type MESSAGE = NumMsg;
}
impl FieldDecode for NumExtension1 {
    fn decode_field<R: PbRead>(
        &mut self,
        _tag: Tag,
        decoder: &mut PbDecoder<R>,
        _registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>> {
        self.0 = decoder.decode_varint32()?;
        Ok(())
    }
}
impl FieldEncode for NumExtension1 {
    fn encode_field<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        _registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error> {
        encoder.encode_tag(Tag::from_parts(1, 0))?;
        encoder.encode_varint32(self.0)?;
        Ok(())
    }

    fn compute_field_size(&self, _registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
        sizeof_tag(Tag::from_parts(1, 0)) + sizeof_varint32(self.0)
    }
}

// Extension for RecursiveMsg instead of NumMsg
#[derive(Debug, Default)]
struct NumExtension2(u32);
impl ExtensionField for NumExtension2 {
    const FIELD_NUM: u32 = 1;
    type MESSAGE = RecursiveMsg;
}
impl FieldDecode for NumExtension2 {
    fn decode_field<R: PbRead>(
        &mut self,
        _tag: Tag,
        decoder: &mut PbDecoder<R>,
        _registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>> {
        self.0 = decoder.decode_varint32()?;
        Ok(())
    }
}
impl FieldEncode for NumExtension2 {
    fn encode_field<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        _registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error> {
        encoder.encode_tag(Tag::from_parts(1, 0))?;
        encoder.encode_varint32(self.0)?;
        Ok(())
    }

    fn compute_field_size(&self, _registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
        sizeof_tag(Tag::from_parts(1, 0)) + sizeof_varint32(self.0)
    }
}

#[derive(Debug, Default)]
struct RecursiveExtension(RecursiveMsg);

impl ExtensionField for RecursiveExtension {
    const FIELD_NUM: u32 = 2;
    type MESSAGE = RecursiveMsg;
}
impl FieldDecode for RecursiveExtension {
    fn decode_field<R: PbRead>(
        &mut self,
        _tag: Tag,
        decoder: &mut PbDecoder<R>,
        registry: Option<&mut dyn ExtensionRegistryDecode<R>>,
    ) -> Result<(), DecodeError<R::Error>> {
        let registry = registry.unwrap();
        let id = registry.alloc_ext(&self.0.type_id()).unwrap();
        self.0.ext = Some(id);
        loop {
            let idx = decoder.bytes_read();
            let tag = match decoder.decode_tag() {
                Ok(tag) => tag,
                Err(DecodeError::UnexpectedEof) if decoder.bytes_read() == idx => return Ok(()),
                Err(e) => return Err(e),
            };
            registry.decode_ext_field(id, tag, decoder)?;
        }
    }
}
impl FieldEncode for RecursiveExtension {
    fn encode_field<W: PbWrite>(
        &self,
        encoder: &mut PbEncoder<W>,
        registry: Option<&dyn ExtensionRegistryEncode<W>>,
    ) -> Result<(), W::Error> {
        let registry = registry.unwrap();
        if let Some(id) = self.0.ext {
            encoder.encode_tag(Tag::from_parts(2, 0))?;
            registry.encode_ext(id, encoder)?;
        }
        Ok(())
    }

    fn compute_field_size(&self, registry: Option<&dyn ExtensionRegistrySizeof>) -> usize {
        let registry = registry.unwrap();
        self.0
            .ext
            .map(|id| sizeof_tag(Tag::from_parts(2, 0)) + registry.compute_ext_size(id).unwrap())
            .unwrap_or(0)
    }
}

static_extension_registry!(
    TestRegistry,
    NumMsg[5] => {
        num: NumExtension1,
    }
    RecursiveMsg[8] => {
        num: NumExtension2,
        msg: RecursiveExtension
    }
);

#[test]
fn map_macro() {
    let mut registry = TestRegistry::default();
    let mut decoder = PbDecoder::new([0x57].as_slice());
    assert_eq!(registry.alloc_ext(&TypeId::of::<u32>()), None);
    let id = registry.alloc_ext(&TypeId::of::<NumMsg>()).unwrap();

    assert!(registry.get_field::<NumExtension1>(id).unwrap().is_none());
    assert!(registry.mut_field::<NumExtension1>(id).unwrap().is_none());
    // unknown field number, ignored
    assert!(!registry
        .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
        .unwrap());
    assert!(registry
        .decode_ext_field(id, Tag::from_parts(1, 0), &mut decoder)
        .unwrap());
    assert_eq!(
        registry.get_field::<NumExtension1>(id).unwrap().unwrap().0,
        0x57
    );

    let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
    registry.mut_field::<NumExtension1>(id).unwrap().unwrap().0 = 0x69;
    assert!(registry.encode_ext(id, &mut encoder).unwrap());
    // encoding also outputs the tag
    assert_eq!(encoder.into_inner(), &[0x08, 0x69]);
    assert_eq!(registry.compute_ext_size(id).unwrap(), 2);

    assert_eq!(
        registry.clear_field::<NumExtension2>(id).unwrap_err(),
        RegistryError::BadField,
    );
    registry.clear_field::<NumExtension1>(id).unwrap();
    registry.clear_field::<NumExtension1>(id).unwrap();
    assert!(registry.mut_field::<NumExtension1>(id).unwrap().is_none());
    let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
    assert!(registry.encode_ext(id, &mut encoder).unwrap());
    // Encode with no fields
    assert_eq!(encoder.into_inner(), &[]);
    assert_eq!(registry.compute_ext_size(id).unwrap(), 0);
    registry.add_field::<NumExtension1>(id).unwrap().0 = 0x13;
    assert_eq!(
        registry.get_field::<NumExtension1>(id).unwrap().unwrap().0,
        0x13
    );

    registry.dealloc_ext(id).unwrap();
    // Deallocating a "null" ID is not an error
    registry.dealloc_ext(id).unwrap();
    assert_eq!(
        registry.mut_field::<NumExtension1>(id).unwrap_err(),
        RegistryError::IdNotFound
    );
}

#[test]
fn map_macro_recursive() {
    let mut registry = TestRegistry::default();
    // 0x08 is tag for field 1, 0x10 is tag for field 2
    let mut decoder = PbDecoder::new([0x08, 0x34, 0x10, 0x08, 0x12, 0x10, 0x08, 0x55].as_slice());
    let id = registry.alloc_ext(&TypeId::of::<RecursiveMsg>()).unwrap();

    // Populate the RecursiveMsg field of the extension
    assert!(registry
        .decode_ext_field(id, Tag::from_parts(2, 0), &mut decoder)
        .unwrap());
    let id1 = registry
        .get_field::<RecursiveExtension>(id)
        .unwrap()
        .unwrap()
        .0
        .ext
        .unwrap();
    assert_eq!(
        registry.get_field::<NumExtension2>(id1).unwrap().unwrap().0,
        0x34
    );
    let id2 = registry
        .get_field::<RecursiveExtension>(id1)
        .unwrap()
        .unwrap()
        .0
        .ext
        .unwrap();
    assert_eq!(
        registry.get_field::<NumExtension2>(id2).unwrap().unwrap().0,
        0x12
    );
    let id3 = registry
        .get_field::<RecursiveExtension>(id2)
        .unwrap()
        .unwrap()
        .0
        .ext
        .unwrap();
    assert_eq!(
        registry.get_field::<NumExtension2>(id3).unwrap().unwrap().0,
        0x55
    );
    assert!(registry
        .get_field::<RecursiveExtension>(id3)
        .unwrap()
        .is_none());

    let mut encoder = PbEncoder::new(heapless::Vec::<u8, 10>::new());
    registry.add_field::<NumExtension2>(id).unwrap().0 = 0x02;
    registry.clear_field::<RecursiveExtension>(id1).unwrap();
    assert!(registry.encode_ext(id, &mut encoder).unwrap());
    let out = encoder.into_inner();
    assert_eq!(out, &[0x08, 0x02, 0x10, 0x08, 0x34]);
    assert_eq!(registry.compute_ext_size(id).unwrap(), out.len());
}

// Check that decode_only macro compiles with minimal imports
mod decode_only {
    use super::{NumExtension1, NumExtension2, NumMsg, RecursiveExtension, RecursiveMsg};

    use micropb::static_extension_registry_decode_only;

    static_extension_registry_decode_only!(
        TestRegistry,
        NumMsg[5] => {
            num: NumExtension1,
        }
        RecursiveMsg[8] => {
            num: NumExtension2,
            msg: RecursiveExtension
        }
    );
}

// Check that encode_only macro compiles with minimal imports
mod encode_only {
    use super::{NumExtension1, NumExtension2, NumMsg, RecursiveExtension, RecursiveMsg};

    use micropb::static_extension_registry_encode_only;

    static_extension_registry_encode_only!(
        TestRegistry,
        NumMsg[5] => {
            num: NumExtension1,
        }
        RecursiveMsg[8] => {
            num: NumExtension2,
            msg: RecursiveExtension
        }
    );
}
