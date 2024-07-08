mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/keyword_fields.rs"));
}

#[test]
fn renamed_fields() {
    // Only the `super` field was renamed, every other field are raw or sanitized identifiers
    let msg = proto::crate_::self_::r#async::Msg::default();
    assert!(!msg.super_);
    assert!(!msg._has.super_());
    assert!(!msg.r#i32);
    assert!(!msg._has.r#i32());
    assert!(!msg.r#type);
    assert!(!msg._has.r#type());
    assert!(msg.r#try.is_none());
}

#[test]
fn sanitized_enum() {
    let e = proto::crate_::self_::r#async::Enum::_1;
    assert!(e != proto::crate_::self_::r#async::Enum::Self_);
}
