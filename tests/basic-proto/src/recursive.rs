use proto::Recursive;

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/recursive.rs"));
}

#[test]
fn recursive_types() {
    let mut recursive = proto::Recursive::default();
    let _: &Option<Box<proto::Recursive>> = &recursive.recursive;
    let _: &Option<Box<proto::mod_Recursive::Of>> = &recursive.of;
    recursive.of = Some(Box::new(proto::mod_Recursive::Of::Num(1)));
    match *recursive.of.unwrap() {
        proto::mod_Recursive::Of::Rec(r) => {
            let _: Box<Recursive> = r;
        }
        proto::mod_Recursive::Of::Num(i) => {
            let _: i32 = i;
        }
    }
}
