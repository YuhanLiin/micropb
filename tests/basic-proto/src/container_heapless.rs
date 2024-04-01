use std::mem::{size_of, size_of_val};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/container_heapless.rs"));
}

#[test]
fn string_bytes() {
    let data = proto::Data::default();
    assert!(data.s().is_none());
    assert!(data.b().is_none());
    assert_eq!(data.s, "a\n\0");
    assert_eq!(data.b, &[0x0, 0xFF]);
    assert_eq!(data.s.capacity(), 3);
    assert_eq!(data.b.capacity(), 5);
    let _: micropb::heapless::String<3> = data.s;
    let _: micropb::heapless::Vec<u8, 5> = data.b;
}

#[test]
fn repeated() {
    let list = proto::List::default();
    assert!(list.list.is_empty());
    assert_eq!(
        size_of_val(&list),
        size_of::<micropb::heapless::Vec<proto::Data, 2>>()
    );
    assert_eq!(list.list.capacity(), 2);
    let _: micropb::heapless::Vec<proto::Data, 2> = list.list;

    let numlist = proto::NumList::default();
    assert!(numlist.list.is_empty());
    assert_eq!(numlist.list.capacity(), 2);
    assert_eq!(
        size_of_val(&numlist),
        size_of::<micropb::heapless::Vec<u8, 2>>()
    );
    let _: micropb::heapless::Vec<u8, 2> = numlist.list;
}

#[test]
fn map() {
    let map = proto::Map::default();
    assert!(map.mapping.is_empty());
    assert_eq!(map.mapping.capacity(), 8);
    assert_eq!(
        size_of_val(&map),
        size_of::<
            micropb::heapless::FnvIndexMap<
                micropb::heapless::String<4>,
                micropb::heapless::Vec<u8, 3>,
                8,
            >,
        >()
    );
    let _: micropb::heapless::FnvIndexMap<
        micropb::heapless::String<4>,
        micropb::heapless::Vec<u8, 3>,
        8,
    > = map.mapping;
}
