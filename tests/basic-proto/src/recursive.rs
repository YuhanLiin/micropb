use micropb::{DecodeError, MessageDecode, MessageEncode, PbEncoder, DEFAULT_NESTING_LIMIT};
use proto::Recursive;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/recursive.rs"));
}

#[test]
fn recursive_types() {
    let mut recursive = proto::Recursive::default();
    let _: &Option<Box<proto::Recursive>> = &recursive.recursive;
    let _: &Option<proto::Recursive_::Of> = &recursive.of;
    let _: &Vec<proto::Recursive> = &recursive.multi; // shouldn't be boxed
    recursive.of = Some(proto::Recursive_::Of::Num(1));
    match recursive.of.unwrap() {
        proto::Recursive_::Of::Rec(r) => {
            let _: Box<Recursive> = r;
        }
        proto::Recursive_::Of::Num(i) => {
            let _: i32 = i;
        }
    }

    assert_eq!(
        proto::Recursive::MAX_SIZE,
        Err("(.Recursive.recursive) cyclical reference")
    );
}

fn nested(levels: usize) -> Recursive {
    let mut msg = Recursive::default();
    for _ in 0..levels {
        msg = Recursive {
            recursive: Some(Box::new(msg)),
            ..Default::default()
        };
    }
    msg
}

fn encode(msg: &Recursive) -> Vec<u8> {
    let mut encoder = PbEncoder::new(Vec::new());
    msg.encode(&mut encoder).unwrap();
    encoder.into_writer()
}

#[test]
fn recursive_nesting_limit() {
    let ok = encode(&nested(DEFAULT_NESTING_LIMIT));
    Recursive::default().decode_from_bytes(&ok).unwrap();

    // The oneof and repeated fields go through the same check.
    let via_oneof = Recursive {
        of: Some(proto::Recursive_::Of::Rec(Box::new(nested(
            DEFAULT_NESTING_LIMIT,
        )))),
        ..Default::default()
    };
    assert_eq!(
        Recursive::default().decode_from_bytes(&encode(&via_oneof)),
        Err(DecodeError::NestingLimit)
    );
    let via_repeated = Recursive {
        multi: vec![nested(DEFAULT_NESTING_LIMIT)],
        ..Default::default()
    };
    assert_eq!(
        Recursive::default().decode_from_bytes(&encode(&via_repeated)),
        Err(DecodeError::NestingLimit)
    );

    let too_deep = encode(&nested(DEFAULT_NESTING_LIMIT + 1));
    assert_eq!(
        Recursive::default().decode_from_bytes(&too_deep),
        Err(DecodeError::NestingLimit)
    );
}
