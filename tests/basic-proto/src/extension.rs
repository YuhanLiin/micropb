mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/extension.rs"));
}

// Extensions aren't actually supported, so just make sure things compile
#[test]
fn dummy() {
    let ext = proto::ext_::Extendee::default();
    assert!(!ext._has.dbl());
    let _: f64 = ext.dbl;
}
