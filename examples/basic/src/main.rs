use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};

mod example {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    // Let's assume that Example is the only message define in the .proto file that has been
    // converted into a Rust struct
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

fn main() {
    let example = example::Example {
        f_int32: 12,
        f_bool: true,
        f_float: 0.234,
        ..Default::default()
    };

    // Maximum size of the message type on the wire, scaled to the next power of 2 due to heapless::Vec
    const CAPACITY: usize = example::Example::MAX_SIZE.unwrap().next_power_of_two();
    // For the example message above we can use a smaller capacity
    // const CAPACITY: usize = 32;

    // Use heapless::Vec as the output stream and build an encoder around it
    let mut encoder = PbEncoder::new(micropb::heapless::Vec::<u8, CAPACITY>::new());

    // Compute the size of the `Example` on the wire
    let _size = example.compute_size();
    // Encode the `Example` to the data stream
    example.encode(&mut encoder).expect("Vec over capacity");

    let data = encoder.into_writer();
    // Construct new decoder from byte slice
    let mut decoder = PbDecoder::new(data.as_slice());

    // Decode a new instance of `Example` into a new struct
    let mut new = example::Example::default();
    new.decode(&mut decoder, data.len())
        .expect("decoding failed");
    assert_eq!(example, new);
}
