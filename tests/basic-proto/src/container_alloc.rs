use std::{
    collections::BTreeMap,
    mem::{size_of, size_of_val},
};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/container_alloc.rs"));
}

#[test]
fn string_bytes() {
    let data = proto::Data::default();
    assert!(data.s().is_none());
    assert!(data.b().is_none());
    assert_eq!(data.s, "a\n\0");
    assert_eq!(data.b, &[0x0, 0xFF]);
    let _: String = data.s;
    let _: Vec<u8> = data.b;
}

#[test]
fn repeated() {
    let list = proto::List::default();
    assert!(list.list.is_empty());
    assert_eq!(size_of_val(&list), size_of::<Vec<proto::Data>>());
    let _: Vec<proto::Data> = list.list;

    let numlist = proto::NumList::default();
    assert!(numlist.list.is_empty());
    assert_eq!(size_of_val(&numlist), size_of::<Vec<u8>>());
    let _: Vec<u8> = numlist.list;
}

#[test]
fn map() {
    let map = proto::Map::default();
    assert!(map.mapping.is_empty());
    assert_eq!(size_of_val(&map), size_of::<BTreeMap<String, Vec<u8>>>());
    let _: BTreeMap<String, Vec<u8>> = map.mapping;
}
