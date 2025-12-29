use std::borrow::Cow;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/container_cow.rs"));
}

// Include this module just to see that the generated code compiles
#[allow(dead_code)]
mod proto_cached {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/container_cow.cached.rs"));
}

#[test]
fn string_bytes() {
    let data = proto::Data::default();
    assert!(data.s().is_none());
    assert!(data.b().is_none());
    assert_eq!(data.s, "a\n\0");
    assert_eq!(data.b.as_ref(), &[0x0, 0xFF]);
    let _: Cow<str> = data.s;
    let _: Cow<[u8]> = data.b;
}

#[test]
fn repeated() {
    let list = proto::List::default();
    assert!(list.list.is_empty());
    assert_eq!(size_of_val(&list), size_of::<Vec<proto::Data>>());
    let _: Cow<[proto::Data<'_>]> = list.list;

    let numlist = proto::NumList::default();
    assert!(numlist.list.is_empty());
    assert_eq!(size_of_val(&numlist), size_of::<Vec<u8>>());
    let _: Cow<[u32]> = numlist.list;
}
