use micropb::{FieldDecode, FieldEncode, Tag};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/with_config_file.rs"));
}

#[derive(Default, PartialEq, Debug, Clone)]
struct CustomField {}

impl FieldDecode for CustomField {
    fn decode_field<R: micropb::PbRead>(
        &mut self,
        _tag: Tag,
        _decoder: &mut micropb::PbDecoder<R>,
    ) -> Result<bool, micropb::DecodeError<R::Error>> {
        unimplemented!()
    }
}

impl FieldEncode for CustomField {
    const MAX_SIZE: Result<usize, &'static str> = Ok(0);

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
    let data = proto::Data::default();
    let _: micropb::heapless::String<4> = data.s;
    let _: micropb::heapless::Vec<u8, 6> = data.b;

    let list = proto::List::default();
    let _: micropb::heapless::Vec<_, 4> = list.list;

    let strlist = proto::StrList::default();
    let _: micropb::heapless::Vec<micropb::heapless::String<4>, 2> = strlist.list;

    let fixedlist = proto::FixedList::default();
    let _: micropb::heapless::Vec<_, 4> = fixedlist.list;

    let numlist = proto::NumList::default();
    let _: micropb::heapless::Vec<_, 8> = numlist.list;

    let enumlist = proto::EnumList::default();
    let _: micropb::heapless::Vec<_, 2> = enumlist.list;

    let map = proto::Map::default();
    let _: std::collections::BTreeMap<micropb::heapless::String<4>, micropb::heapless::Vec<u8, 3>> =
        map.mapping;

    let basic = proto::basic_::BasicTypes::default();
    let _: i8 = basic.int32_num;
    let _: Option<u32> = basic.uint32_num;
    let _: CustomField = basic.sint32_num;
}
