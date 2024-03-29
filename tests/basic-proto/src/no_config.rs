mod proto {
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
