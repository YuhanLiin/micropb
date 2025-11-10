use micropb::MessageEncode;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/minimal_accessors.rs"));
}

#[test]
fn sanity_check() {
    let mut basic = proto::basic_::BasicTypes::default();
    assert_eq!(proto::basic_::BasicTypes::MAX_SIZE, None);
    assert!(!basic._has.dbl());
    assert_eq!(basic.dbl, 0.0);
    assert_eq!(basic.dbl(), None);
    basic._has.set_dbl();
    assert_eq!(basic.dbl(), Some(&0.0));

    assert!(!basic._has.flt());
    assert_eq!(basic.flt, 1.0); // custom default
    assert_eq!(basic.flt(), None);
    basic._has.set_flt();
    assert_eq!(basic.flt(), Some(&1.0));

    let mut nested = proto::nested_::Nested::default();
    assert_eq!(proto::nested_::Nested::MAX_SIZE, None);
    assert_eq!(nested.basic(), None);
    nested.basic = Some(basic);
    assert!(nested.basic().is_some());
}
