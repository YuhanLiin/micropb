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
