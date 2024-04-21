use micropb::{DecodeError, MessageDecode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/no_config.rs"));
}

#[test]
fn enum_test() {
    assert_eq!(proto::basic::Enum::Zero, proto::basic::Enum(0));
    assert_eq!(proto::basic::Enum::One, proto::basic::Enum(1));
    assert_eq!(proto::basic::Enum::Two, proto::basic::Enum(2));
    assert_eq!(proto::basic::Enum::Two, proto::basic::Enum::default());
    assert_eq!(
        std::mem::size_of::<proto::basic::Enum>(),
        std::mem::size_of::<i32>()
    );
    let _: i32 = proto::basic::Enum(0).0;
}

#[test]
fn basic_msg() {
    let mut basic = proto::basic::BasicTypes::default();
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
    assert_eq!(basic.enumeration, proto::basic::Enum::One); // custom default
    assert_eq!(basic.enumeration(), None);
    assert_eq!(basic.mut_enumeration(), None);

    basic.enumeration = proto::basic::Enum::One;
    basic._has.set_enumeration(true);
    assert!(basic._has.enumeration());
    assert_eq!(basic.enumeration, proto::basic::Enum::One);
    assert_eq!(basic.enumeration(), Some(&proto::basic::Enum::One));
    *basic.mut_enumeration().unwrap() = proto::basic::Enum::Zero;
    assert_eq!(basic.enumeration(), Some(&proto::basic::Enum::Zero));

    basic.set_int32_num(100);
    assert!(basic._has.int32_num());
    assert_eq!(basic.int32_num(), Some(&100));
    basic.clear_int32_num();
    assert!(!basic._has.int32_num());
    assert_eq!(basic.int32_num(), None);
}

#[test]
fn basic_type_check() {
    let basic = proto::basic::BasicTypes::default();
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
    let mut nested = proto::nested::Nested::default();
    nested._has.set_basic(true);
    assert_eq!(nested.basic(), Some(&proto::basic::BasicTypes::default()));
    assert!(nested.inner.is_none());
    nested.inner = Some(proto::nested::mod_Nested::Inner::InnerMsg(
        proto::nested::mod_Nested::InnerMsg::default(),
    ));

    let _: proto::basic::BasicTypes = nested.basic;
    let _: Option<proto::nested::mod_Nested::Inner> = nested.inner;
    match nested.inner.unwrap() {
        proto::nested::mod_Nested::Inner::Scalar(v) => {
            let _: bool = v;
        }
        proto::nested::mod_Nested::Inner::InnerMsg(m) => {
            let _: proto::nested::mod_Nested::InnerMsg = m;
            assert_eq!(m.val, 0);
        }
        proto::nested::mod_Nested::Inner::Enumeration(e) => {
            let _: proto::basic::Enum = e;
        }
        proto::nested::mod_Nested::Inner::InnerEnum(e) => {
            let _: proto::nested::mod_Nested::InnerEnum = e;
        }
    }
}

#[test]
fn proto3() {
    let non_opt = proto::basic3::NonOptional::default();
    let _: i32 = non_opt.non_opt;
    // no hazzer, so message size should equal field size
    assert_eq!(
        std::mem::size_of::<proto::basic3::NonOptional>(),
        std::mem::size_of::<i32>()
    );

    let opt = proto::basic3::Optional::default();
    let _: i32 = opt.opt;
    let _: proto::basic3::ZST = opt.zst_opt;
    let _: proto::basic3::ZST = opt.zst;
    // regardless of whether the ZST is marked as optional, it should be treated as optional
    assert!(opt.zst().is_none());
    assert!(opt.zst_opt().is_none());
    // hazzer exists, so message size should exceed field size
    assert!(std::mem::size_of::<proto::basic3::Optional>() > std::mem::size_of::<i32>());
}

#[test]
fn decode_varint() {
    let mut basic = proto::basic::BasicTypes::new();
    let mut decoder = PbDecoder::new([3, 0x08, 0x96, 0x01].as_slice()); // field 1
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.int32_num(), Some(&150));

    let mut decoder = PbDecoder::new(
        [
            0x10, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.int64_num(), Some(&i64::min_value()));

    let mut decoder = PbDecoder::new(
        [
            0x18, 0x96, 0x01, // field 3
            0x20, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 4
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.uint32_num(), Some(&150));
    assert_eq!(basic.uint64_num(), Some(&u64::max_value()));

    let mut decoder = PbDecoder::new(
        [
            0x28, 0xFE, 0xFF, 0xFF, 0xFF, 0x7F, // field 5
            0x30, 0x01, // field 6
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.sint32_num(), Some(&0x7FFFFFFF));
    assert_eq!(basic.sint64_num(), Some(&-1));
}

#[test]
fn decode_fixed() {
    let mut basic = proto::basic::BasicTypes::new();
    let mut decoder = PbDecoder::new(
        [
            0x39, 0x11, 0x00, 0x00, 0x12, // field 7
            0x41, 0x30, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 8
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
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
    let len = decoder.reader.len();
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
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.boolean(), Some(&true));
    assert_eq!(basic.flt(), Some(&-29.03456));
    assert_eq!(basic.dbl(), Some(&26.029345233467545));
}

#[test]
fn decode_enum() {
    let mut basic = proto::basic::BasicTypes::new();
    let mut decoder = PbDecoder::new([0x70, 0x00].as_slice());
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.enumeration(), Some(&proto::basic::Enum::Zero));

    let mut decoder = PbDecoder::new(
        [
            0x70, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 14
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    basic.decode(&mut decoder, len).unwrap();
    assert_eq!(basic.enumeration(), Some(&proto::basic::Enum(-2)));
}

#[test]
fn decode_nested() {
    let mut nested = proto::nested::Nested::new();
    let mut decoder = PbDecoder::new([0x0A, 0x00].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic(), Some(&Default::default()));

    let mut decoder = PbDecoder::new([0x0A, 0x02, 0x08, 0x01].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic().unwrap().int32_num(), Some(&1));

    let mut decoder = PbDecoder::new([0x0A, 0x02, 0x10, 0x02].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(nested.basic().unwrap().int64_num(), Some(&2));

    let mut decoder = PbDecoder::new([0x10, 0x00].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested::mod_Nested::Inner::Enumeration(0.into())
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
        proto::nested::mod_Nested::Inner::InnerMsg(msg) if msg.val() == Some(&-1) && msg.val2() == Some(&1)
    ));

    let mut decoder = PbDecoder::new([0x20, 0x00].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested::mod_Nested::Inner::InnerEnum(0.into())
    );

    let mut decoder = PbDecoder::new([0x28, 0x00].as_slice());
    let len = decoder.reader.len();
    nested.decode(&mut decoder, len).unwrap();
    assert_eq!(
        nested.inner.as_ref().unwrap(),
        &proto::nested::mod_Nested::Inner::Scalar(false)
    );
}

#[test]
fn decode_non_optional() {
    let mut non_opt = proto::basic3::NonOptional::new();
    let mut decoder = PbDecoder::new([0x08, 0x96, 0x01].as_slice());
    let len = decoder.reader.len();
    non_opt.decode(&mut decoder, len).unwrap();
    assert_eq!(non_opt.non_opt, 150);
}

#[test]
fn decode_errors() {
    let mut basic = proto::basic::BasicTypes::new();
    let mut decoder = PbDecoder::new([0x00, 0x96, 0x01].as_slice()); // field 0
    let len = decoder.reader.len();
    assert_eq!(basic.decode(&mut decoder, len), Err(DecodeError::ZeroField));

    let mut decoder = PbDecoder::new([0x18, 0x96, 0xFF, 0xFF, 0xFF, 0xFF, 0x01].as_slice()); // field 3
    let len = decoder.reader.len();
    assert_eq!(
        basic.decode(&mut decoder, len),
        Err(DecodeError::VarIntLimit(5))
    );

    let mut decoder = PbDecoder::new([0x10, 0x96, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()); // field 2
    let len = decoder.reader.len();
    assert_eq!(
        basic.decode(&mut decoder, len),
        Err(DecodeError::UnexpectedEof)
    );

    let mut decoder = PbDecoder::new([0x10, 0x96, 0x01].as_slice()); // field 2
    assert_eq!(
        basic.decode(&mut decoder, 2),
        Err(DecodeError::WrongLen {
            expected: 2,
            actual: 3
        })
    );
    assert_eq!(decoder.bytes_read(), 3);

    let mut decoder = PbDecoder::new([0x02, 0x10, 0x96, 0x01].as_slice()); // field 2
    assert_eq!(
        basic.decode_len_delimited(&mut decoder),
        Err(DecodeError::WrongLen {
            expected: 2,
            actual: 3
        })
    );
    assert_eq!(decoder.bytes_read(), 4);
}
