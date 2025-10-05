use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/single_oneof.rs"));
}

#[test]
fn typecheck_normal_oneof() {
    let nested = proto::nested_::Nested::default();
    let _: proto::basic_::BasicTypes = nested.basic;
    let _: Option<proto::nested_::Nested_::Inner> = nested.inner;
}

#[test]
fn decode_single_oneof() {
    let mut oneof = proto::SingleOneof::default();
    assert_eq!(oneof, proto::SingleOneof::None);

    // Unknown field
    let mut decoder = PbDecoder::new([0x0A, 0x00].as_slice());
    let len = decoder.as_reader().len();
    oneof.decode(&mut decoder, len).unwrap();
    assert_eq!(oneof, proto::SingleOneof::None);

    // Decode the InnerMsg variant twice to make sure the field isn't cleared between decodes
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x08, 0x01].as_slice());
    let len = decoder.as_reader().len();
    oneof.decode(&mut decoder, len).unwrap();
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x10, 0x02].as_slice());
    let len = decoder.as_reader().len();
    oneof.decode(&mut decoder, len).unwrap();
    let proto::SingleOneof::InnerMsg(ref msg) = &oneof else {
        panic!("unexpected variant")
    };
    let _: &Box<_> = msg;
    assert!(msg.val() == Some(&-1) && msg.val2() == Some(&1));

    let mut decoder = PbDecoder::new([0x20, 0x00].as_slice());
    let len = decoder.as_reader().len();
    oneof.decode(&mut decoder, len).unwrap();
    assert_eq!(oneof, proto::SingleOneof::InnerEnum(0.into()));
}

#[test]
fn encode_single_oneof() {
    let mut oneof = proto::SingleOneof::default();
    assert_eq!(oneof.compute_size(), 0);

    oneof = proto::SingleOneof::InnerMsg({
        let mut msg = proto::SingleOneof_::InnerMsg::default();
        msg.set_val(-1);
        msg.set_val2(-3);
        Box::new(msg)
    });
    assert_eq!(oneof.compute_size(), 6);
    let mut encoder = PbEncoder::new(vec![]);
    oneof.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x1A, 4, 0x08, 1, 0x10, 5]);

    oneof = proto::SingleOneof::InnerEnum(0.into());
    assert_eq!(oneof.compute_size(), 2);
    let mut encoder = PbEncoder::new(vec![]);
    oneof.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x20, 0x00]);
}
