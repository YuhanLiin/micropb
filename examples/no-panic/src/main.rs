use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use no_panic::no_panic;

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/gps_example.rs"));
}

fn gps_point(gps: proto::gps::Gps) -> proto::gps::mod_LocationData::Point {
    use proto::gps::mod_LocationData::*;

    Point {
        point: Some(mod_Point::Point::Gps(gps)),
    }
}

fn accel_point(accel: proto::gps::Accel) -> proto::gps::mod_LocationData::Point {
    use proto::gps::mod_LocationData::*;

    Point {
        point: Some(mod_Point::Point::Accel(accel)),
    }
}

#[inline]
fn raw_point(raw: &[u8]) -> proto::gps::mod_LocationData::Point {
    use proto::gps::mod_LocationData::*;

    Point {
        point: Some(mod_Point::Point::Raw(raw.try_into().unwrap())),
    }
}

#[no_panic]
fn round_trip() -> Result<(proto::gps::LocationData, proto::gps::LocationData), &'static str> {
    let mut points = micropb::heapless::Vec::new();
    points
        .push(gps_point(proto::gps::Gps {
            time: 165547,
            longitude: -79.012343,
            latitude: 45.092345,
            speed: 45.0,
            altitude: 7.8,
        }))
        .unwrap();
    points
        .push(accel_point({
            let mut a = proto::gps::Accel {
                time: 165577,
                accel: -4.5,
                ..Default::default()
            };
            a.set_gyro(2.1);
            a
        }))
        .unwrap();
    points.push(raw_point(b"abcdefg")).unwrap();

    let mut time_to_type = micropb::heapless::FnvIndexMap::new();
    // Can't unwrap these inserts, since the compiler will generate panics here
    let _ = time_to_type.insert(165547, proto::gps::mod_LocationData::Type::Gps);
    let _ = time_to_type.insert(165577, proto::gps::mod_LocationData::Type::Accel);
    let _ = time_to_type.insert(165578, proto::gps::mod_LocationData::Type::Raw);

    let input_location = proto::gps::LocationData {
        checksum: 0xDEADBEEF,
        comment: micropb::heapless::String::try_from("nice").unwrap(),
        points,
        time_to_type,
    };

    let mut encoder = PbEncoder::new(micropb::heapless::Vec::<u8, 100>::new());
    input_location
        .encode(&mut encoder)
        .map_err(|_| "Encode error")?;
    let data = encoder.into_writer();

    let mut decoder = PbDecoder::new(data.as_slice());
    let mut output_location = proto::gps::LocationData::default();
    output_location
        .decode(&mut decoder, data.len())
        .map_err(|_| "Decode error")?;

    Ok((input_location, output_location))
}

fn main() {
    let (input_location, output_location) = round_trip().unwrap();
    assert_eq!(
        input_location, output_location,
        "encoded and decoded values should be equal"
    );
}
