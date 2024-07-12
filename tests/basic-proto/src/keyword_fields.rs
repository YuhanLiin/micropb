mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/keyword_fields.rs"));
}

#[test]
fn keyword_fields() {
    // Every field is raw or sanitized identifier
    let mut msg = proto::crate_::self_::async_::Msg::default();
    assert!(!msg._super);
    assert!(!msg._has._super());
    msg.set_super(true);
    assert!(!msg.r#i32);
    assert!(!msg._has.r#i32());
    assert!(!msg.r#type);
    assert!(!msg._has.r#type());

    let _: proto::crate_::self_::async_::_Self = msg.self_msg;
    let _: proto::crate_::self_::async_::Self_::_Self = msg.self_enum;

    assert!(msg.r#try.is_none());
    assert!(msg._self.is_none());
    // Ensure that the enum type and variant names are generated properly
    let _ = proto::crate_::self_::async_::Msg_::Try::As;
    let _ = proto::crate_::self_::async_::Msg_::_Self::Crate;
    let _ = proto::crate_::self_::async_::Self_::Crate::_Self;
}

#[test]
fn sanitized_enum() {
    let e = proto::crate_::self_::async_::Enum::_1;
    assert!(e != proto::crate_::self_::async_::Enum::_Self);
    assert!(e != proto::crate_::self_::async_::Enum::Error);
}
