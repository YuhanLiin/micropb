use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use no_panic::no_panic;

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/gps_example.rs"));
}

fn gps_point(gps: proto::gps_::Gps) -> proto::gps_::LocationData_::Point {
    use proto::gps_::LocationData_::*;

    Point {
        point: Some(Point_::Point::Gps(gps)),
    }
}

fn accel_point(accel: proto::gps_::Accel) -> proto::gps_::LocationData_::Point {
    use proto::gps_::LocationData_::*;

    Point {
        point: Some(Point_::Point::Accel(accel)),
    }
}

#[inline]
fn raw_point(raw: &[u8]) -> proto::gps_::LocationData_::Point {
    use proto::gps_::LocationData_::*;

    Point {
        point: Some(Point_::Point::Raw(raw.try_into().unwrap())),
    }
}

#[no_panic]
fn round_trip() -> Result<(proto::gps_::LocationData, proto::gps_::LocationData), &'static str> {
    let mut points = micropb::heapless::Vec::new();
    points
        .push(gps_point(proto::gps_::Gps {
            time: 165547,
            longitude: -79.012343,
            latitude: 45.092345,
            speed: 45.0,
            altitude: 7.8,
        }))
        .unwrap();
    points
        .push(accel_point({
            let mut a = proto::gps_::Accel {
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
    let _ = time_to_type.insert(165547, proto::gps_::LocationData_::Type::Gps);
    let _ = time_to_type.insert(165577, proto::gps_::LocationData_::Type::Accel);
    let _ = time_to_type.insert(165578, proto::gps_::LocationData_::Type::Raw);

    let input_location = proto::gps_::LocationData {
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
    let mut output_location = proto::gps_::LocationData::default();
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
