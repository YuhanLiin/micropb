use micropb::FieldEncode;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/lifetime_fields.rs"));
}

#[derive(Clone, PartialEq)]
struct RefField<'a>(&'a i32);

impl<'a> FieldEncode for RefField<'a> {
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
    let nested = proto::nested_::Nested::<'_> {
        inner: RefField(&5),
    };
    let _: RefField = nested.inner;

    let inner = proto::nested_::Nested_::InnerMsg {
        val: Default::default(),
        val2: Default::default(),
        _has: Default::default(),
        _unknown: Some(RefField(&12)),
    };
    let _: Option<RefField> = inner._unknown;
}
