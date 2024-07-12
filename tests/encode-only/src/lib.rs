#![no_std]

#[cfg(test)]
use micropb::MessageEncode;

mod example {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/encode_only.rs"));
}

#[cfg(test)]
#[test]
fn encode() {
    let ex = example::Msg {
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
    };

    let mut output = micropb::heapless::Vec::<_, 100>::new();
    let mut encoder = micropb::PbEncoder::new(&mut output);
    ex.encode(&mut encoder).unwrap();

    assert_eq!(
        output,
        &[
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
    );
}
