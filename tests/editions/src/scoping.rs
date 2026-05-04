use micropb::{MessageEncode, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/scoping.rs"));
}

#[test]
fn check_types() {
    let msg = proto::Example::default();
    let _: bool = msg.valid;
    let _: Option<i32> = msg.user_id;
    let inner = proto::Example_::Inner::default();
    let _: Option<_> = inner.status;
}

#[test]
fn packed() {
    let mut packed = proto::Packed::default();
    packed.list.push(true);
    packed.list.push(false);

    let mut encoder = PbEncoder::new(vec![]);
    packed.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x0A, 2, 0x01, 0x00])
}

#[test]
fn expanded() {
    let mut expanded = proto::Expanded::default();
    expanded.list.push(true);
    expanded.list.push(false);

    let mut encoder = PbEncoder::new(vec![]);
    expanded.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x08, 0x01, 0x08, 0x00]);
}
