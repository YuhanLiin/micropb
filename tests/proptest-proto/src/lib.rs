extern crate alloc;

#[cfg(test)]
mod tests {
    use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

    mod micropb_types {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/micropb_all_types.rs"));
    }

    mod micropb_oneof_enum {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/micropb_oneof_enum.rs"));
    }

    mod prost_types {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/_.rs"));
    }

    fn bytes() -> impl proptest::strategy::Strategy<Value = Vec<u8>> {
        proptest::collection::vec(
            proptest::num::u8::ANY,
            proptest::collection::size_range(0..2000),
        )
    }

    fn test_roundtrip<M>(msg: M)
    where
        M: MessageEncode + MessageDecode + Default + PartialEq + std::fmt::Debug,
    {
        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();
        assert_eq!(msg.compute_size(), encoder.as_writer().len());

        let mut decoder = PbDecoder::new(encoder.as_writer().as_slice());
        let mut output = M::default();
        output
            .decode(&mut decoder, encoder.as_writer().len())
            .unwrap();
        assert_eq!(msg, output);
    }

    fn test_proto_roundtrip<M>(msg: M)
    where
        M: MessageEncode + MessageDecode + Default + PartialEq + std::fmt::Debug,
    {
        use prost::Message;

        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();

        let proto_msg = prost_types::TestOneOf::decode(encoder.as_writer().as_slice()).unwrap();
        let buf = proto_msg.encode_to_vec();

        let mut decoder = PbDecoder::new(buf.as_slice());
        decoder.ignore_wrong_len = true;
        let mut output = M::default();
        output.decode(&mut decoder, buf.len()).unwrap();
        assert_eq!(msg, output);
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(2000))]

        // Roundtrip random message structures
        #[test]
        fn roundtrip(msg: micropb_types::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip(msg: micropb_types::TestOneOf) {
            test_proto_roundtrip(msg);
        }

        #[test]
        fn roundtrip_oneof_enum(msg: micropb_oneof_enum::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip_oneof_enum(msg: micropb_oneof_enum::TestOneOf) {
            test_proto_roundtrip(msg);
        }

        // Decode random data to ensure it doesn't crash
        #[test]
        fn decode_random(data in bytes()) {
            let mut decoder = PbDecoder::new(data.as_slice());
            let mut msg = micropb_types::TestOneOf::default();
            if msg.decode(&mut decoder, data.len()).is_ok() {
                let mut encoder = PbEncoder::new(vec![]);
                let _ = msg.encode(&mut encoder);
            }
        }

        #[test]
        fn max_size(sin1: micropb_types::TestTypesSingular1, opt1: micropb_types::TestTypesOptional1) {
            const SIN1_MAX: usize = micropb_types::TestTypesSingular1::MAX_SIZE.unwrap();
            assert!(sin1.compute_size() <= SIN1_MAX);
            const OPT1_MAX: usize = micropb_types::TestTypesOptional1::MAX_SIZE.unwrap();
            assert!(opt1.compute_size() <= OPT1_MAX);
        }
    }
}
