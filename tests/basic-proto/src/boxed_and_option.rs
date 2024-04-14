use micropb::{MessageDecode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/boxed_and_option.rs"));
}

#[test]
fn boxed_and_option() {
    let mut basic = proto::basic::BasicTypes::default();

    // Option<Box<bool>>
    assert_eq!(basic.boolean, None);
    assert_eq!(basic.boolean(), None);
    basic.set_boolean(true);
    assert_eq!(basic.boolean, Some(Box::new(true)));
    assert_eq!(basic.boolean(), Some(&true));

    // Option<i32>
    assert_eq!(basic.int32_num, None);
    assert_eq!(basic.int32_num(), None);
    basic.set_int32_num(32);
    assert_eq!(basic.int32_num, Some(32));
    assert_eq!(basic.int32_num(), Some(&32));

    // Box<u32>
    assert_eq!(basic.uint32_num, Box::new(0));
    assert_eq!(basic.uint32_num(), None);
    basic.set_uint32_num(3);
    assert_eq!(basic.uint32_num, Box::new(3));
    assert_eq!(basic.uint32_num(), Some(&3));
    assert!(basic._has.uint32_num());
}

#[test]
fn decode() {
    let mut basic = proto::basic::BasicTypes::default();
    let mut decoder = PbDecoder::new(
        [
            0x58, 0x01, // field 11
            0x08, 0x96, 0x01, // field 1
            0x18, 0x03, // field 3
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.boolean, Some(Box::new(true)));
    assert_eq!(basic.int32_num, Some(150));
    assert_eq!(basic.uint32_num, Box::new(3));
}

#[test]
fn decode_boxed_oneof() {
    let mut nested = proto::nested::Nested::default();

    let mut decoder = PbDecoder::new([0x10, 0x00].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested::mod_Nested::Inner::Enumeration(Box::new(0.into()))
    );

    // Decode the InnerMsg variant twice to make sure the field isn't cleared between decodes
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x08, 0x01].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x10, 0x02].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert!(matches!(
        nested.inner.as_ref().unwrap(),
        proto::nested::mod_Nested::Inner::InnerMsg(msg) if msg.val == Some(Box::new(-1)) && msg.val2() == Some(&1)
    ));
}
