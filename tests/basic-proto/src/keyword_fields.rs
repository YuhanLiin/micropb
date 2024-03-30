use std::mem::size_of;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/keyword_fields.rs"));
}

#[test]
fn renamed_fields() {
    let msg = proto::Msg::default();
    assert!(!msg.super_);
    assert!(!msg._has.super_());
    assert!(!msg.i32_);
    assert!(!msg._has.i32_());
    assert!(!msg.typ);
    assert!(!msg._has.typ());
}
