mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/conflicting_names.rs"));
}

// Just make sure this module compiles
#[test]
fn dummy() {}
