mod proto {
    #![deny(missing_docs)] // Ensure that all docs are generated properly
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!("../test.proto.rs");
}

#[cfg(test)]
#[test]
fn sanity_check() {
    // Just make sure the types exist
    let _: [proto::Msg; 0] = [];
    let _: [proto::Count; 0] = [];
    let _: [proto::Msg_::Variant; 0] = [];
    let _: [proto::Msg_::Inner; 0] = [];
    let _: [proto::Msg_::Count; 0] = [];
}
