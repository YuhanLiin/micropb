use std::mem::size_of;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/skip.rs"));
}

#[test]
fn empty_msg() {
    assert_eq!(size_of::<proto::nested_::Nested>(), size_of::<bool>());
}
