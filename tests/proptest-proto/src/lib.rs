extern crate alloc;

#[cfg(test)]
mod tests {
    use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

    mod micropb_types {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/all_types.rs"));
    }

    mod proto_types {
        #![allow(clippy::all)]
        #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
        include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
    }

    fn bytes() -> impl proptest::strategy::Strategy<Value = Vec<u8>> {
        proptest::collection::vec(
            proptest::num::u8::ANY,
            proptest::collection::size_range(0..2000),
        )
    }

    fn test_roundtrip(msg: micropb_types::TestOneOf) {
        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();
        assert_eq!(msg.compute_size(), encoder.as_writer().len());

        let mut decoder = PbDecoder::new(encoder.as_writer().as_slice());
        let mut output = micropb_types::TestOneOf::default();
        output
            .decode(&mut decoder, encoder.as_writer().len())
            .unwrap();
        assert_eq!(msg, output);
    }

<<<<<<< HEAD
    fn test_proto_roundtrip(msg: micropb_types::TestOneOf) {
        use protobuf::Message;

        let mut encoder = PbEncoder::new(vec![]);
        msg.encode(&mut encoder).unwrap();

        println!("{:0x?}", encoder.as_writer().as_slice());
        let proto_msg =
            proto_types::all_types::TestOneOf::parse_from_bytes(encoder.as_writer().as_slice())
                .unwrap();
        dbg!(&proto_msg);
        let buf = proto_msg.write_to_bytes().unwrap();
        println!("{:0x?}", buf);

        let mut decoder = PbDecoder::new(buf.as_slice());
        decoder.ignore_wrong_len = true;
        let mut output = micropb_types::TestOneOf::default();
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

    #[test]
    fn negative_enum() {
        use micropb_types::*;

        let msg = TestOneOf {
            inner: Some(TestOneOf_::Inner::Repeat2(TestTypesRepeated2 {
                fixed32_field: vec![],
                fixed64_field: vec![],
                sfixed32_field: vec![],
                sfixed64_field: vec![],
                bool_field: vec![],
                string_field: vec![],
                bytes_field: vec![],
                enum_field: vec![TestEnum(-1), TestEnum(0)],
                message_field: vec![],
            })),
        };
        test_proto_roundtrip(msg);
    }
}
