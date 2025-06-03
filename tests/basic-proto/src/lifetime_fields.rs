use micropb::FieldEncode;

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
