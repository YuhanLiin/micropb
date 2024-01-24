#![allow(clippy::result_unit_err)]

use core::{mem::MaybeUninit, ops::DerefMut};

pub trait PbContainer: Default {
    unsafe fn pb_set_len(&mut self, len: usize);

    /// Reserves capacity for at least `additional` more elements to be inserted. No-op for
    /// fixed-capacity containers.
    fn pb_reserve(&mut self, _additional: usize) {}

    fn pb_clear(&mut self);
}

pub trait PbVec<T>: PbContainer + DerefMut<Target = [T]> {
    fn pb_push(&mut self, elem: T) -> Result<(), ()>;

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>];
}

pub trait PbString: PbContainer + DerefMut<Target = str> {
    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>];
}

#[cfg(feature = "container-arrayvec")]
mod impl_arrayvec {
    use crate::encode::PbWrite;

    use super::*;

    use arrayvec::{ArrayString, ArrayVec, CapacityError};

    impl<T, const N: usize> PbContainer for ArrayVec<T, N> {
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        fn pb_clear(&mut self) {
            self.clear()
        }
    }

    impl<const N: usize> PbContainer for ArrayString<N> {
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        fn pb_clear(&mut self) {
            self.clear()
        }
    }

    impl<T, const N: usize> PbVec<T> for ArrayVec<T, N> {
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.try_push(elem).map_err(drop)
        }

        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            &mut slice[self.len()..]
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_bytes_mut().as_mut_ptr() as *mut MaybeUninit<u8>,
                    N,
                )
            };
            &mut slice[self.len()..]
        }
    }

    impl<const N: usize> PbWrite for ArrayVec<u8, N> {
        type Error = CapacityError;

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.try_extend_from_slice(data)
        }
    }
}

#[cfg(feature = "container-heapless")]
mod impl_heapless {
    use crate::encode::PbWrite;

    use super::*;

    use heapless::{String, Vec};

    impl<T, const N: usize> PbContainer for Vec<T, N> {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }
    }

    impl<const N: usize> PbContainer for String<N> {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.as_mut_vec().set_len(len)
        }
    }

    impl<T, const N: usize> PbVec<T> for Vec<T, N> {
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.push(elem).map_err(drop)
        }

        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            &mut slice[self.len()..]
        }
    }

    impl<const N: usize> PbString for String<N> {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_bytes_mut().as_mut_ptr() as *mut MaybeUninit<u8>,
                    N,
                )
            };
            &mut slice[self.len()..]
        }
    }

    impl<const N: usize> PbWrite for Vec<u8, N> {
        type Error = ();

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.extend_from_slice(data)
        }
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;

    use alloc::{string::String, vec::Vec};

    impl<T> PbContainer for Vec<T> {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        fn pb_reserve(&mut self, additional: usize) {
            self.reserve(additional)
        }
    }

    impl PbContainer for String {
        fn pb_clear(&mut self) {
            self.clear()
        }

        unsafe fn pb_set_len(&mut self, len: usize) {
            self.as_mut_vec().set_len(len)
        }

        fn pb_reserve(&mut self, additional: usize) {
            self.reserve(additional)
        }
    }

    impl<T> PbVec<T> for Vec<T> {
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.push(elem);
            Ok(())
        }

        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            self.spare_capacity_mut()
        }
    }

    impl PbString for String {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: spare_capacity_mut() is a safe call, since it doesn't change any bytes
            unsafe { self.as_mut_vec().spare_capacity_mut() }
        }
    }
}
