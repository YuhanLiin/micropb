mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/conflicting_names.rs"));
}

// Just make sure this module compiles
#[test]
fn dummy() {}
