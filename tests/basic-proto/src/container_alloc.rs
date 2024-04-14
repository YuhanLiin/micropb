use std::{
    collections::BTreeMap,
    mem::{size_of, size_of_val},
};

use micropb::{MessageDecode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
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

#[test]
fn decode_string_bytes() {
    let mut data = proto::Data::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 4, b'a', b'b', b'c', b'd', // field 1
            0x12, 7, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(data.s, "abcd");
    assert_eq!(data.b, &[1, 2, 3, 4, 5, 6, 7]);

    let mut decoder = PbDecoder::new(
        [
            0x0A, 4, 208, 151, 208, 180, // field 1
            0x12, 0, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(data.s, "Зд");
    assert_eq!(data.b, &[]);
}

#[test]
fn decode_repeated() {
    let mut list = proto::List::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 3, 0x0A, 1, b'a', // field 1
            0x0A, 3, 0x0A, 1, b'b', // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 2);
    assert_eq!(list.list[0].s, "a");
    assert_eq!(list.list[1].s, "b");

    let mut decoder = PbDecoder::new([0x0A, 3, 0x0A, 1, b'x'].as_slice()); // field 1
    let len = decoder.reader.len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 3);
    assert_eq!(list.list[2].s, "x");
}

#[test]
fn decode_packed() {
    let mut numlist = proto::NumList::default();
    // non-packed encoding
    let mut decoder = PbDecoder::new(
        [
            0x08, 0x12, // field 1
            0x08, 0x01, // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list.len(), 2);
    assert_eq!(numlist.list, &[0x12, 0x01]);

    // packed encoding
    let mut decoder = PbDecoder::new([0x0A, 4, 0x01, 0x96, 0x01, 0x03].as_slice());
    let len = decoder.reader.len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list.len(), 5);
    assert_eq!(&numlist.list[2..], &[1, 150, 3]);
}

#[test]
fn decode_packed_fixed() {
    let mut list = proto::FixedList::default();
    // non-packed encoding
    let mut decoder = PbDecoder::new([0x08, 0x12, 0x11, 0x00, 0x00].as_slice());
    let len = decoder.reader.len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 1);
    assert_eq!(list.list, &[0x1112]);

    // packed encoding
    let mut decoder =
        PbDecoder::new([0x0A, 8, 0x01, 0x96, 0x01, 0x03, 0x22, 0x34, 0xFF, 0xFF].as_slice());
    let len = decoder.reader.len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 3);
    assert_eq!(&list.list[1..], &[0x03019601, 0xFFFF3422]);
}

#[test]
fn decode_map() {
    let mut map = proto::Map::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 7, 0x0A, 2, b'a', b'c', 0x12, 1, 0x02, // field 1
            0x0A, 7, 0x0A, 1, b'a', 0x12, 2, 0x02, 0x12, // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    map.decode(&mut decoder, len).unwrap();
    assert_eq!(map.mapping.len(), 2);
    assert_eq!(map.mapping["ac"], &[0x02]);
    assert_eq!(map.mapping["a"], &[0x02, 0x12]);

    // replace one of the existing keys instead of adding a new key
    let mut decoder = PbDecoder::new(
        [
            0x0A, 9, 0x0A, 2, b'a', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1
        ]
        .as_slice(),
    );
    let len = decoder.reader.len();
    map.decode(&mut decoder, len).unwrap();
    assert_eq!(map.mapping.len(), 2);
    assert_eq!(map.mapping["ac"], &[0x02, 0x01, 0x02]);
}
