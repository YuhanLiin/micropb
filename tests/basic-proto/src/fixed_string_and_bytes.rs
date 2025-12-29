use micropb::{FixedLenString, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/fixed_string_and_bytes.rs"));
}

mod proto_cached {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(
        env!("OUT_DIR"),
        "/fixed_string_and_bytes.cached.rs"
    ));
}

#[test]
fn string_bytes() {
    let data = proto::Data::default();
    assert!(data.s().is_none());
    assert!(data.b().is_none());
    assert_eq!(&*data.s, "a\n\0");
    assert_eq!(&data.b, &[0x0, 0xFF]);
    let _: FixedLenString<3> = data.s;
    let _: [u8; 2] = data.b;
}

#[test]
fn decode_string_bytes() {
    let mut data = proto::Data::default();
    let mut decoder = PbDecoder::new(
        [
            0x0A, 3, b'a', b'b', b'c', // field 1
            0x12, 2, 0x01, 0x02, // field 2
        ]
        .as_slice(),
    );
    let len = decoder.as_reader().len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(&*data.s, "abc");
    assert_eq!(&data.b, &[1, 2]);

    let mut decoder = PbDecoder::new([0x0A, 0, 0x12, 0].as_slice());
    let len = decoder.as_reader().len();
    data.decode(&mut decoder, len).unwrap();
    assert_eq!(&*data.s, "\0\0\0");
    assert_eq!(&data.b, &[1, 2]);
}

macro_rules! encode_tests {
    ($mod:ident) => {
        #[test]
        fn encode_string_bytes() {
            let mut data = $mod::Data::default();
            data.set_s(FixedLenString::try_from("abc").unwrap());
            data.set_b([1, 2]);

            let mut encoder = PbEncoder::new(vec![]);
            data.encode(&mut encoder).unwrap();
            assert_eq!(
                encoder.into_writer(),
                &[
                    0x0A, 3, b'a', b'b', b'c', // field 1
                    0x12, 2, 0x01, 0x02, // field 2
                ]
            );
        }
    };
}

encode_tests!(proto);

mod cached {
    use super::*;
    encode_tests!(proto_cached);
}
