use std::mem::size_of;

use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/int_type.rs"));
}

#[test]
fn enum_int_type() {
    let enumeration = proto::basic_::Enum::One;
    let _: i8 = enumeration.0;
    assert_eq!(size_of::<proto::basic_::Enum>(), 1);
}

#[test]
fn field_int_type() {
    let basic = proto::basic_::BasicTypes::default();
    assert_eq!(basic.int32_num, -5i8);
    let _: i8 = basic.int32_num;
    let _: Option<&i8> = basic.int32_num();
    let _: i16 = basic.int64_num;
    let _: Option<&i16> = basic.int64_num();
    let _: u8 = basic.uint32_num;
    let _: Option<&u8> = basic.uint32_num();
    let _: u16 = basic.uint64_num;
    let _: Option<&u16> = basic.uint64_num();
    let _: i64 = basic.sfixed32_num;
    let _: Option<&i64> = basic.sfixed32_num();
    let _: i32 = basic.sfixed64_num;
    let _: Option<&i32> = basic.sfixed64_num();
    let _: u64 = basic.fixed32_num;
    let _: Option<&u64> = basic.fixed32_num();
    let _: u32 = basic.fixed64_num;
    let _: Option<&u32> = basic.fixed64_num();
}

#[test]
fn decode_int_overflow() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new([0x03, 0x08, 0x96, 0x01].as_slice()); // field 1
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.int32_num(), Some(&-106)); // 150 overflows i8

    let mut decoder = PbDecoder::new([0x03, 0x18, 0x96, 0x02].as_slice()); // field 3
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.uint32_num(), Some(&22)); // 278 overflows u8
}

#[test]
fn encode() {
    let mut basic = proto::basic_::BasicTypes::default();
    basic.set_int32_num(-1);
    assert_eq!(basic.compute_size(), 11);
    // Regardless of the int type, fixed numbers have fixed size
    basic.set_sfixed32_num(i64::min_value());
    assert_eq!(basic.compute_size(), 16);

    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[
            0x08, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 1
            0x4D, 0x00, 0x00, 0x00, 0x00 // field 9
        ]
    );
}

#[test]
fn decode_64_as_32() {
    let mut basic = proto::basic_::BasicTypes::default();
    let mut decoder = PbDecoder::new(
        [
            0x10, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 2
            0x20, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 4
            0x30, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 6
            0x41, 0xAB, 0xCD, 0xEF, 0x01, 0x00, 0x00, 0x00, 0x00, // field 8
            0x51, 0x11, 0xCD, 0xEF, 0x01, 0x00, 0x00, 0x00, 0x00, // field 10
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    basic.decode(&mut decoder, len).unwrap();

    assert_eq!(basic.int64_num, -2);
    assert_eq!(basic.uint64_num, 0xFFFF);
    assert_eq!(basic.sint64_num, 0x7FFFFFFF); // Ignore all varint bytes after the 5th one
    assert_eq!(basic.fixed64_num, 0x01EFCDAB);
    assert_eq!(basic.sfixed64_num, 0x01EFCD11);
}

#[test]
fn encode_64_as_32() {
    let mut basic = proto::basic_::BasicTypes::default();
    basic.set_int64_num(-2);
    basic.set_uint64_num(5);
    basic.set_sint64_num(-0x80000000); // Should only write 5 bytes, since the int is 32 bits
    basic.set_fixed64_num(0xABCDEF01);
    basic.set_sfixed64_num(0xAA);

    let exp = [
        0x10, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 2
        0x20, 0x05, // field 4
        0x30, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F, // field 6
        0x41, 0x01, 0xEF, 0xCD, 0xAB, 0x00, 0x00, 0x00, 0x00, // field 8
        0x51, 0xAA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 10
    ];
    assert_eq!(basic.compute_size(), exp.len());
    let mut encoder = PbEncoder::new(vec![]);
    basic.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &exp);
}
