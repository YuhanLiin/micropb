use micropb::{MessageDecode, MessageEncode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/implicit_presence.rs"));
}

#[test]
fn implicit_presence() {
    let mut non_opt = proto::NonOptional {
        int32_num: 3,
        int64_num: 3,
        uint32_num: 3,
        uint64_num: 3,
        sint32_num: 3,
        sint64_num: 3,
        fixed32_num: 3,
        fixed64_num: 3,
        sfixed32_num: 3,
        sfixed64_num: 3,
        boolean: true,
        flt: 3.0,
        dbl: 3.0,
        enumeration: proto::Enum::Two,
        st: String::from("stuff"),
        bt: vec![0x01, 0x02],
    };
    let orig = non_opt.clone();

    // Decoding 0s shouldn't overwrite any field
    let mut decoder = PbDecoder::new(
        [
            0x08, 0x00, // field 1
            0x10, 0x00, // field 2
            0x18, 0x00, // field 3
            0x20, 0x00, // field 4
            0x28, 0x00, // field 5
            0x30, 0x00, // field 6
            0x38, 0x00, 0x00, 0x00, 0x00, // field 7
            0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 8
            0x48, 0x00, 0x00, 0x00, 0x00, // field 9
            0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 10
            0x58, 0x00, // field 11
            0x60, 0x00, 0x00, 0x00, 0x00, // field 12
            0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 13
            0x70, 0x00, // field 14
            0x7A, 0x00, // field 15
            0x82, 0x01, 0x00, // field 16
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    non_opt.decode(&mut decoder, len).unwrap();
    assert_eq!(non_opt, orig);
}

#[test]
fn encode_implicit_presence() {
    let mut non_opt = proto::NonOptional::default();
    assert_eq!(non_opt.compute_size(), 0);
    non_opt.st = String::from("");
    non_opt.bt = vec![];
    assert_eq!(non_opt.compute_size(), 0);
}

#[test]
fn explicit_presence() {
    let mut opt = proto::Optional {
        int32_num: 3,
        int64_num: 3,
        uint32_num: 3,
        uint64_num: 3,
        sint32_num: 3,
        sint64_num: 3,
        fixed32_num: 3,
        fixed64_num: 3,
        sfixed32_num: 3,
        sfixed64_num: 3,
        boolean: true,
        flt: 3.0,
        dbl: 3.0,
        enumeration: proto::Enum::Two,
        st: String::from("stuff"),
        bt: vec![0x01, 0x02],
        _has: proto::mod_Optional::_Hazzer::default(),
    };

    // All fields should be written with 0s or empty strings
    let mut decoder = PbDecoder::new(
        [
            0x08, 0x00, // field 1
            0x10, 0x00, // field 2
            0x18, 0x00, // field 3
            0x20, 0x00, // field 4
            0x28, 0x00, // field 5
            0x30, 0x00, // field 6
            0x38, 0x00, 0x00, 0x00, 0x00, // field 7
            0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 8
            0x48, 0x00, 0x00, 0x00, 0x00, // field 9
            0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 10
            0x58, 0x00, // field 11
            0x60, 0x00, 0x00, 0x00, 0x00, // field 12
            0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // field 13
            0x70, 0x00, // field 14
            0x7A, 0x00, // field 15
            0x82, 0x01, 0x00, // field 16
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    opt.decode(&mut decoder, len).unwrap();
    assert_eq!(opt.int32_num(), Some(&0));
    assert_eq!(opt.int64_num(), Some(&0));
    assert_eq!(opt.uint32_num(), Some(&0));
    assert_eq!(opt.uint64_num(), Some(&0));
    assert_eq!(opt.sint32_num(), Some(&0));
    assert_eq!(opt.sint64_num(), Some(&0));
    assert_eq!(opt.fixed32_num(), Some(&0));
    assert_eq!(opt.fixed64_num(), Some(&0));
    assert_eq!(opt.sfixed32_num(), Some(&0));
    assert_eq!(opt.sfixed64_num(), Some(&0));
    assert_eq!(opt.boolean(), Some(&false));
    assert_eq!(opt.flt(), Some(&0.0));
    assert_eq!(opt.dbl(), Some(&0.0));
    assert_eq!(opt.enumeration(), Some(&proto::Enum(0)));
    assert!(opt.st().as_ref().unwrap().is_empty());
    assert!(opt.bt().as_ref().unwrap().is_empty());
}

#[test]
fn encode_explicit_presence() {
    let mut opt = proto::Optional::default();
    opt.set_int32_num(0);
    opt.set_int64_num(0);
    opt.set_uint32_num(0);
    opt.set_uint64_num(0);
    opt.set_sint32_num(0);
    opt.set_sint64_num(0);
    opt.set_fixed32_num(0);
    opt.set_fixed64_num(0);
    opt.set_sfixed32_num(0);
    opt.set_sfixed64_num(0);
    opt.set_boolean(false);
    opt.set_flt(0.0);
    opt.set_dbl(0.0);
    opt.set_enumeration(0.into());
    opt.set_st(String::from(""));
    opt.set_bt(vec![]);
    assert_eq!(opt.compute_size(), 63);
}
