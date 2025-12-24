use micropb::{MessageDecode, MessageEncode, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/large_field_nums.rs"));
}

#[test]
fn encode() {
    let msg = proto::Msg {
        a: 150,
        of: Some(proto::Msg_::Of::B(5)),
    };
    let mut encoder = PbEncoder::new(vec![]);
    msg.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[0x90, 0x03, 0x96, 0x01, 0xb8, 0xf0, 0x10, 0x05]
    );
    assert_eq!(msg.compute_size(), 8);

    let msg = proto::Msg {
        a: 150,
        of: Some(proto::Msg_::Of::C(5)),
    };
    let mut encoder = PbEncoder::new(vec![]);
    msg.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[0x90, 0x03, 0x96, 0x01, 0xf8, 0xff, 0xff, 0xff, 0x0f, 0x05]
    );
    assert_eq!(msg.compute_size(), 10);
}

#[test]
fn decode() {
    let mut msg = proto::Msg::default();
    msg.decode_from_bytes(&[0x90, 0x03, 0x96, 0x01, 0xb8, 0xf0, 0x10, 0x05])
        .unwrap();
    assert_eq!(msg.a, 150);
    assert_eq!(msg.of, Some(proto::Msg_::Of::B(5)));

    let mut msg = proto::Msg::default();
    msg.decode_from_bytes(&[0x90, 0x03, 0x96, 0x01, 0xf8, 0xff, 0xff, 0xff, 0x0f, 0x05])
        .unwrap();
    assert_eq!(msg.a, 150);
    assert_eq!(msg.of, Some(proto::Msg_::Of::C(5)));
}

#[test]
fn max_size() {
    let max_size = 2/* tag */ + 10 + 5/* tag */ + 10;
    assert_eq!(proto::Msg::MAX_SIZE, Ok(max_size));
}
