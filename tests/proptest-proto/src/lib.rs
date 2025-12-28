extern crate alloc;

#[cfg(test)]
mod tests {
    use micropb::{size::max_encoded_size, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

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

    mod micropb_types_cached {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/micropb_all_types.cached.rs"));
    }

    mod micropb_oneof_enum_cached {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/micropb_oneof_enum.cached.rs"));
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
        // 1. micropb input -> 2. micropb encode -> 3. micropb decode
        // Assert 1 and 3 are equal

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
        // 1. micropb input -> 2. micropb encode -> 3. prost decode -> 4. prost encode
        // -> 5. micropb decode -> 6. micropb encode -> 7. prost decode
        //
        // Assert that 1 and 5 are equal (micropb), and that 3 and 7 are equal (prost). These two
        // roundtrip tests will pick up issues in micropb even if the decoding and encoding are
        // internally consistent with one another.

        use prost::Message;

        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();

        let prost_msg = prost_types::TestOneOf::decode(encoder.as_writer().as_slice()).unwrap();
        let buf = prost_msg.encode_to_vec();

        let mut decoder = PbDecoder::new(buf.as_slice());
        decoder.ignore_wrong_len = true;
        let mut output = M::default();
        output.decode(&mut decoder, buf.len()).unwrap();
        assert_eq!(msg, output);

        let mut encoder = PbEncoder::new(vec![]);
        output.encode(&mut encoder).unwrap();
        let prost_output = prost_types::TestOneOf::decode(encoder.as_writer().as_slice()).unwrap();
        assert_eq!(prost_msg, prost_output);
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(2000))]

        // Roundtrip random message structures
        #[test]
        fn roundtrip(msg: micropb_types::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn map_roundtrip(msg: micropb_types::TestMaps) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip(msg: micropb_types::TestOneOf) {
            test_proto_roundtrip(msg);
        }

        // Roundtrip oneof enum messages
        #[test]
        fn roundtrip_oneof_enum(msg: micropb_oneof_enum::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn map_roundtrip_oneof_enum(msg: micropb_oneof_enum::TestMaps) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip_oneof_enum(msg: micropb_oneof_enum::TestOneOf) {
            test_proto_roundtrip(msg);
        }

        // Roundtrip messages with cached encoding
        #[test]
        fn roundtrip_cached(msg: micropb_types_cached::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn map_roundtrip_cached(msg: micropb_types_cached::TestMaps) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip_cached(msg: micropb_types_cached::TestOneOf) {
            test_proto_roundtrip(msg);
        }

        // Roundtrip oneof enum messages with cached encoding
        #[test]
        fn roundtrip_oneof_enum_cached(msg: micropb_oneof_enum_cached::TestOneOf) {
            test_roundtrip(msg);
        }

        #[test]
        fn map_roundtrip_oneof_enum_cached(msg: micropb_oneof_enum_cached::TestMaps) {
            test_roundtrip(msg);
        }

        #[test]
        fn proto_roundtrip_oneof_enum_cached(msg: micropb_oneof_enum_cached::TestOneOf) {
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
            const SIN1_MAX: usize = max_encoded_size::<micropb_types::TestTypesSingular1>();
            assert!(sin1.compute_size() <= SIN1_MAX);
            const OPT1_MAX: usize = max_encoded_size::<micropb_types::TestTypesOptional1>();
            assert!(opt1.compute_size() <= OPT1_MAX);
        }
    }
}
