use std::collections::HashMap;

use micropb::{FieldEncode, MessageEncode, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/lifetime_fields.rs"));
}

#[derive(Clone, PartialEq)]
struct RefField<'a>(&'a i32);

impl FieldEncode for RefField<'_> {
    const MAX_SIZE: Option<usize> = None;

    fn encode_fields<W: micropb::PbWrite>(
        &self,
        _encoder: &mut micropb::PbEncoder<W>,
    ) -> Result<(), W::Error> {
        unimplemented!()
    }

    fn compute_fields_size(&self) -> usize {
        unimplemented!()
    }
}

#[test]
fn type_check() {
    let inner = proto::nested_::Nested_::InnerMsg {
        val: Default::default(),
        val2: Default::default(),
        _has: Default::default(),
        _unknown: Some(RefField(&12)),
    };
    let _: Option<RefField> = inner._unknown;

    let nested = proto::nested_::Nested::<'_> { inner: None };
    let _: Option<proto::nested_::Nested_::Inner<'_>> = nested.inner;
}

#[test]
fn ref_containers() {
    // Declare data as local variables to ensure that the message fields are declared with
    // non-static references
    let b = b"123".to_owned();
    let s = "abc".to_owned();
    let list1 = [34, 150];
    let list2 = ["ab", "cd"];

    let data = proto::Data {
        b: b.as_ref(),
        s: s.as_str(),
        _has: proto::Data_::_Hazzer::default().init_b().init_s(),
    };
    let list = proto::List { list: &[data] };
    let mut encoder = PbEncoder::new(vec![]);
    list.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.as_writer(),
        &[0x0A, 10, 0x0A, 3, b'a', b'b', b'c', 0x12, 3, b'1', b'2', b'3']
    );

    let num_list = proto::NumList { list: &list1[..] };
    let mut encoder = PbEncoder::new(vec![]);
    num_list.encode(&mut encoder).unwrap();
    assert_eq!(encoder.as_writer(), &[0x08, 34, 0x08, 0x96, 0x01]);

    let str_list = proto::StrList { list: &list2[..] };
    let mut encoder = PbEncoder::new(vec![]);
    str_list.encode(&mut encoder).unwrap();
    assert_eq!(
        encoder.as_writer(),
        &[0x0A, 2, b'a', b'b', 0x0A, 2, b'c', b'd']
    );

    let map = proto::Map {
        mapping: &HashMap::from([("x", b"y".as_slice())]),
    };
    let mut encoder = PbEncoder::new(vec![]);
    map.encode(&mut encoder).unwrap();
    assert_eq!(encoder.as_writer(), &[0xA, 6, 0xA, 1, b'x', 0x12, 1, b'y']);
}
