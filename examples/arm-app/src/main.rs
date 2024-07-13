#![no_std]
#![no_main]

use core::{mem::size_of, str::FromStr};

use cortex_m_semihosting::{debug, hprintln};
use micropb::{
    heapless::{String, Vec},
    MessageDecode, MessageEncode, PbDecoder, PbEncoder,
};
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/arm-example-proto.rs"));
}

use proto::accel_::*;
use proto::google_::protobuf_::*;
use proto::gps_::*;
use proto::packet_::*;
use proto::raw_::*;

#[entry]
fn main() -> ! {
    hprintln!(
        "size of Packet = {}, Packet_::Msg = {}, LogBundle = {}, Log = {}",
        size_of::<Packet>(),
        size_of::<Packet_::Msg>(),
        size_of::<LogBundle>(),
        size_of::<Log>()
    )
    .unwrap();

    let init_pkt = Packet {
        msg: Some(Packet_::Msg::Init(Init {
            id: -1,
            version: String::from_str("1.73.0").unwrap(),
        })),
    };
    let logs_pkt = Packet {
        msg: Some(Packet_::Msg::Logs(LogBundle {
            logs: Vec::from_slice(&[
                Log {
                    time: Timestamp {
                        seconds: 1777674,
                        nanos: 2000,
                    },
                    msg: Some(Log_::Msg::Gps(Gps {
                        time: Timestamp {
                            seconds: 1777674,
                            nanos: 2000,
                        },
                        longitude: 79.882,
                        latitude: 120.234,
                        speed: 10.0,
                        _has: Gps_::_Hazzer::default().init_time(),
                    })),
                    _has: Log_::_Hazzer::default().init_time(),
                },
                Log {
                    time: Timestamp {
                        seconds: 1777675,
                        nanos: 0,
                    },
                    msg: Some(Log_::Msg::Accel(Accel {
                        acceleration: -2.56,
                        gyro: -5.987,
                    })),
                    _has: Log_::_Hazzer::default().init_time(),
                },
                Log {
                    time: Timestamp {
                        seconds: 1777676,
                        nanos: 1500,
                    },
                    msg: Some(Log_::Msg::Raw(RawMsg {
                        r#type: 12,
                        payload: Vec::from_slice(b"abcde").unwrap(),
                    })),
                    _has: Log_::_Hazzer::default().init_time(),
                },
            ])
            .unwrap(),
        })),
    };

    let mut stream = Vec::<u8, 100>::new();

    // Encode the init and logs packets to the stream
    let mut encoder = PbEncoder::new(&mut stream);
    init_pkt.encode_len_delimited(&mut encoder).unwrap();
    logs_pkt.encode_len_delimited(&mut encoder).unwrap();

    // Decode the init and logs packets from the stream
    let mut decoder = PbDecoder::new(stream.as_slice());
    let mut init_pkt_out = Packet::default();
    init_pkt_out.decode_len_delimited(&mut decoder).unwrap();
    let mut logs_pkt_out = Packet::default();
    logs_pkt_out.decode_len_delimited(&mut decoder).unwrap();

    // Don't use assert to check message equality, because debug printing logic bloats code size
    if init_pkt != init_pkt_out {
        // Uncomment to debug panic
        //hprintln!("init_pkt = {init_pkt:?}\ninit_pkt_out = {init_pkt_out:?}").unwrap();
        panic!("init package fails roundtrip test");
    }
    if logs_pkt != logs_pkt_out {
        // Uncomment to debug panic
        //hprintln!("logs_pkt = {logs_pkt:?}\nlogs_pkt_out = {logs_pkt_out:?}").unwrap();
        panic!("log package fails roundtrip test");
    }

    hprintln!("Example complete").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
