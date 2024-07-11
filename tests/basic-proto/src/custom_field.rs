use std::mem::size_of;

use micropb::{
    size::sizeof_tag, FieldDecode, FieldEncode, MessageDecode, MessageEncode, PbDecoder, PbEncoder,
    Tag, WIRE_TYPE_I32,
};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/custom_field.rs"));
}

#[derive(Default)]
struct MockField {
    tags: Vec<Tag>,
}

impl FieldDecode for MockField {
    fn decode_field<R: micropb::PbRead>(
        &mut self,
        tag: Tag,
        decoder: &mut micropb::PbDecoder<R>,
    ) -> Result<bool, micropb::DecodeError<R::Error>> {
        if tag.wire_type() >= WIRE_TYPE_I32 {
            // Case where `decode_field` returns false
            return Ok(false);
        }
        decoder.skip_wire_value(tag.wire_type())?;
        self.tags.push(tag);
        Ok(true)
    }
}

// All this impl does is write out all the tags as varints
impl FieldEncode for MockField {
    fn encode_fields<W: micropb::PbWrite>(
        &self,
        encoder: &mut micropb::PbEncoder<W>,
    ) -> Result<(), W::Error> {
        for tag in &self.tags {
            encoder.encode_tag(*tag)?;
        }
        Ok(())
    }

    fn compute_fields_size(&self) -> usize {
        self.tags.iter().copied().map(sizeof_tag).sum()
    }
}

#[test]
fn type_check() {
    let nested = proto::nested_::Nested::default();
    // custom_inner + _unknown, which are both MockField
    assert_eq!(
        size_of::<proto::nested_::Nested>(),
        size_of::<MockField>() * 2
    );
    let _: MockField = nested.custom_inner;

    let list = proto::List::default();
    // only one MockField
    assert_eq!(size_of::<proto::List>(), size_of::<MockField>());
    let _: MockField = list.list;
}

#[test]
fn decode_custom_fields() {
    let mut nested = proto::nested_::Nested::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 0x02, 0x08, 0x01, // Field 1
            0x1A, 0x02, 0x08, 0x01, // Field 3 (oneof)
            0x20, 0x00, // Field 4 (oneof)
            0x28, 0x00, // Field 5 (oneof)
            0x30, 0x00, // Field 6 (unknown, shouldn't be included)
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.custom_inner.tags.len(), 4);
    assert_eq!(nested.custom_inner.tags[0], Tag::from_parts(1, 2));
    assert_eq!(nested.custom_inner.tags[1], Tag::from_parts(3, 2));
    assert_eq!(nested.custom_inner.tags[2], Tag::from_parts(4, 0));
    assert_eq!(nested.custom_inner.tags[3], Tag::from_parts(5, 0));
}

#[test]
fn encode_custom_fields() {
    let mut nested = proto::nested_::Nested::default();
    assert_eq!(nested.compute_size(), 0);
    nested.custom_inner.tags.push(Tag::from_parts(1, 2));
    nested.custom_inner.tags.push(Tag::from_parts(3, 2));
    nested.custom_inner.tags.push(Tag::from_parts(4, 0));
    assert_eq!(nested.compute_size(), 3);

    nested._unknown.tags.push(Tag::from_parts(6, 2));
    assert_eq!(nested.compute_size(), 4);

    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0xA, 0x1A, 0x20, 0x32]);
}

#[test]
fn decode_unknown() {
    let mut nested = proto::nested_::Nested::default();
    let mut decoder = PbDecoder::new(
        [
            0x30, 0x00, // Field 6
            0x38, 0x00, // Field 7
            0x45, 0x00, 0x00, 0x00, 0x00, // Field 8 with I32, which should be skipped
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested._unknown.tags.len(), 2);
    assert_eq!(nested._unknown.tags[0], Tag::from_parts(6, 0));
    assert_eq!(nested._unknown.tags[1], Tag::from_parts(7, 0));
    assert!(nested.custom_inner.tags.is_empty());
}

#[test]
fn decode_custom_repeated() {
    let mut list = proto::List::default();
    let mut decoder = PbDecoder::new(
        [
            0x08, 0x01, // Field 1
            0x08, 0x02, // Field 1
            0x08, 0x01, // Field 1
            0x38, 0x00, // Field 7 (unknown, should be skipped)
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.tags.len(), 3);
    assert_eq!(list.list.tags[0], Tag::from_parts(1, 0));
    assert_eq!(list.list.tags[1], Tag::from_parts(1, 0));
    assert_eq!(list.list.tags[2], Tag::from_parts(1, 0));
}

#[test]
fn decode_custom_error() {
    let mut list = proto::List::default();
    // Field 1 with I32 wire type, which should trigger a custom handler error
    let mut decoder = PbDecoder::new([0x0D, 0x00, 0x00, 0x00, 0x00].as_slice());
    let len = decoder.as_reader().len();
    assert_eq!(
        list.decode(&mut decoder, len),
        Err(micropb::DecodeError::CustomField)
    );
}
