mod proto {
    include!(concat!(env!("OUT_DIR"), "/boxed_and_option.rs"));
}

#[test]
fn boxed_and_option() {
    let mut basic = proto::basic::BasicTypes::default();

    assert_eq!(basic.int32_num, None);
    assert_eq!(basic.int32_num(), None);
}
