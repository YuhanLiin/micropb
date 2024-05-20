use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/unsigned_enum.rs"));
}

#[test]
fn decode_enum() {
    let mut basic = proto::basic::BasicTypes::default();
    let mut decoder = PbDecoder::new([0x70, 0x00].as_slice());
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.enumeration(), Some(&proto::basic::Enum::Zero));

    let mut decoder = PbDecoder::new(
        [
            0x70, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 14
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    // Since we're only decoding unsigned ints, decoding long varints result in errors
    assert_eq!(
        basic.decode(&mut decoder, len),
        Err(DecodeError::VarIntLimit(5))
    );
}

#[test]
fn encode_enum() {
    let mut basic = proto::basic::BasicTypes::default();

    basic.set_enumeration(proto::basic::Enum(130));
    assert_eq!(basic.compute_size(), 3);
    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x70, 0x82, 0x01]);

    // Will be encoded as 0xFFFFFFFF
    basic.set_enumeration(proto::basic::Enum(-1));
    assert_eq!(basic.compute_size(), 6);
    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x70, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
}
