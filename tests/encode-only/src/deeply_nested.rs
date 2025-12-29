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
        encoder: &mut micropb::PbEncoder<W>,
    ) -> Result<(), W::Error> {
        encoder.encode_fixed32(0xDEADBEEF)?;
        Ok(())
    }

    fn compute_fields_size(&self) -> usize {
        self.0.set(self.0.get() + 1);
        4
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
    lookup.table.insert(true, many1).unwrap();
    lookup.table.insert(false, many2).unwrap();

    let mut encoder = PbEncoder::new(heapless::Vec::<_, 64>::default());
    lookup.encode_len_delimited(&mut encoder).unwrap();

    // Order of expected output relies on order of elements in the map, which isn't guaranteed
    assert_eq!(
        encoder.as_writer(),
        &[
            62, // length
            0x0A, 44, 0x08, 0x1, 0x12, 40, // first entry
            0x0A, 8, 0x12, 6, 0x0A, 4, 0xEF, 0xBE, 0xAD, 0xDE, // elem1
            0x0A, 8, 0x12, 6, 0x0A, 4, 0xEF, 0xBE, 0xAD, 0xDE, // elem2
            0x0A, 8, 0x12, 6, 0x0A, 4, 0xEF, 0xBE, 0xAD, 0xDE, // elem3
            0x0A, 8, 0x12, 6, 0x0A, 4, 0xEF, 0xBE, 0xAD, 0xDE, // elem4
            0x0A, 14, 0x08, 0x0, 0x12, 10, // second entry
            0x0A, 8, 0x12, 6, 0x0A, 4, 0xEF, 0xBE, 0xAD, 0xDE, // elem1
        ]
    );

    // Verify that the size computation for the nested fields was only called once, which indicates
    // that the size calculations are being cached.
    for elem in lookup.table.values().flat_map(|many| many.many.iter()) {
        let proto::Either::Wrapper(wrapper) = elem else {
            unreachable!()
        };
        assert_eq!(wrapper.leaf.field.0.get(), 1);
    }

    assert_eq!(lookup.compute_size(), 62);
}
