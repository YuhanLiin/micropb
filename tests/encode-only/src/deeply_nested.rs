use core::cell::Cell;

use micropb::{heapless, FieldEncode, MessageEncode, PbEncoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/deeply_nested.rs"));
}

// Count the number of times the field size was computed
#[derive(Default, Clone, PartialEq, Debug, Eq)]
struct Counter(Cell<u32>);

impl FieldEncode for Counter {
    const MAX_SIZE: Result<usize, &str> = Ok(1);

    fn encode_fields<W: micropb::PbWrite>(
        &self,
        _encoder: &mut micropb::PbEncoder<W>,
    ) -> Result<(), W::Error> {
        Ok(())
    }

    fn compute_fields_size(&self) -> usize {
        self.0.set(self.0.get() + 1);
        1
    }
}

#[test]
fn cache_size_calcs() {
    let mk_elem =
        || proto::Either::Wrapper(proto::Wrapper::default().init_leaf(proto::Leaf::default()));

    let many1 = proto::Many {
        many: heapless::Vec::from_iter((0..4).map(|_| mk_elem())),
    };
    let many2 = proto::Many {
        many: heapless::Vec::from_iter(core::iter::once(mk_elem())),
    };
    let mut lookup = proto::Lookup::default();
    lookup.table.insert(12, many1).unwrap();
    lookup.table.insert(-2, many2).unwrap();

    let mut encoder = PbEncoder::new(heapless::Vec::<_, 64>::default());
    lookup.encode(&mut encoder).unwrap();

    // We don't care about the output of the encoder, since that's already tested elsewhere.
    // Instead we want to verify that the size computation for the nested fields was only called
    // once, which indicates that the size calculations are being cached.
    for elem in lookup.table.values().flat_map(|many| many.many.iter()) {
        let proto::Either::Wrapper(wrapper) = elem else {
            unreachable!()
        };
        assert_eq!(wrapper.leaf.field.0.get(), 1);
    }
}
