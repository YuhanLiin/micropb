#[cfg(test)]
use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

extern crate alloc;

#[cfg(test)]
mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/all_types.rs"));
}

#[cfg(test)]
fn bytes() -> impl proptest::strategy::Strategy<Value = Vec<u8>> {
    proptest::collection::vec(
        proptest::num::u8::ANY,
        proptest::collection::size_range(0..2000),
    )
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(2000))]

    // Roundtrip random message structures
    #[test]
    fn roundtrip(msg: proto::TestOneOf) {
        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();

        let mut decoder = PbDecoder::new(encoder.as_writer().as_slice());
        let mut output = proto::TestOneOf::default();
        output.decode(&mut decoder, encoder.as_writer().len()).unwrap();
        assert_eq!(msg, output);
    }

    // Decode random data to ensure it doesn't crash
    #[test]
    fn decode_random(data in bytes()) {
        let mut decoder = PbDecoder::new(data.as_slice());
        let mut msg = proto::TestOneOf::default();
        if msg.decode(&mut decoder, data.len()).is_ok() {
            let mut encoder = PbEncoder::new(vec![]);
            let _ = msg.encode(&mut encoder);
        }
    }
}
