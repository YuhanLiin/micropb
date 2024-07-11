use std::mem::size_of;

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/skip.rs"));
}

#[test]
fn empty_msg() {
    let mut nested = proto::nested_::Nested::default();
    nested.inner = Some(proto::nested_::Nested_::Inner::Scalar(true));
    assert_eq!(size_of::<proto::nested_::Nested>(), size_of::<bool>());
}
