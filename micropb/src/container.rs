#![allow(clippy::result_unit_err)]

use core::{mem::MaybeUninit, ops::DerefMut};

pub trait PbContainer: Default {
    /// Sets length of string
    ///
    /// # Safety
    /// New length must be smaller than the capacity, and the elements between the old
    /// and new lengths must be initialized and valid.
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

pub trait PbMap<K: 'static, V: 'static>: Default {
    type Iter<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        Self: 'a;

    fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()>;

    fn pb_iter(&self) -> Self::Iter<'_>;

    fn pb_get(&self, key: &K) -> Option<&V>;

    fn pb_get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn pb_remove(&mut self, key: &K) -> Option<V>;

    fn pb_clear(&mut self);
}

#[cfg(feature = "container-arrayvec")]
mod impl_arrayvec {
    use super::*;

    use arrayvec::{ArrayString, ArrayVec};

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
            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            &mut slice[len..]
        }
    }

    impl<const N: usize> PbString for ArrayString<N> {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            let len = self.len();
            // Works in Miri with tree borrows, but not stack borrows due to provenance issues with
            // `deref_mut`
            self.clear();
            let s = self.deref_mut().as_mut_ptr();
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe { core::slice::from_raw_parts_mut(s as *mut MaybeUninit<u8>, N) };
            &mut slice[len..]
        }
    }

    #[cfg(feature = "encode")]
    impl<const N: usize> crate::encode::PbWrite for ArrayVec<u8, N> {
        type Error = arrayvec::CapacityError;

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.try_extend_from_slice(data)
        }
    }
}

#[cfg(feature = "container-heapless")]
mod impl_heapless {
    use super::*;

    use core::hash::{BuildHasher, Hash};

    use heapless::{IndexMap, IndexMapIter, String, Vec};

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
            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut MaybeUninit<T>, N)
            };
            &mut slice[len..]
        }
    }

    impl<const N: usize> PbString for String<N> {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            let len = self.len();
            // SAFETY: Underlying storage is array of N bytes, so the slice is valid
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.as_mut_vec().as_mut_ptr() as *mut MaybeUninit<u8>,
                    N,
                )
            };
            &mut slice[len..]
        }
    }

    impl<K: 'static + Eq + Hash, V: 'static, S: Default + BuildHasher, const N: usize> PbMap<K, V>
        for IndexMap<K, V, S, N>
    {
        type Iter<'a> = IndexMapIter<'a, K, V> where S: 'a;

        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val).map_err(drop)?;
            Ok(())
        }

        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }

        fn pb_get(&self, key: &K) -> Option<&V> {
            self.get(key)
        }

        fn pb_get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.get_mut(key)
        }

        fn pb_remove(&mut self, key: &K) -> Option<V> {
            self.remove(key)
        }

        fn pb_clear(&mut self) {
            self.clear();
        }
    }

    #[cfg(feature = "encode")]
    impl<const N: usize> crate::encode::PbWrite for Vec<u8, N> {
        type Error = ();

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.extend_from_slice(data)
        }
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;

    use alloc::{
        collections::{btree_map, BTreeMap},
        string::String,
        vec::Vec,
    };

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

    impl<K: 'static + Ord, V: 'static> PbMap<K, V> for BTreeMap<K, V> {
        type Iter<'a> = btree_map::Iter<'a, K, V>;

        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }

        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }

        fn pb_get(&self, key: &K) -> Option<&V> {
            self.get(key)
        }

        fn pb_get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.get_mut(key)
        }

        fn pb_remove(&mut self, key: &K) -> Option<V> {
            self.remove(key)
        }

        fn pb_clear(&mut self) {
            self.clear();
        }
    }

    #[cfg(feature = "std")]
    impl<K: 'static + Eq + core::hash::Hash, V: 'static> PbMap<K, V>
        for std::collections::HashMap<K, V>
    {
        type Iter<'a> = std::collections::hash_map::Iter<'a, K, V>;

        fn pb_insert(&mut self, key: K, val: V) -> Result<(), ()> {
            self.insert(key, val);
            Ok(())
        }

        fn pb_iter(&self) -> Self::Iter<'_> {
            self.iter()
        }

        fn pb_get(&self, key: &K) -> Option<&V> {
            self.get(key)
        }

        fn pb_get_mut(&mut self, key: &K) -> Option<&mut V> {
            self.get_mut(key)
        }

        fn pb_remove(&mut self, key: &K) -> Option<V> {
            self.remove(key)
        }

        fn pb_clear(&mut self) {
            self.clear();
        }
    }

    #[cfg(feature = "encode")]
    impl crate::encode::PbWrite for Vec<u8> {
        type Error = never::Never;

        fn pb_write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            self.extend_from_slice(data);
            Ok(())
        }
    }
}
