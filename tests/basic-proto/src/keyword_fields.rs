mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/keyword_fields.rs"));
}

#[test]
fn renamed_fields() {
    // Only the `super` field was renamed, every other field are raw identifiers
    let msg = proto::_crate::_self::r#async::Msg::default();
    assert!(!msg.super_);
    assert!(!msg._has.super_());
    assert!(!msg.r#i32);
    assert!(!msg._has.r#i32());
    assert!(!msg.r#type);
    assert!(!msg._has.r#type());
    assert!(msg.r#try.is_none());
}
