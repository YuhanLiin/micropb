mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/files_with_same_package.rs"));
}

#[test]
fn same_package() {
    let basic = proto::basic_::BasicTypes::default();
    assert_eq!(basic.boolean(), None);

    let basic_dup = proto::basic_::BasicDup::default();
    assert_eq!(basic_dup.x, 0);
}
