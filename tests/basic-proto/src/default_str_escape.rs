mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/default_str_escape.rs"));
}

#[test]
fn default_str_escape() {
    let p = proto::Person::default();
    assert_eq!(p.name, "[\"unknown\"]");
}
