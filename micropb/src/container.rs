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
//! - For `heapless`, [`PbString`], [`PbBytes`], [`PbVec`], and [`PbMap`] are
//!   implemented on `heapless::String`, `heapless::Vec`, and `heapless::IndexMap`.
//! - For `arrayvec`, [`PbString`], [`PbBytes`], and [`PbVec`] are implemented on
//!   `arrayvec::ArrayString` and `arrayvec::ArrayVec`.
//! - For `alloc`, [`PbString`], [`PbBytes`], [`PbVec`], and [`PbMap`] are implemented on `String`,
//!   `Vec`, and `BTreeMap`. If `std` is enabled, [`PbMap`] is also implemented for `HashMap`.
//!
//! It is also possible to use other types as containers if the container traits are implemented.

#![allow(clippy::result_unit_err)]

use core::{mem::MaybeUninit, ops::Deref};

/// Container that stores a string. Represents Protobuf `string` field.
///
/// # Safety
/// The bytes in the container must be valid UTF-8.
pub trait PbString {
    /// Sets length of container (number of elements).
    ///
    /// # Safety
    /// New length must be smaller than the total capacity, and the elements between the old and
    /// new lengths must be initialized and valid UTF-8.
    unsafe fn pb_set_len(&mut self, len: usize);

    /// Reserves capacity for at least `additional` more elements to be inserted. No-op for
    /// fixed-capacity containers.
    fn pb_reserve(&mut self, _additional: usize) {}

    /// Clear all elements of the container
    fn pb_clear(&mut self);

    /// Returns the remaining spare capacity of the string as a slice of `MaybeUninit<u8>`.
    ///
    /// The returned slice can be filled with bytes before marking the data as initialized using
    /// [`pb_set_len`](PbString::pb_set_len).
    ///
    /// # Safety
    /// When calling [`pb_set_len`](PbString::pb_set_len) after filling the spare capacity with
    /// bytes, the entirety of the new string must be valid UTF-8.
    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>];
}

impl<T: PbString> PbString for &mut T {
    unsafe fn pb_set_len(&mut self, len: usize) {
        (*self).pb_set_len(len);
    }

    fn pb_clear(&mut self) {
        (*self).pb_clear();
    }

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
        (*self).pb_spare_cap()
    }

    fn pb_reserve(&mut self, additional: usize) {
        (*self).pb_reserve(additional);
    }
}

/// Container that stores a sequence of arbitrary bytes. Represents Protobuf `bytes` field.
///
/// `PbBytes` is a subtrait of [`PbString`] because the set of containers that can safely hold any
/// byte pattern is a subset of the set of containers that can safely hold a UTF-8 byte pattern.
/// **As such, this trait essentially "lifts" the UTF-8 safety constraint from `PbString`.**
/// Container types used for `bytes` fields will implement both `PbString` and `PbBytes`.
pub trait PbBytes: PbString {}

impl<T: PbBytes> PbBytes for &mut T {}

/// Generic vector that stores multiple elements. Represents repeated field.
pub trait PbVec<T> {
    /// Push a new element to the back of the vector.
    ///
    /// Returns error if the fixed capacity is already full.
    fn pb_push(&mut self, elem: T) -> Result<(), ()>;
}

impl<T, V: PbVec<T>> PbVec<T> for &mut V {
    fn pb_push(&mut self, elem: T) -> Result<(), ()> {
        (*self).pb_push(elem)
    }
}

/// Map that stores key-value pairs. Represents Protobuf `map` field.
pub trait PbMap<K, V> {
    /// Inserts a new key-value pair into the map.
    ///
    /// Returns error if the new pair would make the map go over its fixed capacity.
    fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()>;
}

impl<K, V, M: PbMap<K, V>> PbMap<K, V> for &mut M {
    fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
        (*self).pb_insert(key, val)
    }
}

pub(crate) mod impl_fixed_len {
    use core::{array::TryFromSliceError, ops::DerefMut};

    use super::*;

    /// String with fixed length, used for representing Protobuf `string` fields with constant size.
    ///
    /// Length information is not included in the string, so this type saves memory compared to
    /// dynamically-sized strings, even those with fixed capacity.
    ///
    /// Default value is all null bytes.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct FixedLenString<const N: usize>([u8; N]);

    impl<const N: usize> Default for FixedLenString<N> {
        #[inline]
        fn default() -> Self {
            Self([0; N])
        }
    }

    impl<const N: usize> PbString for FixedLenString<N> {
        #[inline]
        unsafe fn pb_set_len(&mut self, _len: usize) {}

        /// Zero out all bytes to prevent UTF-8 invalidation
        #[inline]
        fn pb_clear(&mut self) {
            self.0 = [0; N];
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: Converting to MaybeUninit is always safe
            unsafe {
                core::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut MaybeUninit<u8>, N)
            }
        }
    }

    impl<const N: usize> Deref for FixedLenString<N> {
        type Target = str;

        #[inline]
        fn deref(&self) -> &Self::Target {
            // SAFETY: Only safe way to set bytes is via pb_from_str, which is guaranteed to be
            // valid UTF-8
            unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
        }
    }

    impl<const N: usize> DerefMut for FixedLenString<N> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            // SAFETY: See above
            unsafe { core::str::from_utf8_unchecked_mut(self.0.as_mut_slice()) }
        }
    }

    impl<const N: usize> AsRef<str> for FixedLenString<N> {
        fn as_ref(&self) -> &str {
            self
        }
    }

    impl<const N: usize> AsMut<str> for FixedLenString<N> {
        fn as_mut(&mut self) -> &mut str {
            self
        }
    }

    impl<const N: usize> TryFrom<&str> for FixedLenString<N> {
        type Error = TryFromSliceError;

        /// Return error if `s.len() != N`
        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Ok(Self(s.as_bytes().try_into()?))
        }
    }

    impl<const N: usize> From<FixedLenString<N>> for [u8; N] {
        fn from(value: FixedLenString<N>) -> Self {
            value.0
        }
    }

    /// Allow byte array to be used for `bytes` fields with fixed length
    impl<const N: usize> PbBytes for [u8; N] {}
    impl<const N: usize> PbString for [u8; N] {
        #[inline]
        unsafe fn pb_set_len(&mut self, _len: usize) {}

        #[inline]
        fn pb_clear(&mut self) {}

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: Converting to MaybeUninit is always safe
            unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<u8>, N) }
        }
    }
}

#[cfg(feature = "container-arrayvec-0-7")]
mod impl_arrayvec {
    use core::ops::DerefMut;

    use super::*;

    use arrayvec::{ArrayString, ArrayVec};

    impl<const N: usize> PbBytes for ArrayVec<u8, N> {}
    impl<const N: usize> PbString for ArrayVec<u8, N> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<u8>, N)
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.set_len(len)
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.clear()
        }

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
    }

    impl<T, const N: usize> PbVec<T> for ArrayVec<T, N> {
        #[inline]
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.try_push(elem).map_err(drop)
        }
    }
}

#[allow(unused)]
macro_rules! impl_heapless {
    ($mod_name:ident, $pkg_name:ident) => {
        mod $mod_name {
            use super::*;

            use core::hash::{BuildHasher, Hash};

            use $pkg_name::{IndexMap, String, Vec};

            impl<const N: usize> PbBytes for Vec<u8, N> {}
            impl<const N: usize> PbString for Vec<u8, N> {
                #[inline]
                fn pb_clear(&mut self) {
                    self.clear()
                }

                #[inline]
                unsafe fn pb_set_len(&mut self, len: usize) {
                    self.set_len(len)
                }

                #[inline]
                fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
                    let len = self.len();
                    // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
                    // of N values
                    let slice = unsafe {
                        core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<u8>, N)
                    };
                    slice.get_mut(len..).unwrap_or(&mut [])
                }
            }

            impl<const N: usize> PbString for String<N> {
                #[inline]
                fn pb_clear(&mut self) {
                    self.clear()
                }

                #[inline]
                unsafe fn pb_set_len(&mut self, len: usize) {
                    self.as_mut_vec().set_len(len)
                }

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
            }

            impl<T, const N: usize> PbVec<T> for Vec<T, N> {
                #[inline]
                fn pb_push(&mut self, elem: T) -> Result<(), ()> {
                    self.push(elem).map_err(drop)
                }
            }

            impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> PbMap<K, V> for IndexMap<K, V, S, N> {
                #[inline]
                fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
                    self.insert(key, val).map_err(drop)?;
                    Ok(())
                }
            }
        }
    };
}

#[cfg(feature = "container-heapless-0-8")]
impl_heapless!(impl_heapless_0_8, heapless_0_8);
#[cfg(feature = "container-heapless-0-9")]
impl_heapless!(impl_heapless_0_9, heapless_0_9);

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;

    use alloc::{borrow::Cow, collections::BTreeMap, string::String, vec::Vec};

    impl PbBytes for Vec<u8> {}
    impl PbString for Vec<u8> {
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

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            self.spare_capacity_mut()
        }
    }

    impl PbBytes for Cow<'_, [u8]> {}
    impl PbString for Cow<'_, [u8]> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.to_mut().set_len(len);
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.to_mut().clear()
        }

        #[inline]
        fn pb_reserve(&mut self, additional: usize) {
            self.to_mut().reserve(additional)
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            self.to_mut().spare_capacity_mut()
        }
    }

    impl PbString for String {
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

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: spare_capacity_mut() is a safe call, since it doesn't change any bytes
            unsafe { self.as_mut_vec().spare_capacity_mut() }
        }
    }

    impl PbString for Cow<'_, str> {
        #[inline]
        unsafe fn pb_set_len(&mut self, len: usize) {
            self.to_mut().as_mut_vec().set_len(len);
        }

        #[inline]
        fn pb_clear(&mut self) {
            self.to_mut().clear()
        }

        #[inline]
        fn pb_reserve(&mut self, additional: usize) {
            self.to_mut().reserve(additional)
        }

        #[inline]
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // SAFETY: spare_capacity_mut() is a safe call, since it doesn't change any bytes
            unsafe { self.to_mut().as_mut_vec() }.spare_capacity_mut()
        }
    }

    impl<T> PbVec<T> for Vec<T> {
        #[inline]
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.push(elem);
            Ok(())
        }
    }

    impl<T> PbVec<T> for Cow<'_, [T]>
    where
        [T]: alloc::borrow::ToOwned<Owned = Vec<T>>,
    {
        fn pb_push(&mut self, elem: T) -> Result<(), ()> {
            self.to_mut().push(elem);
            Ok(())
        }
    }

    impl<K: Ord, V> PbMap<K, V> for BTreeMap<K, V> {
        #[inline]
        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    impl<K: Eq + core::hash::Hash, V> PbMap<K, V> for std::collections::HashMap<K, V> {
        #[inline]
        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }
    }
}
