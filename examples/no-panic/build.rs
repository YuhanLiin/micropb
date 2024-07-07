use micropb_gen::{Config, Generator};

fn main() {
    let mut gen = Generator::new();
    gen.use_container_heapless()
        .configure(".gps.LocationData", Config::new().max_len(4).max_bytes(8))
        .compile_protos(
            &["gps.proto"],
            std::env::var("OUT_DIR").unwrap() + "/gps_example.rs",
        )
        .unwrap();
}
