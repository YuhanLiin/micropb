use core::mem::{self, MaybeUninit};

pub(crate) fn maybe_uninit_write_slice<'a, T>(
    this: &'a mut [MaybeUninit<T>],
    src: &[T],
) -> &'a mut [T]
where
    T: Copy,
{
    // SAFETY: copy-paste from rust stdlib.
    let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };

    this.copy_from_slice(uninit_src);

    unsafe { &mut *(this as *mut [MaybeUninit<T>] as *mut [T]) }
}

#[inline]
pub(crate) unsafe fn maybe_ununit_array_assume_init<T, const N: usize>(
    array: [MaybeUninit<T>; N],
) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    (&array as *const _ as *const [T; N]).read()
}

#[inline]
pub(crate) unsafe fn maybe_uninit_slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    // SAFETY: casting `slice` to a `*const [T]` is safe since the caller guarantees that
    // `slice` is initialized, and `MaybeUninit` is guaranteed to have the same layout as `T`.
    // The pointer obtained is valid since it refers to memory owned by `slice` which is a
    // reference and thus guaranteed to be valid for reads.
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}
