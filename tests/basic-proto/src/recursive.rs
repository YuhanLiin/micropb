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
    let _: &Option<Box<proto::Recursive_::Of>> = &recursive.of;
    recursive.of = Some(Box::new(proto::Recursive_::Of::Num(1)));
    match *recursive.of.unwrap() {
        proto::Recursive_::Of::Rec(r) => {
            let _: Box<Recursive> = r;
        }
        proto::Recursive_::Of::Num(i) => {
            let _: i32 = i;
        }
    }
}
