mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/static_lifetime_fields.rs"));
}

// Ensure that the generated structs other than Data don't require lifetime params
struct _Test {
    data: proto::Data<'static>,
    list: proto::List,
    numlist: proto::NumList,
    strlist: proto::StrList,
    map: proto::Map,
}

#[test]
fn ref_containers() {
    // Check that it's possible to create Data with a non-static field `b`
    let b = b"123".to_owned();
    let data = proto::Data {
        b: b.as_ref(),
        s: "abc",
        _has: proto::Data_::_Hazzer::default().init_b().init_s(),
    };
    let _: &'static str = data.s;

    static LIST: &[proto::Data] = &[proto::Data {
        b: b"123",
        s: "abc",
        _has: proto::Data_::_Hazzer::_new().init_b().init_s(),
    }];
    let list = proto::List { list: LIST };
    let _: &'static [proto::Data<'static>] = list.list;

    static NUMLIST: &[u32] = &[13];
    let numlist = proto::NumList { list: NUMLIST };
    let _: &'static [u32] = numlist.list;
}
