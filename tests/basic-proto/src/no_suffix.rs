mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/no_suffix.rs"));
}

#[test]
fn paths() {
    let _ = proto::basic::Enum(0);
    let _ = proto::basic::BasicTypes::default();
    let _ = proto::nested::Nested::default();
    let _ = Option::<proto::nested::Nested_::Inner>::None;
}
