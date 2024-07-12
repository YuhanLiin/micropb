mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/conflicting_names.rs"));
}

// Just make sure this module compiles
#[test]
fn dummy() {}
