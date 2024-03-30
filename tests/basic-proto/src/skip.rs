use std::mem::size_of;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/skip.rs"));
}

#[test]
fn empty_msg() {
    let mut nested = proto::nested::Nested::default();
    nested.inner = Some(proto::nested::mod_Nested::Inner::Scalar(true));
    assert_eq!(size_of::<proto::nested::Nested>(), size_of::<bool>());
}
