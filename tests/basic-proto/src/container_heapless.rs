use std::mem::{size_of, size_of_val};

use micropb::{DecodeError, MessageDecode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
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

#[test]
fn decode_string_bytes_cap() {
    let mut data = proto::Data::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 3, b'a', b'b', b'c', // field 1
            0x12, 5, 0x01, 0x02, 0x03, 0x04, 0x05, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(data.s, "abc");
    assert_eq!(data.b, &[1, 2, 3, 4, 5]);

    let mut decoder = PbDecoder::new([0x0A, 4, b'a', b'b', b'c', b'd'].as_slice()); // field 1
    let len = decoder.as_reader().len();
    assert_eq!(data.decode(&mut decoder, len), Err(DecodeError::Capacity));

    let mut decoder = PbDecoder::new([0x12, 6, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06].as_slice()); // field 2
    let len = decoder.as_reader().len();
    assert_eq!(data.decode(&mut decoder, len), Err(DecodeError::Capacity));
}

#[test]
fn decode_repeated_cap() {
    let mut list = proto::List::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 3, 0x0A, 1, b'a', // field 1
            0x0A, 3, 0x0A, 1, b'b', // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 2);
    assert_eq!(list.list[0].s, "a");
    assert_eq!(list.list[1].s, "b");

    let mut decoder = PbDecoder::new([0x0A, 3, 0x0A, 1, b'x'].as_slice()); // field 1
    let len = decoder.as_reader().len();
    assert_eq!(list.decode(&mut decoder, len), Err(DecodeError::Capacity));

    let mut decoder = PbDecoder::new([0x0A, 3, 0x0A, 1, b'x'].as_slice()); // field 1
    decoder.ignore_repeated_cap_err = true;
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 2);
    assert_eq!(list.list[0].s, "a");
    assert_eq!(list.list[1].s, "b");
}

#[test]
fn decode_repeated_cap_inner() {
    let mut list = proto::StrList::default();
    let mut decoder = PbDecoder::new([0x0A, 3, b'a', b'b', b'c'].as_slice());
    let len = decoder.as_reader().len();
    assert_eq!(list.decode(&mut decoder, len), Err(DecodeError::Capacity));
}

#[test]
fn decode_packed_cap() {
    let mut numlist = proto::NumList::default();
    // packed encoding
    let mut decoder = PbDecoder::new([0x0A, 3, 0x01, 0x96, 0x01].as_slice());
    let len = decoder.as_reader().len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list, &[1, 150]);

    let mut decoder = PbDecoder::new([0x0A, 1, 0x01].as_slice());
    let len = decoder.as_reader().len();
    assert_eq!(
        numlist.decode(&mut decoder, len),
        Err(DecodeError::Capacity)
    );

    let mut decoder = PbDecoder::new([0x0A, 1, 0x01].as_slice());
    decoder.ignore_repeated_cap_err = true;
    let len = decoder.as_reader().len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list, &[1, 150]);
}

#[test]
fn decode_packed_cap_oneshot() {
    let mut numlist = proto::NumList::default();
    let mut decoder = PbDecoder::new([0x0A, 4, 0x01, 0x96, 0x01, 0x05].as_slice());
    let len = decoder.as_reader().len();
    assert_eq!(
        numlist.decode(&mut decoder, len),
        Err(DecodeError::Capacity)
    );

    numlist.list.clear();
    let mut decoder = PbDecoder::new([0x0A, 3, 0x08, 0x01, 0x05].as_slice());
    decoder.ignore_repeated_cap_err = true;
    let len = decoder.as_reader().len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list, &[8, 1]);
}

#[test]
fn decode_packed_fixed_cap() {
    let mut list = proto::FixedList::default();
    // packed encoding
    let mut decoder = PbDecoder::new(
        [
            0x0A, 12, 0x01, 0x96, 0x01, 0x03, 0x22, 0x34, 0xFF, 0xFF, 0x00, 0x01, 0x00, 0x01,
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    assert_eq!(list.decode(&mut decoder, len), Err(DecodeError::Capacity));
}

#[test]
fn decode_map_cap() {
    let mut map = proto::Map::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 7, 0x0A, 2, b'a', b'c', 0x12, 1, 0x02, // field 1
            0x0A, 7, 0x0A, 1, b'a', 0x12, 2, 0x02, 0x12, // field 1 again
            0x0A, 7, 0x0A, 1, b'b', 0x12, 2, 0x02, 0x12, // field 1 again
            0x0A, 7, 0x0A, 1, b'c', 0x12, 2, 0x02, 0x12, // field 1 again
            0x0A, 9, 0x0A, 2, b'd', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1 again
            0x0A, 9, 0x0A, 2, b'e', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1 again
            0x0A, 9, 0x0A, 2, b'f', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1 again
            0x0A, 9, 0x0A, 2, b'g', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1 again
            0x0A, 9, 0x0A, 2, b'h', b'c', 0x12, 3, 0x02, 0x01, 0x02, // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    assert_eq!(map.decode(&mut decoder, len), Err(DecodeError::Capacity));
    assert_eq!(map.mapping.len(), map.mapping.capacity());

    let mut decoder = PbDecoder::new(
        [
            0x0A, 7, 0x0A, 2, b'x', b'y', 0x12, 1, 0x02, // field 1
            0x0A, 7, 0x0A, 1, b'x', 0x12, 2, 0x02, 0x12, // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    decoder.ignore_repeated_cap_err = true;
    map.decode(&mut decoder, len).unwrap();
    assert_eq!(map.mapping.len(), map.mapping.capacity());
}

#[test]
fn decode_map_cap_inner() {
    let mut map = proto::Map::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 10, 0x0A, 5, b'a', b'c', b'd', b'k', b'o', 0x12, 1, 0x02,
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    assert_eq!(map.decode(&mut decoder, len), Err(DecodeError::Capacity));

    let mut decoder = PbDecoder::new(
        [
            0x0A, 12, 0x0A, 4, b'a', b'c', b'd', b'k', 0x12, 4, 0x02, 0x03, 0x04, 0x05,
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    assert_eq!(map.decode(&mut decoder, len), Err(DecodeError::Capacity));
}
