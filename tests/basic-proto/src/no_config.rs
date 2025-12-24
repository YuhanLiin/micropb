use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/no_config.rs"));
}

#[test]
fn enum_test() {
    assert_eq!(proto::basic_::Enum::Zero, proto::basic_::Enum(0));
    assert_eq!(proto::basic_::Enum::One, proto::basic_::Enum(1));
    assert_eq!(proto::basic_::Enum::Two, proto::basic_::Enum(2));
    assert_eq!(proto::basic_::Enum::Two, proto::basic_::Enum::default());
    assert_eq!(
        std::mem::size_of::<proto::basic_::Enum>(),
        std::mem::size_of::<i32>()
    );
    let _: i32 = proto::basic_::Enum(0).0;
}

#[test]
fn basic_msg() {
    let mut basic = proto::basic_::BasicTypes::default();
    assert!(!basic._has.dbl());
    assert_eq!(basic.dbl, 0.0);
    assert_eq!(basic.dbl(), None);
    assert_eq!(basic.mut_dbl(), None);

    assert!(!basic._has.flt());
    assert_eq!(basic.flt, 1.0); // custom default
    assert_eq!(basic.flt(), None);
    assert_eq!(basic.mut_flt(), None);

    assert!(!basic._has.boolean());
    assert!(!basic.boolean);
    assert_eq!(basic.boolean(), None);
    assert_eq!(basic.mut_boolean(), None);

    assert!(!basic._has.int32_num());
    assert_eq!(basic.int32_num, -5); // custom default
    assert_eq!(basic.int32_num(), None);
    assert_eq!(basic.mut_int32_num(), None);

    assert!(!basic._has.int64_num());
    assert_eq!(basic.int64_num, 0);
    assert_eq!(basic.int64_num(), None);
    assert_eq!(basic.mut_int64_num(), None);

    assert!(!basic._has.enumeration());
    assert_eq!(basic.enumeration, proto::basic_::Enum::One); // custom default
    assert_eq!(basic.enumeration(), None);
    assert_eq!(basic.mut_enumeration(), None);

    basic.enumeration = proto::basic_::Enum::One;
    basic._has.set_enumeration();
    assert!(basic._has.enumeration());
    assert_eq!(basic.enumeration, proto::basic_::Enum::One);
    assert_eq!(basic.enumeration(), Some(&proto::basic_::Enum::One));
    *basic.mut_enumeration().unwrap() = proto::basic_::Enum::Zero;
    assert_eq!(basic.enumeration(), Some(&proto::basic_::Enum::Zero));

    basic.set_int32_num(100);
    assert!(basic._has.int32_num());
    assert_eq!(basic.int32_num(), Some(&100));
    basic.clear_int32_num();
    assert!(!basic._has.int32_num());
    assert_eq!(basic.int32_num(), None);
}

#[test]
fn chain_init() {
    let basic = proto::basic_::BasicTypes::default()
        .init_boolean(true)
        .init_int32_num(32)
        .init_fixed32_num(10);
    assert_eq!(basic.boolean(), Some(&true));
    assert_eq!(basic.int32_num(), Some(&32));
    assert_eq!(basic.fixed32_num(), Some(&10));
    assert_eq!(basic.dbl(), None);
}

#[test]
fn basic_type_check() {
    let basic = proto::basic_::BasicTypes::default();
    let _: i32 = basic.int32_num;
    let _: i64 = basic.int64_num;
    let _: u32 = basic.uint32_num;
    let _: u64 = basic.uint64_num;
    let _: i32 = basic.sint32_num;
    let _: i64 = basic.sint64_num;
    let _: u32 = basic.fixed32_num;
    let _: u64 = basic.fixed64_num;
    let _: i32 = basic.sfixed32_num;
    let _: i64 = basic.sfixed64_num;
    let _: bool = basic.boolean;
    let _: f32 = basic.flt;
    let _: f64 = basic.dbl;
}

#[test]
fn nested_msg() {
    let mut nested = proto::nested_::Nested::default();
    nested._has.set_basic();
    assert_eq!(nested.basic(), Some(&proto::basic_::BasicTypes::default()));
    assert!(nested.inner.is_none());
    nested.inner = Some(proto::nested_::Nested_::Inner::InnerMsg(
        proto::nested_::Nested_::InnerMsg::default(),
    ));

    let _: proto::basic_::BasicTypes = nested.basic;
    let _: Option<proto::nested_::Nested_::Inner> = nested.inner;
    match nested.inner.unwrap() {
        proto::nested_::Nested_::Inner::Scalar(v) => {
            let _: bool = v;
        }
        proto::nested_::Nested_::Inner::InnerMsg(m) => {
            let _: proto::nested_::Nested_::InnerMsg = m;
            assert_eq!(m.val, 0);
        }
        proto::nested_::Nested_::Inner::Enumeration(e) => {
            let _: proto::basic_::Enum = e;
        }
        proto::nested_::Nested_::Inner::InnerEnum(e) => {
            let _: proto::nested_::Nested_::InnerEnum = e;
        }
    }
}

#[test]
fn proto3() {
    let non_opt = proto::basic3_::NonOptional::default();
    let _: i32 = non_opt.non_opt;
    // no hazzer, so message size should equal field size
    assert_eq!(
        std::mem::size_of::<proto::basic3_::NonOptional>(),
        std::mem::size_of::<i32>()
    );

    let opt = proto::basic3_::Optional::default();
    let _: i32 = opt.opt;
    let _: proto::basic3_::ZST = opt.zst_opt;
    let _: proto::basic3_::ZST = opt.zst;
    // regardless of whether the ZST is marked as optional, it should be treated as optional
    assert!(opt.zst().is_none());
    assert!(opt.zst_opt().is_none());
    // hazzer exists, so message size should exceed field size
    assert!(std::mem::size_of::<proto::basic3_::Optional>() > std::mem::size_of::<i32>());
}

#[test]
fn partial_eq() {
    // PartialEq with singular fields
    let non_opt1 = proto::basic3_::NonOptional::default();
    let mut non_opt2 = proto::basic3_::NonOptional::default();
    assert_eq!(non_opt1, non_opt2);
    non_opt2.non_opt = 12;
    assert_ne!(non_opt1, non_opt2);

    // PartialEq with Hazzers
    let mut basic1 = proto::basic_::BasicTypes::default();
    let mut basic2 = proto::basic_::BasicTypes::default();
    assert_eq!(basic1, basic2);
    basic2.int32_num = 12;
    // int32_num has no bearing on equality if the Hazzer bit is off
    assert_eq!(basic1, basic2);
    basic2._has.set_int32_num();
    assert_ne!(basic1, basic2);
    basic1._has.set_int32_num();
    assert_ne!(basic1, basic2);

    // PartialEq with oneof fields
    let mut nested1 = proto::nested_::Nested::default();
    let mut nested2 = proto::nested_::Nested::default();
    assert_eq!(nested1, nested2);
    nested1.inner = Some(proto::nested_::Nested_::Inner::InnerMsg(
        proto::nested_::Nested_::InnerMsg::default(),
    ));
    assert_ne!(nested1, nested2);
    nested2.inner = Some(proto::nested_::Nested_::Inner::InnerMsg(
        proto::nested_::Nested_::InnerMsg::default(),
    ));
    assert_eq!(nested1, nested2);
}

#[test]
fn decode_varint() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new([3, 0x08, 0x96, 0x01].as_slice()); // field 1
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.int32_num(), Some(&150));

    let mut decoder = PbDecoder::new(
        [
            0x10, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.int64_num(), Some(&i64::MIN));

    let mut decoder = PbDecoder::new(
        [
            0x18, 0x96, 0x01, // field 3
            0x20, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 4
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.uint32_num(), Some(&150));
    assert_eq!(basic.uint64_num(), Some(&u64::MAX));

    let mut decoder = PbDecoder::new(
        [
            0x28, 0xFE, 0xFF, 0xFF, 0xFF, 0x7F, // field 5
            0x30, 0x01, // field 6
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.sint32_num(), Some(&0x7FFFFFFF));
    assert_eq!(basic.sint64_num(), Some(&-1));
}

#[test]
fn encode_varint() {
    let mut basic = proto::basic_::BasicTypes::default();
    assert_eq!(basic.compute_size(), 0);
    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert!(encoder.into_writer().is_empty());

    basic.set_int32_num(1);
    assert_eq!(basic.compute_size(), 2);
    basic.set_int64_num(-1);
    assert_eq!(basic.compute_size(), 13);
    basic.set_uint32_num(150);
    assert_eq!(basic.compute_size(), 16);
    basic.set_uint64_num(0);
    assert_eq!(basic.compute_size(), 18);
    basic.set_sint32_num(-1);
    assert_eq!(basic.compute_size(), 20);

    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[
            0x08, 0x01, // field 1
            0x10, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 2
            0x18, 0x96, 0x01, // field 3
            0x20, 0x00, // field 4
            0x28, 0x01, // field 5
        ]
    );
}

#[test]
fn decode_fixed() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new(
        [
            0x39, 0x11, 0x00, 0x00, 0x12, // field 7
            0x41, 0x30, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 8
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.fixed32_num(), Some(&0x12000011));
    assert_eq!(basic.fixed64_num(), Some(&0x0130));

    let mut decoder = PbDecoder::new(
        [
            0x49, 0x12, 0x32, 0x98, 0xF4, // field 9
            0x51, 0x30, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 10
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.sfixed32_num(), Some(&-0x0B67CDEE));
    assert_eq!(basic.sfixed64_num(), Some(&0x0130));

    let mut decoder = PbDecoder::new(
        [
            0x58, 0x01, // field 11
            0x61, 0xC7, 0x46, 0xE8, 0xC1, // field 12
            0x69, 0x5E, 0x09, 0x52, 0x2B, 0x83, 0x07, 0x3A, 0x40, // field 13
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.boolean(), Some(&true));
    assert_eq!(basic.flt(), Some(&-29.03456));
    assert_eq!(basic.dbl(), Some(&26.029345233467545));
}

#[test]
fn encode_fixed() {
    let mut basic = proto::basic_::BasicTypes::default();
    basic.set_fixed32_num(0);
    assert_eq!(basic.compute_size(), 5);
    basic.set_fixed64_num(0xABCDEF);
    assert_eq!(basic.compute_size(), 14);
    basic.set_sfixed32_num(-10);
    assert_eq!(basic.compute_size(), 19);
    basic.set_sfixed64_num(-9);
    assert_eq!(basic.compute_size(), 28);
    basic.set_flt(-29.03456);
    assert_eq!(basic.compute_size(), 33);
    basic.set_dbl(26.029345233467545);
    assert_eq!(basic.compute_size(), 42);
    basic.set_boolean(true);
    assert_eq!(basic.compute_size(), 44);

    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[
            0x3D, 0x00, 0x00, 0x00, 0x00, // field 7
            0x41, 0xEF, 0xCD, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, // field 8
            0x4D, 0xF6, 0xFF, 0xFF, 0xFF, // field 9
            0x51, 0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // field 10
            0x58, 0x01, // field 13
            0x65, 0xC7, 0x46, 0xE8, 0xC1, // field 11
            0x69, 0x5E, 0x09, 0x52, 0x2B, 0x83, 0x07, 0x3A, 0x40, // field 12
        ]
    );
}

#[test]
fn decode_enum() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new([0x70, 0x00].as_slice());
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.enumeration(), Some(&proto::basic_::Enum::Zero));

    let mut decoder = PbDecoder::new(
        [
            0x70, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 14
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.enumeration(), Some(&proto::basic_::Enum(-2)));
}

#[test]
fn encode_enum() {
    let mut basic = proto::basic_::BasicTypes::default();
    basic.set_enumeration(proto::basic_::Enum::Two);
    assert_eq!(basic.compute_size(), 2);
    basic.set_enumeration(proto::basic_::Enum(130));
    assert_eq!(basic.compute_size(), 3);

    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x70, 0x82, 0x01]);
}

#[test]
fn decode_nested() {
    let mut nested = proto::nested_::Nested::default();
    let mut decoder = PbDecoder::new([0x0A, 0x00].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic(), Some(&Default::default()));

    let mut decoder = PbDecoder::new([0x0A, 0x02, 0x08, 0x01].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic().unwrap().int32_num(), Some(&1));

    let mut decoder = PbDecoder::new([0x0A, 0x02, 0x10, 0x02].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic().unwrap().int64_num(), Some(&2));

    let mut decoder = PbDecoder::new([0x10, 0x00].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested_::Nested_::Inner::Enumeration(0.into())
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
        proto::nested_::Nested_::Inner::InnerMsg(msg) if msg.val() == Some(&-1) && msg.val2() == Some(&1)
    ));

    let mut decoder = PbDecoder::new([0x20, 0x00].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested_::Nested_::Inner::InnerEnum(0.into())
    );

    let mut decoder = PbDecoder::new([0x28, 0x00].as_slice());
    let len = decoder.as_reader().len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested_::Nested_::Inner::Scalar(false)
    );
}

#[test]
fn encode_nested() {
    let mut nested = proto::nested_::Nested::default();
    nested._has.set_basic();
    assert_eq!(nested.compute_size(), 2);
    nested.basic.set_int32_num(14);
    assert_eq!(nested.compute_size(), 4);
    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x0A, 2, 0x08, 0x0E]);

    nested.clear_basic();
    assert_eq!(nested.compute_size(), 0);

    nested.inner = Some(proto::nested_::Nested_::Inner::InnerMsg({
        let mut msg = proto::nested_::Nested_::InnerMsg::default();
        msg.set_val(-1);
        msg.set_val2(-3);
        msg
    }));
    assert_eq!(nested.compute_size(), 6);
    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x1A, 4, 0x08, 1, 0x10, 5]);

    nested.inner = Some(proto::nested_::Nested_::Inner::InnerEnum(0.into()));
    assert_eq!(nested.compute_size(), 2);
    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x20, 0x00]);

    nested.inner = Some(proto::nested_::Nested_::Inner::Scalar(false));
    assert_eq!(nested.compute_size(), 2);
    let mut encoder = PbEncoder::new(vec![]);
    nested.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x28, 0x00]);
}

#[test]
fn decode_non_optional() {
    let mut non_opt = proto::basic3_::NonOptional::default();
    let mut decoder = PbDecoder::new([0x08, 0x96, 0x01].as_slice());
    let len = decoder.as_reader().len();
    non_opt.decode(&mut decoder, len).unwrap();
    assert_eq!(non_opt.non_opt, 150);
}

#[test]
fn encode_non_optional() {
    let mut non_opt = proto::basic3_::NonOptional::default();
    assert_eq!(non_opt.compute_size(), 0);
    let mut encoder = PbEncoder::new(vec![]);
    non_opt.encode(&mut encoder).unwrap();
    assert!(encoder.into_writer().is_empty());

    non_opt.non_opt = 150;
    assert_eq!(non_opt.compute_size(), 3);
    let mut encoder = PbEncoder::new(vec![]);
    non_opt.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x08, 0x96, 0x01]);
}

#[test]
fn decode_errors() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new([0x00, 0x96, 0x01].as_slice()); // field 0
    let len = decoder.as_reader().len();
    assert_eq!(basic.decode(&mut decoder, len), Err(DecodeError::ZeroField));

    let mut decoder = PbDecoder::new(
        [
            0x18, 0x96, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 3
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    assert_eq!(
        basic.decode(&mut decoder, len),
        Err(DecodeError::VarIntLimit)
    );

    let mut decoder = PbDecoder::new([0x10, 0x96, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()); // field 2
    let len = decoder.as_reader().len();
    assert_eq!(
        basic.decode(&mut decoder, len),
        Err(DecodeError::UnexpectedEof)
    );

    let mut decoder = PbDecoder::new([0x02, 0x10, 0x96, 0x01].as_slice()); // field 2
    assert_eq!(
        basic.decode_len_delimited(&mut decoder),
        Err(DecodeError::WrongLen)
    );
    assert_eq!(decoder.bytes_read(), 4);
}

#[test]
fn max_size() {
    let basic_max_size =
        (14/* tags */) + 10 + 10 + 5 + 10 + 5 + 10 + 4 + 8 + 4 + 8 + 1 + 4 + 8 + 10;
    assert_eq!(proto::basic_::BasicTypes::MAX_SIZE, Ok(basic_max_size));

    let optional_max_size = (3/* tags */) + 10 + 1 + 1;
    assert_eq!(proto::basic3_::Optional::MAX_SIZE, Ok(optional_max_size));
    assert_eq!(proto::basic3_::ZST::MAX_SIZE, Ok(0));

    let inner_max_size = (2/* tags */) + 5 + 5;
    assert_eq!(
        proto::nested_::Nested_::InnerMsg::MAX_SIZE,
        Ok(inner_max_size)
    );

    let nested_max_size = (2/* tags */) + (1 + basic_max_size) + (1 + inner_max_size);
    assert_eq!(proto::nested_::Nested::MAX_SIZE, Ok(nested_max_size));
}
