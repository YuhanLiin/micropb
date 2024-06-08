//! Functions for calculating the size of Protobuf values on the wire, which is necessary for
//! encoding Protobuf messages.

use crate::Tag;

/// Calculate size of `uint32` on the wire.
pub const fn sizeof_varint32(v: u32) -> usize {
    match v {
        0x0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1FFFFF => 3,
        0x200000..=0xFFFFFFF => 4,
        _ => 5,
    }
}

/// Calculate size of `uint64` on the wire.
pub const fn sizeof_varint64(v: u64) -> usize {
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

#[inline]
/// Calculate size of `int32` on the wire.
pub const fn sizeof_int32(i: i32) -> usize {
    if i >= 0 {
        sizeof_varint32(i as u32)
    } else {
        10
    }
}

#[inline]
/// Calculate size of `int64` on the wire.
pub const fn sizeof_int64(i: i64) -> usize {
    sizeof_varint64(i as u64)
}

#[inline]
/// Calculate size of `sint32` on the wire.
pub const fn sizeof_sint32(i: i32) -> usize {
    sizeof_varint32(((i << 1) ^ (i >> 31)) as u32)
}

#[inline]
/// Calculate size of `sint64` on the wire.
pub const fn sizeof_sint64(i: i64) -> usize {
    sizeof_varint64(((i << 1) ^ (i >> 63)) as u64)
}

#[inline]
/// Calculate size of Protobuf tag on the wire.
pub const fn sizeof_tag(tag: Tag) -> usize {
    sizeof_varint32(tag.varint())
}

/// Calculate size of a repeated packed field on the wire. Does not include the length prefix.
///
/// ```
/// use micropb::size::*;
///
/// // Calculate size of a LEN record for a repeated packed field
/// let packed = &[1, 2, 150];
/// let size = sizeof_len_record(sizeof_packed(packed, |v| sizeof_int32(*v)));
/// assert_eq!(size, 4);
/// ```
pub fn sizeof_packed<T: Copy, F: Fn(&T) -> usize>(elems: &[T], sizer: F) -> usize {
    elems.iter().map(sizer).sum()
}

#[inline]
/// Calculate size of LEN record on the wire, including the length prefix.
pub const fn sizeof_len_record(len: usize) -> usize {
    len + sizeof_varint32(len as u32)
}

/// Calculate size of a key-value pair in a map. Does not include the length prefix.
pub fn sizeof_map_elem<K: ?Sized, V: ?Sized, FK: FnMut(&K) -> usize, FV: FnMut(&V) -> usize>(
    key: &K,
    val: &V,
    mut key_sizer: FK,
    mut val_sizer: FV,
) -> usize {
    // key and value field numbers are 1 and 2, so the tags will always be small numbers, so tag
    // sizes are 1 each
    2 + key_sizer(key) + val_sizer(val)
}
