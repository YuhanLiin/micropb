use std::mem::size_of;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/int_type.rs"));
}

#[test]
fn enum_int_type() {
    let enumeration = proto::basic::Enum::One;
    let _: i8 = enumeration.0;
    assert_eq!(size_of::<proto::basic::Enum>(), 1);
}

#[test]
fn field_int_type() {
    let basic = proto::basic::BasicTypes::default();
    let _: i16 = basic.int32_num;
    let _: Option<&i16> = basic.int32_num();
    assert_eq!(basic.int32_num, -5i16);
}
