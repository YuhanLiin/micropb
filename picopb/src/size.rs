use crate::container::PbVec;

pub fn sizeof_varint32(v: u32) -> usize {
    match v {
        0x0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1FFFFF => 3,
        0x200000..=0xFFFFFFF => 4,
        _ => 5,
    }
}

pub fn sizeof_varint64(v: u64) -> usize {
    const U32_MAX: u64 = u32::MAX as u64;
    const U32_OVER_MAX: u64 = U32_MAX + 1;
    match v {
        0x0..=U32_MAX => sizeof_varint32(v as u32),
        U32_OVER_MAX..=0x7FFFFFFFF => 5,
        0x0800000000..=0x3FFFFFFFFFF => 6,
        0x040000000000..=0x1FFFFFFFFFFFF => 7,
        0x02000000000000..=0xFFFFFFFFFFFFFF => 8,
        0x0100000000000000..=0x7FFFFFFFFFFFFFFF => 9,
        _ => 10,
    }
}

pub fn sizeof_int32(i: i32) -> usize {
    if i >= 0 {
        sizeof_varint32(i as u32)
    } else {
        sizeof_varint64(i as u64)
    }
}

pub fn sizeof_int64(i: i64) -> usize {
    sizeof_varint64(i as u64)
}

pub fn sizeof_sint32(i: i32) -> usize {
    sizeof_varint32(((i << 1) ^ (i >> 31)) as u32)
}

pub fn sizeof_sint64(i: i64) -> usize {
    sizeof_varint64(((i << 1) ^ (i >> 63)) as u64)
}

pub fn sizeof_packed<T: Copy, S: PbVec<T>, F: Fn(T) -> usize>(vec: S, sizeof: F) -> usize {
    vec.iter().copied().map(sizeof).sum()
}

pub fn sizeof_packed_fixed<T: Copy, S: PbVec<T>>(vec: S) -> usize {
    let slice: &[_] = vec.deref();
    core::mem::size_of_val(slice)
}
