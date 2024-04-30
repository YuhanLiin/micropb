use std::{
    collections::BTreeMap,
    mem::{size_of, size_of_val},
};

use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

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
    let len = decoder.as_reader().len();
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
    let len = decoder.as_reader().len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(data.s, "Зд");
    assert_eq!(data.b, &[]);

    let mut decoder = PbDecoder::new([0x0A, 2, 0xC3, 0x28].as_slice());
    let len = decoder.as_reader().len();
    assert!(matches!(
        data.decode(&mut decoder, len),
        Err(micropb::DecodeError::Utf8(_))
    ));

    let mut decoder = PbDecoder::new([0x0A, 0, 0x12, 0].as_slice());
    let len = decoder.as_reader().len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(data.s, "");
    assert_eq!(data.b, &[]);
}

#[test]
fn encode_string_bytes() {
    let mut data = proto::Data::default();
    assert_eq!(data.compute_size(), 0);
    data.set_s(String::from("abcdefg"));
    assert_eq!(data.compute_size(), 9);
    let mut encoder = PbEncoder::new(vec![]);
    data.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[0x0A, 7, b'a', b'b', b'c', b'd', b'e', b'f', b'g',]
    );

    data.set_s(String::from(""));
    assert_eq!(data.compute_size(), 2);
    let mut encoder = PbEncoder::new(vec![]);
    data.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x0A, 0]);

    data.clear_s();

    data.set_b(vec![0x0A, 0x0B]);
    assert_eq!(data.compute_size(), 4);
    data.set_b(vec![0x01; 150]);
    // 2 bytes for length of bytes instead of 1
    assert_eq!(data.compute_size(), 153);
    let mut encoder = PbEncoder::new(vec![]);
    data.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        [[0x12, 0x96, 0x01].as_slice(), [0x01; 150].as_slice()].concat()
    );
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
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 2);
    assert_eq!(list.list[0].s, "a");
    assert_eq!(list.list[1].s, "b");

    let mut decoder = PbDecoder::new([0x0A, 3, 0x0A, 1, b'x'].as_slice()); // field 1
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 3);
    assert_eq!(list.list[2].s, "x");

    list.list.clear();
    // Decode empty struct
    let mut decoder = PbDecoder::new([0x0A, 0].as_slice());
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 1);
    assert_eq!(list.list[0], Default::default());
}

#[test]
fn encode_repeated() {
    let mut list = proto::List::default();
    assert_eq!(list.compute_size(), 0);
    list.list.push(proto::Data::default());
    list.list.push(proto::Data::default());
    assert_eq!(list.compute_size(), 4);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[0x0A, 0, 0x0A, 0]); // field 1 twice

    list.list[0].set_s(String::from("xyz"));
    assert_eq!(list.compute_size(), 9);
    list.list[1].set_s(String::from("u"));
    list.list[1].set_b(vec![b'x']);
    assert_eq!(list.compute_size(), 15);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[
            0x0A, 5, 0x0A, 3, b'x', b'y', b'z', // field 1
            0x0A, 6, 0x0A, 1, b'u', 0x12, 1, b'x', // field 1
        ]
    );
}

#[test]
fn decode_packed() {
    let mut numlist = proto::NumList::default();
    // non-packed decoding
    let mut decoder = PbDecoder::new(
        [
            0x08, 0x12, // field 1
            0x08, 0x01, // field 1 again
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list.len(), 2);
    assert_eq!(numlist.list, &[0x12, 0x01]);

    // packed decoding
    let mut decoder = PbDecoder::new([0x0A, 4, 0x01, 0x96, 0x01, 0x03].as_slice());
    let len = decoder.as_reader().len();
    numlist.decode(&mut decoder, len).unwrap();
    assert_eq!(numlist.list.len(), 5);
    assert_eq!(&numlist.list[2..], &[1, 150, 3]);
}

#[test]
fn decode_packed_fixed() {
    let mut list = proto::FixedList::default();
    // non-packed decoding
    let mut decoder = PbDecoder::new([0x08, 0x12, 0x11, 0x00, 0x00].as_slice());
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 1);
    assert_eq!(list.list, &[0x1112]);

    // packed decoding
    let mut decoder =
        PbDecoder::new([0x0A, 8, 0x01, 0x96, 0x01, 0x03, 0x22, 0x34, 0xFF, 0xFF].as_slice());
    let len = decoder.as_reader().len();
    list.decode(&mut decoder, len).unwrap();
    assert_eq!(list.list.len(), 3);
    assert_eq!(&list.list[1..], &[0x03019601, 0xFFFF3422]);
}

#[test]
fn encode_non_packed() {
    let mut list = proto::NumList::default();
    assert_eq!(list.compute_size(), 0);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[]);

    list.list.push(12);
    list.list.push(150);
    list.list.push(0);
    assert_eq!(list.compute_size(), 7);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[0x08, 0x0C, 0x08, 0x96, 0x01, 0x08, 0x00]
    );
}

#[test]
fn encode_packed() {
    let mut list = proto::FixedList::default();
    assert_eq!(list.compute_size(), 0);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[]);

    list.list.push(12);
    list.list.push(150);
    list.list.push(0xFFFFFFFF);
    assert_eq!(list.compute_size(), 14);
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.into_writer(),
        &[0x0A, 12, 0x0C, 0x00, 0x00, 0x00, 0x96, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]
    );
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
    let len = decoder.as_reader().len();
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
    let len = decoder.as_reader().len();
    map.decode(&mut decoder, len).unwrap();
    assert_eq!(map.mapping.len(), 2);
    assert_eq!(map.mapping["ac"], &[0x02, 0x01, 0x02]);
}

#[test]
fn encode_map() {
    let mut map = proto::Map::default();
    assert_eq!(map.compute_size(), 0);
    let mut encoder = PbEncoder::new(vec![]);
    map.encode(&mut encoder).unwrap();
    assert_eq!(encoder.into_writer(), &[]);

    map.mapping.insert(String::from("ab"), vec![0x01]);
    assert_eq!(map.compute_size(), 9);
    map.mapping.insert(String::from("a"), vec![0x01, 0x02]);
    assert_eq!(map.compute_size(), 18);
    let mut encoder = PbEncoder::new(vec![]);
    map.encode(&mut encoder).unwrap();
    // Order of expected output relies on order of elements in the map, which isn't guaranteed
    assert_eq!(
        encoder.into_writer(),
        &[
            0xA, 7, 0xA, 1, b'a', 0x12, 2, 0x01, 0x02, // field 1
            0xA, 7, 0xA, 2, b'a', b'b', 0x12, 1, 0x01, // field 1 again
        ]
    );
}
