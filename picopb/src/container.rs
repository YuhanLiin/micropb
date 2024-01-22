#![allow(clippy::result_unit_err)]

use core::ops::{Deref, DerefMut};

pub trait PbVec<T>: Default + Deref<Target = [T]> + DerefMut<Target = [T]> {
    fn push(&mut self, elem: T) -> Result<(), ()>;

    fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), ()>
    where
        T: Copy;

    fn write_slice(&mut self, slice: &[T]) -> Result<(), ()>
    where
        T: Copy;
}

pub trait PbString: Default + Deref<Target = str> + DerefMut<Target = str> {
    fn write_str(&mut self, s: &str) -> Result<(), ()>;
}

#[cfg(feature = "container-arrayvec")]
mod impl_arrayvec {
    use crate::encode::PbWrite;

    use super::*;

    use arrayvec::{ArrayString, ArrayVec, CapacityError};

    impl<T, const N: usize> PbVec<T> for ArrayVec<T, N> {
        fn push(&mut self, elem: T) -> Result<(), ()> {
            self.try_push(elem).map_err(drop)
        }

        fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), ()>
        where
            T: Copy,
        {
            self.try_extend_from_slice(slice).map_err(drop)
        }

        fn write_slice(&mut self, slice: &[T]) -> Result<(), ()>
        where
            T: Copy,
        {
            self.clear();
            self.try_extend_from_slice(slice).map_err(drop)
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        fn write_str(&mut self, s: &str) -> Result<(), ()> {
            self.clear();
            self.try_push_str(s).map_err(drop)
        }
    }

    impl<const N: usize> PbWrite for ArrayVec<u8, N> {
        type Error = CapacityError;

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.try_extend_from_slice(data)
        }
    }
}
