#![no_std]

#[cfg(test)]
use micropb::MessageDecode;

mod example {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/decode_only.rs"));
}

#[cfg(test)]
#[test]
fn decode() {
    let data = [
        0x08, 0x0C, // field 1
        0x10, 0xA7, 0xCB, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, // field 2
        0x18, 0x39, // field 3
        0x20, 0x8F, 0x4E, // field 4
        0x28, 0x43, // field 5
        0x30, 0xF4, 0x0D, // field 6
        0x38, 0x01, // field 7
        0x45, 0x64, 0x00, 0x00, 0x00, // field 8
        0x49, 0xD8, 0x59, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 9
        0x55, 0x85, 0xFF, 0xFF, 0xFF, // field 10
        0x59, 0x4D, 0xEA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // field 11
    ]
    .as_slice();
    let mut decoder = micropb::PbDecoder::new(data);

    let mut ex = example::Msg::default();
    ex.decode(&mut decoder, data.len()).unwrap();

    assert_eq!(
        ex,
        example::Msg {
            f_int32: 12,
            f_int64: -6745,
            f_uint32: 57,
            f_uint64: 9999,
            f_sint32: -34,
            f_sint64: 890,
            f_bool: true,
            f_fixed32: 100,
            f_fixed64: 23000,
            f_sfixed32: -123,
            f_sfixed64: -5555,
        }
    );
}

#[cfg(test)]
#[test]
fn fixed64_as_32() {
    let data = [
        0x49, 0xD8, 0x59, 0x23, 0x43, 0x99, 0x11, 0x43, 0x12, // field 9
        0x59, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, // field 11
    ]
    .as_slice();

    let mut decoder = micropb::PbDecoder::new(data);

    let mut ex = example::Msg::default();
    ex.decode(&mut decoder, data.len()).unwrap();

    // Since these fields are actually 32 bits, the 64-bit wire values should truncate
    assert_eq!(
        ex,
        example::Msg {
            f_fixed64: 0x432359D8,
            f_sfixed64: -1,
            ..Default::default()
        }
    );
}
