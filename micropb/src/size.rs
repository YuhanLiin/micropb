use crate::Tag;

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

pub fn sizeof_tag(tag: Tag) -> usize {
    sizeof_varint32(tag.varint())
}

pub fn sizeof_packed<T: Copy, F: Fn(&T) -> usize>(elems: &[T], sizer: F) -> usize {
    elems.iter().map(sizer).sum()
}

//pub fn sizeof_packed_fixed<T: Copy>(slice: &[T]) -> usize {
//core::mem::size_of_val(slice)
//}

pub fn sizeof_len_record(len: usize) -> usize {
    len + sizeof_varint32(len as u32)
}

pub fn sizeof_map_elem<K: ?Sized, V: ?Sized, FK: FnMut(&K) -> usize, FV: FnMut(&V) -> usize>(
    key: &K,
    val: &V,
    mut key_sizer: FK,
    mut val_sizer: FV,
) -> usize {
    // key and value field numbers are 1 and 2, so the tags will always be small numbers, so tag
    // sizes are 1 each
    sizeof_len_record(2 + key_sizer(key) + val_sizer(val))
}

pub fn sizeof_repeated_with_tag<T, F: FnMut(T) -> usize>(
    tag: Tag,
    elems: impl IntoIterator<Item = T>,
    mut sizer: F,
) -> usize {
    let tag_size = sizeof_tag(tag);
    elems.into_iter().map(|e| tag_size + sizer(e)).sum()
}

pub trait SizeCache {
    fn set_total_size(&mut self, size: usize);

    fn get_total_size(&self) -> Option<usize>;

    fn set_field_size(&mut self, field_num: u32, size: usize);

    fn get_field_size(&mut self, field_num: u32) -> Option<usize>;
}
