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
        assert_eq!(msg.compute_size(), encoder.as_writer().len());

        let mut decoder = PbDecoder::new(encoder.as_writer().as_slice());
        let mut output = proto::TestOneOf::default();
        output.decode(&mut decoder, encoder.as_writer().len()).unwrap();
        assert_eq!(msg, output);
    }

    #[test]
    fn max_size(sin1: proto::TestTypesSingular1, opt1: proto::TestTypesOptional1) {
        const SIN1_MAX: usize = proto::TestTypesSingular1::MAX_SIZE.unwrap();
        assert!(sin1.compute_size() <= SIN1_MAX);
        const OPT1_MAX: usize = proto::TestTypesOptional1::MAX_SIZE.unwrap();
        assert!(opt1.compute_size() <= OPT1_MAX);
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
