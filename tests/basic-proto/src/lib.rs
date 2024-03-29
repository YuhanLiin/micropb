mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn enum_test() {
        assert_eq!(proto::basic::Enum::Zero, proto::basic::Enum(0));
        assert_eq!(proto::basic::Enum::One, proto::basic::Enum(1));
        assert_eq!(proto::basic::Enum::Two, proto::basic::Enum(2));
        assert_eq!(proto::basic::Enum::Two, proto::basic::Enum::default());
        assert_eq!(size_of::<proto::basic::Enum>(), size_of::<i32>());
        let _: i32 = proto::basic::Enum(0).0;
    }

    #[test]
    fn basic_msg() {
        let mut basic = proto::basic::BasicTypes::default();
        assert!(!basic._has.dbl());
        assert_eq!(basic.dbl, 0.0);
        assert!(!basic._has.flt());
        assert_eq!(basic.flt, 0.0);
        assert!(!basic._has.boolean());
        assert!(!basic.boolean);
        assert!(!basic._has.int32_num());
        assert_eq!(basic.int32_num, 0);
        assert!(!basic._has.int64_num());
        assert_eq!(basic.int64_num, 0);
        assert!(!basic._has.enumeration());
        assert_eq!(basic.enumeration, proto::basic::Enum::Two);

        basic.enumeration = proto::basic::Enum::One;
        basic._has.set_enumeration(true);
        assert!(basic._has.enumeration());
        assert_eq!(basic.enumeration, proto::basic::Enum::One);
    }

    #[test]
    fn basic_type_check() {
        let basic = proto::basic::BasicTypes::default();
        let _: i32 = basic.int32_num;
        let _: i64 = basic.int64_num;
        let _: u32 = basic.uint32_num;
        let _: u64 = basic.uint64_num;
        let _: i32 = basic.sint32_num;
        let _: i64 = basic.sint64_num;
        let _: u32 = basic.fixed32_num;
        let _: u64 = basic.fixed64_num;
        let _: i32 = basic.sfixed32_num;
        let _: i64 = basic.sfixed64_num;
        let _: bool = basic.boolean;
        let _: f32 = basic.flt;
        let _: f64 = basic.dbl;
    }
}
