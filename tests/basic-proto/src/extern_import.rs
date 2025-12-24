use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/import_basic.rs"));
    include!(concat!(env!("OUT_DIR"), "/import_nested.rs"));
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Empty;

impl MessageEncode for Empty {
    const MAX_SIZE: Result<usize, &'static str> = Ok(0);

    fn encode<W: micropb::PbWrite>(&self, _encoder: &mut PbEncoder<W>) -> Result<(), W::Error> {
        Ok(())
    }

    fn compute_size(&self) -> usize {
        0
    }
}

impl MessageDecode for Empty {
    fn decode<R: micropb::PbRead>(
        &mut self,
        decoder: &mut PbDecoder<R>,
        len: usize,
    ) -> Result<(), DecodeError<R::Error>> {
        decoder.skip_bytes(len)
    }
}

#[test]
#[allow(unused)]
fn imported_types() {
    let mut nested = proto::nested_::Nested::default();
    let _basic: Empty = nested.basic;
    nested.inner = Some(proto::nested_::Nested_::Inner::Enumeration(
        proto::basic_::Enum(0),
    ));
}

#[test]
fn encode_imported() {
    let mut nested = proto::nested_::Nested::default();
    nested._has.set_basic();
    assert_eq!(nested.compute_size(), 2);

    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x0A, 0]);
}

#[test]
fn decode_imported() {
    let mut nested = proto::nested_::Nested::default();
    let mut decoder = PbDecoder::new([0x0A, 0].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert!(nested._has.basic());
}

#[test]
fn max_size() {
    let inner_max_size = (2/* tags */) + 5 + 5;
    let nested_max_size = (2/* tags */) + 1/* empty msg */ + (1 + inner_max_size);
    assert_eq!(proto::nested_::Nested::MAX_SIZE, Ok(nested_max_size));
}
