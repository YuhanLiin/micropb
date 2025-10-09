use std::collections::BTreeMap;

use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/boxed_and_option.rs"));
}

#[test]
fn boxed_and_option() {
    let mut basic = proto::basic_::BasicTypes::default();

    // Option<Box<bool>>
    assert_eq!(basic.boolean, None);
    assert_eq!(basic.boolean(), None);
    basic.set_boolean(true);
    assert_eq!(basic.boolean, Some(Box::new(true)));
    assert_eq!(basic.boolean(), Some(&true));
    assert_eq!(basic.take_boolean(), Some(Box::new(true)));
    assert_eq!(basic.take_boolean(), None);

    // Option<i32>
    assert_eq!(basic.int32_num, None);
    assert_eq!(basic.int32_num(), None);
    basic.set_int32_num(32);
    assert_eq!(basic.int32_num, Some(32));
    assert_eq!(basic.int32_num(), Some(&32));
    assert_eq!(basic.take_int32_num(), Some(32));
    assert_eq!(basic.take_int32_num(), None);

    // Box<u32> (hazzer)
    assert_eq!(basic.uint32_num, Box::new(0));
    assert_eq!(basic.uint32_num(), None);
    basic.set_uint32_num(3);
    assert_eq!(basic.uint32_num, Box::new(3));
    assert_eq!(basic.uint32_num(), Some(&3));
    assert!(basic._has.uint32_num());
    assert_eq!(basic.take_uint32_num(), Some(Box::new(3)));
    assert_eq!(basic.take_uint32_num(), None);

    // Box<f32> (non-option)
    assert_eq!(basic.flt, Box::new(1.0));
    assert_eq!(basic.flt(), Some(&1.0));
    basic.set_flt(3.0);
    assert_eq!(basic.flt(), Some(&3.0));

    let nested = proto::nested_::Nested::default();
    assert_eq!(nested.basic, proto::basic_::BasicTypes::default());
    assert_eq!(nested.basic(), Some(&proto::basic_::BasicTypes::default()));
}

#[test]
fn boxed_collections() {
    let mut data = proto::Data::default();

    assert_eq!(data.s(), None);
    data.set_s(String::from("a"));
    assert_eq!(data.s(), Some(&String::from("a")));
    assert_eq!(data.s, Some(Box::new(String::from("a"))));

    assert_eq!(data.b(), None);
    data.set_b(vec![]);
    assert_eq!(data.b(), Some(&vec![]));
    assert_eq!(data.b, Box::default());

    let list = proto::List::default();
    let _: Box<Vec<proto::Data>> = list.list;

    let numlist = proto::NumList::default();
    let _: Box<Vec<u32>> = numlist.list;

    let strlist = proto::StrList::default();
    let _: Box<Vec<String>> = strlist.list;

    let fixedlist = proto::FixedList::default();
    let _: Box<Vec<u32>> = fixedlist.list;

    let map = proto::Map::default();
    let _: Box<BTreeMap<String, Vec<u8>>> = map.mapping;
}

#[test]
fn decode() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new(
        [
            0x58, 0x01, // field 11
            0x08, 0x96, 0x01, // field 1
            0x18, 0x03, // field 3
            0x65, 0x00, 0x00, 0x00, 0x00, // field 12
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.boolean, Some(Box::new(true)));
    assert_eq!(basic.int32_num, Some(150));
    assert_eq!(basic.uint32_num, Box::new(3));
    // Even though we're decoding a zero, it should still go through because this field doesn't use
    // implicit presence
    assert_eq!(basic.flt, Box::new(0.0));
}

#[test]
fn encode() {
    let mut basic = proto::basic_::BasicTypes::default();
    // The flt field is always-on, so the size will start at 5 bytes
    assert_eq!(basic.compute_size(), 5);
    basic.boolean = Some(Box::new(true));
    assert_eq!(basic.compute_size(), 7);
    basic.int32_num = Some(150);
    assert_eq!(basic.compute_size(), 10);
    basic.flt = Box::new(0.0);

    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[
            0x08, 0x96, 0x01, // field 1
            0x58, 0x01, // field 11
            0x65, 0x00, 0x00, 0x00, 0x00, // field 12
        ]
    );
}

#[test]
fn decode_boxed_oneof() {
    let mut nested = proto::nested_::Nested::default();

    let mut decoder = PbDecoder::new([0x10, 0x00].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested_::Nested_::Inner::Enumeration(Box::new(0.into()))
    );

    // Decode the InnerMsg variant twice to make sure the field isn't cleared between decodes
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x08, 0x01].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    let mut decoder = PbDecoder::new([0x1A, 0x02, 0x10, 0x02].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert!(matches!(
        nested.inner.as_ref().unwrap(),
        proto::nested_::Nested_::Inner::InnerMsg(msg) if msg.val == Some(Box::new(-1)) && msg.val2() == Some(&1)
    ));
}
