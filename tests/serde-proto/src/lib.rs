#[no_std]

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/serde_proto.rs"));
}

#[cfg(test)]
#[test]
fn serde_test() {
    let mut msg = proto::Data::default();
    msg.set_int(12);
    msg.set_s(micropb::heapless::String::try_from("abc").unwrap());
    msg.list
        .push(micropb::heapless::String::try_from("bc").unwrap())
        .unwrap();

    let buf = serde_json_core::ser::to_vec::<_, 200>(&msg).unwrap();
    let (decoded, len): (proto::Data, _) = serde_json_core::de::from_slice(&buf).unwrap();
    assert_eq!(len, buf.len());
    assert_eq!(decoded, msg);
}

#[cfg(test)]
#[test]
fn impl_eq() {
    fn is_eq<T: Eq>() {}
    is_eq::<proto::Data>();
}
