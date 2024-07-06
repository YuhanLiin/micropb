use proto::Recursive;

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/recursive.rs"));
}

#[test]
fn recursive_types() {
    let recursive = proto::Recursive::default();
    let _: Option<Box<proto::Recursive>> = recursive.recursive;
    match recursive.of {
        Some(proto::mod_Recursive::Of::Rec(r)) => {
            let _: Box<Recursive> = r;
        }
        Some(proto::mod_Recursive::Of::Num(i)) => {
            let _: i32 = i;
        }
        None => {}
    }
}
