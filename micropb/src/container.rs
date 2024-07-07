//! Traits for Rust representations of Protobuf `string`, `bytes`, repeated, and `map` fields.
//!
//! In order to represent `string`, `bytes`, repeated, and `map` fields in Rust, multi-element
//! containers are needed. To ensure flexibility, `micropb` interfaces with containers using traits
//! from this module rather than hard-coded types. This allows compatibility with different
//! container implementations. For example, `no-std` users can use fixed-capacity containers from
//! `heapless` or `arrayvec`, and `alloc` users can use dynamic-capacity containers from the
//! standard library.
//!
//! For convenience, container trait implementations on existing types are provided in this module,
//! gated by feature flags.
//!
//! - For `heapless`, [`PbVec`], [`PbString`], and [`PbMap`] are
//! implemented on `heapless::Vec`, `heapless::String`, and `heapless::IndexMap`
//! respectively.
//! - For `arrayvec`, [`PbVec`] and [`PbString`] are implemented on `arrayvec::ArrayVec` and
//! `arrayvec::ArrayString` respectively.
//! - For `alloc`, [`PbVec`], [`PbString`], and [`PbMap`] are implemented on `Vec`, `String`,
//! and `BTreeMap` respectively. If `std` is enabled, [`PbMap`] is also implemented for
//! `HashMap`.
//!
//! It is also possible to use other types as containers if the container traits are implemented.

#![allow(clippy::result_unit_err)]

use core::{mem::MaybeUninit, ops::Deref};

/// Basic container trait required for all multi-element containers, except for maps.
pub trait PbContainer: Sized {
    /// Sets length of container (number of elements).
    ///
    /// # Safety
    /// New length must be smaller than the total capacity, and the elements between the old and
    /// new lengths must be initialized and valid.
    unsafe fn pb_set_len(&mut self, len: usize);

    /// Reserves capacity for at least `additional` more elements to be inserted. No-op for
    /// fixed-capacity containers.
    fn pb_reserve(&mut self, _additional: usize) {}

    /// Clear all elements of the container
    fn pb_clear(&mut self);
}

/// Generic vector that stores multiple elements.
///
/// Represents repeated field. If `PbVec<u8>` is implemented, also represents `bytes` field.
pub trait PbVec<T>: PbContainer + Deref<Target = [T]> {
    /// Push a new element to the back of the vector.
    ///
    /// Returns error if the fixed capacity is already full.
    fn pb_push(&mut self, elem: T) -> Result<(), ()>;

    /// Returns the remaining spare capacity of the vector as a slice of `MaybeUninit<T>`.
    ///
    /// The returned slice can be filled with data before marking the data as initialized using
    /// [`pb_set_len`](PbContainer::pb_set_len).
    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>];

    /// Construct a vector from a slice.
    ///
    /// Returns error if the slice is longer than the fixed capacity of the vector type.
    fn pb_from_slice(s: &[T]) -> Result<Self, ()>
    where
        T: Clone;
}

/// UTF-8 string that stores characters.
///
/// Represents Protobuf `string` field.
pub trait PbString: PbContainer + Deref<Target = str> {
    /// Returns the remaining spare capacity of the string as a slice of `MaybeUninit<u8>`.
    ///
    /// The returned slice can be filled with bytes before marking the data as initialized using
    /// [`pb_set_len`](PbContainer::pb_set_len).
    ///
    /// # Safety
    /// When calling [`pb_set_len`](PbContainer::pb_set_len) after filling the spare capacity with
    /// bytes, the entirety of the new string must be valid UTF-8.
    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>];

    /// Constructs a string from a string slice.
    ///
    /// Returns error if the slice is longer than the fixed capacity of the string type.
    fn pb_from_str(s: &str) -> Result<Self, ()>;
}

/// Map that stores key-value pairs.
///
/// Represents Protobuf `map` field.
pub trait PbMap<K, V> {
    /// Iterator for looping through each key-value pair in the map
    type Iter<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    /// Inserts a new key-value pair into the map.
    ///
    /// Returns error if the new pair would make the map go over its fixed capacity.
    fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()>;

    /// Iterates through each key-value pair in the map. Order is unspecified.
    fn pb_iter(&self) -> Self::Iter<'_>;
}

#[cfg(feature = "container-arrayvec")]
mod impl_arrayvec {
    use core::ops::DerefMut;

    use super::*;

    use arrayvec::{ArrayString, ArrayVec};

    impl<T, const N: usize> PbContainer for ArrayVec<T, N> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }
    }

    impl<const N: usize> PbContainer for ArrayString<N> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }
    }

    impl<T, const N: usize> PbVec<T> for ArrayVec<T, N> {
        #[inline]
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.try_push(elem).map_err(drop)
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }

        #[inline]
        fn pb_from_slice(s: &[T]) -> Result<Self, ()>
        where
            T: Clone,
        {
            Self::try_from(s).map_err(drop)
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            let len = self.len();
            // Works in Miri with tree borrows, but not stack borrows due to provenance issues with
            // `deref_mut`
            self.clear();
            let s = self.deref_mut().as_mut_ptr();
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe { core::slice::from_raw_parts_mut(s as *mut MaybeUninit<u8>, N) };
            slice.get_mut(len..).unwrap_or(&mut [])
        }

        #[inline]
        fn pb_from_str(s: &str) -> Result<Self, ()> {
            Self::try_from(s).map_err(drop)
        }
    }
}

#[cfg(feature = "container-heapless")]
mod impl_heapless {
    use super::*;

    use core::hash::{BuildHasher, Hash};

    use heapless::{IndexMap, IndexMapIter, String, Vec};

    impl<T, const N: usize> PbContainer for Vec<T, N> {
        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }
    }

    impl<const N: usize> PbContainer for String<N> {
        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.as_mut_vec().set_len(len)
        }
    }

    impl<T, const N: usize> PbVec<T> for Vec<T, N> {
        #[inline]
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.push(elem).map_err(drop)
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }

        #[inline]
        fn pb_from_slice(s: &[T]) -> Result<Self, ()>
        where
            T: Clone,
        {
            Self::try_from(s).map_err(drop)
        }
    }

    impl<const N: usize> PbString for String<N> {
        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            let len = self.len();
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_mut_vec().as_mut_ptr() as *mut MaybeUninit<u8>,
                    N,
                )
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }

        #[inline]
        fn pb_from_str(s: &str) -> Result<Self, ()> {
            Self::try_from(s).map_err(drop)
        }
    }

    impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> PbMap<K, V> for IndexMap<K, V, S, N> {
        type Iter<'a> = IndexMapIter<'a, K, V> where S: 'a, K: 'a, V: 'a;

        #[inline]
        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val).map_err(drop)?;
            Ok(())
        }

        #[inline]
        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;

    use alloc::{
        borrow::ToOwned,
        collections::{btree_map, BTreeMap},
        string::String,
        vec::Vec,
    };

    impl<T> PbContainer for Vec<T> {
        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        #[inline]
        fn pb_reserve(&mut self, additional: usize) {
            self.reserve(additional)
        }
    }

    //impl<'a, T> PbContainer for alloc::borrow::Cow<'a, [T]>
    //where
    //[T]: ToOwned<Owned = Vec<T>>,
    //{
    //unsafe fn pb_set_len(&mut self, len: usize) {
    //self.to_mut().set_len(len);
    //}

    //fn pb_clear(&mut self) {
    //self.to_mut().clear()
    //}

    //fn pb_reserve(&mut self, additional: usize) {
    //self.to_mut().reserve(additional)
    //}
    //}

    impl PbContainer for String {
        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.as_mut_vec().set_len(len)
        }

        #[inline]
        fn pb_reserve(&mut self, additional: usize) {
            self.reserve(additional)
        }
    }

    //impl<'a> PbContainer for alloc::borrow::Cow<'a, str> {
    //unsafe fn pb_set_len(&mut self, len: usize) {
    //self.to_mut().as_mut_vec().set_len(len);
    //}

    //fn pb_clear(&mut self) {
    //self.to_mut().clear()
    //}

    //fn pb_reserve(&mut self, additional: usize) {
    //self.to_mut().reserve(additional)
    //}
    //}

    impl<T> PbVec<T> for Vec<T> {
        #[inline]
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.push(elem);
            Ok(())
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
            self.spare_capacity_mut()
        }

        #[inline]
        fn pb_from_slice(s: &[T]) -> Result<Self, ()>
        where
            T: Clone,
        {
            Ok(Self::from(s))
        }
    }

    //impl<'a, T> PbVec<T> for alloc::borrow::Cow<'a, [T]>
    //where
    //[T]: ToOwned<Owned = Vec<T>>,
    //{
    //fn pb_push(&mut self, elem: T) -> Result<(), ()> {
    //self.to_mut().push(elem);
    //Ok(())
    //}

    //fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<T>] {
    //self.to_mut().spare_capacity_mut()
    //}
    //}

    impl PbString for String {
        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: spare_capacity_mut() is a safe call, since it doesn't change any bytes
            unsafe { self.as_mut_vec().spare_capacity_mut() }
        }

        #[inline]
        fn pb_from_str(s: &str) -> Result<Self, ()> {
            Ok(s.to_owned())
        }
    }

    //impl<'a> PbString for alloc::borrow::Cow<'a, str> {
    //fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
    // // SAFETY: spare_capacity_mut() is a safe call, since it doesn't change any bytes
    //unsafe { self.to_mut().as_mut_vec().spare_capacity_mut() }
    //}
    //}

    impl<K: Ord, V> PbMap<K, V> for BTreeMap<K, V> {
        type Iter<'a> = btree_map::Iter<'a, K, V> where K: 'a, V: 'a;

        #[inline]
        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }

        #[inline]
        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }
    }

    #[cfg(feature = "std")]
    impl<K: Eq + core::hash::Hash, V> PbMap<K, V> for std::collections::HashMap<K, V> {
        type Iter<'a> = std::collections::hash_map::Iter<'a, K, V> where K: 'a, V: 'a;

        #[inline]
        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }

        #[inline]
        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }
    }
}
