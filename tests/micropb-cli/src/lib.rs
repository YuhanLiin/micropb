mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/cli_test.rs"));
}

#[cfg(test)]
mod tests {
    use micropb::{MessageDecode, MessageEncode, PbEncoder};

    use super::proto;

    #[test]
    fn cli_config_changes_generated_type_and_message_round_trips() {
        let message = proto::cli_test::Sample {
            id: 42,
            values: vec![7, 11, 19],
        };
        let _: u8 = message.id;

        let mut encoder = PbEncoder::new(Vec::new());
        message.encode(&mut encoder).unwrap();
        let bytes = encoder.into_writer();

        let mut decoded = proto::cli_test::Sample::default();
        decoded.decode_from_bytes(&bytes).unwrap();
        assert_eq!(decoded, message);
    }

    #[test]
    fn cli_enum() {
        assert_eq!(
            proto::cli_test::State::StateUnknown,
            proto::cli_test::State(0)
        );
        assert_eq!(
            proto::cli_test::State::StateReady,
            proto::cli_test::State(1)
        );
    }
}
