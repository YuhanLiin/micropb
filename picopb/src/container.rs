#![allow(clippy::result_unit_err)]

use core::{mem::MaybeUninit, ops::DerefMut};

use crate::decode::DecodeFixedSize;

pub trait PbContainer: Default {
    fn pb_clear(&mut self);

    unsafe fn pb_set_len(&mut self, len: usize);

    /// Reserves capacity for at least `additional` more elements to be inserted. No-op for
    /// fixed-capacity containers.
    fn pb_reserve(&mut self, _additional: usize) {}
}

pub trait PbVec<T>: PbContainer + DerefMut<Target = [T]> {
    fn pb_push(&mut self, elem: T) -> Result<(), ()>;

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>]
    where
        T: DecodeFixedSize;
}

pub trait PbString: PbContainer + DerefMut<Target = str> {
    fn pb_cap_bytes(&mut self) -> &mut [MaybeUninit<u8>];
}

#[cfg(feature = "container-arrayvec")]
mod impl_arrayvec {
    use core::mem::size_of;

    use crate::encode::PbWrite;

    use super::*;

    use arrayvec::{ArrayString, ArrayVec, CapacityError};

    impl<T, const N: usize> PbContainer for ArrayVec<T, N> {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }
    }

    impl<const N: usize> PbContainer for ArrayString<N> {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }
    }

    impl<T, const N: usize> PbVec<T> for ArrayVec<T, N> {
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.try_push(elem).map_err(drop)
        }

        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>]
        where
            T: DecodeFixedSize,
        {
            let cap_bytes = N * size_of::<T>();
            let len_bytes = self.len() * size_of::<T>();
            // SAFETY: Underlying storage is static array of size N, so cap_bytes must be allocated
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_mut_ptr() as *mut MaybeUninit<u8>,
                    cap_bytes,
                )
            };
            &mut slice[len_bytes..]
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        fn pb_cap_bytes(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_bytes_mut().as_mut_ptr() as *mut MaybeUninit<u8>,
                    N,
                )
            }
        }
    }

    impl<const N: usize> PbWrite for ArrayVec<u8, N> {
        type Error = CapacityError;

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.try_extend_from_slice(data)
        }
    }
}
