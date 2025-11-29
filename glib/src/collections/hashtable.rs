// Take a look at the license at the top of the repository in the LICENSE file.

use std::{marker::PhantomData, ptr};

use crate::{ffi, translate::*, types::StaticType};

// rustdoc-stripper-ignore-next
/// A hash table of key-value pairs.
///
/// This is a safe wrapper around `GHashTable``.
#[repr(transparent)]
#[doc(alias = "GHashTable")]
pub struct HashTable<K: HashTableKey, V: HashTableValue> {
    ptr: ptr::NonNull<ffi::GHashTable>,
    phantom: PhantomData<(K, V)>,
}

impl<K: HashTableKey, V: HashTableValue> HashTable<K, V> {
    // rustdoc-stripper-ignore-next
    /// Creates a new empty hash table.
    #[doc(alias = "g_hash_table_new_full")]
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::g_hash_table_new_full(
                K::hash_func(),
                K::equal_func(),
                K::key_destroy_func(),
                V::value_destroy_func(),
            );
            HashTable {
                ptr: ptr::NonNull::new_unchecked(ptr),
                phantom: PhantomData,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Inserts a key-value pair into the hash table.
    ///
    /// If the key already exists, its value is replaced with the new value.
    #[doc(alias = "g_hash_table_insert")]
    pub fn insert(&mut self, key: K, value: V) {
        unsafe {
            let k = key.to_glib_key_full();
            let v = value.to_glib_value_full();
            let k_ptr = std::mem::transmute_copy(&k);
            let v_ptr = std::mem::transmute_copy(&v);
            ffi::g_hash_table_insert(self.ptr.as_ptr(), k_ptr, v_ptr);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Looks up a value in the hash table.
    ///
    /// Returns `None` if the key is not found.
    #[doc(alias = "g_hash_table_lookup")]
    pub fn get(&self, key: &K::Borrowed) -> Option<V> {
        unsafe {
            let borrowed = K::borrow_to_glib_ptr(key);
            let key_ptr = std::mem::transmute_copy(&borrowed.ptr());
            let value_ptr = ffi::g_hash_table_lookup(self.ptr.as_ptr(), key_ptr);
            V::from_glib_value_none(value_ptr)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the hash table contains the given key.
    #[doc(alias = "g_hash_table_contains")]
    pub fn contains_key(&self, key: &K::Borrowed) -> bool {
        unsafe {
            let borrowed = K::borrow_to_glib_ptr(key);
            let key_ptr = std::mem::transmute_copy(&borrowed.ptr());
            from_glib(ffi::g_hash_table_contains(self.ptr.as_ptr(), key_ptr))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Removes a key and its associated value from the hash table.
    ///
    /// Returns `true` if the key was found and removed.
    #[doc(alias = "g_hash_table_remove")]
    pub fn remove(&mut self, key: &K::Borrowed) -> bool {
        unsafe {
            let borrowed = K::borrow_to_glib_ptr(key);
            let key_ptr = std::mem::transmute_copy(&borrowed.ptr());
            from_glib(ffi::g_hash_table_remove(self.ptr.as_ptr(), key_ptr))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Inserts a new key-value pair, replacing any existing value.
    ///
    /// Returns `true` if the key did not already exist in the hash table.
    #[doc(alias = "g_hash_table_replace")]
    pub fn replace(&mut self, key: K, value: V) -> bool {
        unsafe {
            let k = key.to_glib_key_full();
            let v = value.to_glib_value_full();
            let k_ptr = std::mem::transmute_copy(&k);
            let v_ptr = std::mem::transmute_copy(&v);
            from_glib(ffi::g_hash_table_replace(self.ptr.as_ptr(), k_ptr, v_ptr))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the number of key-value pairs in the hash table.
    #[doc(alias = "g_hash_table_size")]
    pub fn len(&self) -> usize {
        unsafe { ffi::g_hash_table_size(self.ptr.as_ptr()) as usize }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the hash table contains no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // rustdoc-stripper-ignore-next
    /// Removes all key-value pairs from the hash table.
    /// they will be called for each removed entry.
    #[doc(alias = "g_hash_table_remove_all")]
    pub fn remove_all(&mut self) {
        unsafe {
            ffi::g_hash_table_remove_all(self.ptr.as_ptr());
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns an iterator over the hash table's key-value pairs.
    pub fn iter(&self) -> Iter<'_, K, V> {
        unsafe {
            let mut raw_iter = std::mem::zeroed();
            ffi::g_hash_table_iter_init(&mut raw_iter, self.ptr.as_ptr());
            Iter {
                raw_iter,
                phantom: PhantomData,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a vector containing all keys in the hash table.
    #[doc(alias = "g_hash_table_get_keys")]
    pub fn keys(&self) -> Vec<K> {
        unsafe {
            let list = ffi::g_hash_table_get_keys(self.ptr.as_ptr());
            let mut result = Vec::new();
            let mut current = list;
            while !current.is_null() {
                let key_ptr = (*current).data;
                let key = K::from_glib_key_none(key_ptr);
                result.push(key);
                current = (*current).next;
            }
            ffi::g_list_free(list);
            result
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a vector containing all values in the hash table.
    #[doc(alias = "g_hash_table_get_values")]
    pub fn values(&self) -> Vec<V> {
        unsafe {
            let list = ffi::g_hash_table_get_values(self.ptr.as_ptr());
            let mut result = Vec::new();
            let mut current = list;
            while !current.is_null() {
                let value_ptr = (*current).data;
                if let Some(value) = V::from_glib_value_none(value_ptr) {
                    result.push(value);
                }
                current = (*current).next;
            }
            ffi::g_list_free(list);
            result
        }
    }
}

impl<K: HashTableKey, V: HashTableValue> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: HashTableKey, V: HashTableValue> From<std::collections::HashMap<K, V>> for HashTable<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn from(map: std::collections::HashMap<K, V>) -> Self {
        let mut table = HashTable::new();
        for (k, v) in map {
            table.insert(k, v);
        }
        table
    }
}

impl<K: HashTableKey, V: HashTableValue> From<HashTable<K, V>> for std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn from(table: HashTable<K, V>) -> Self {
        table.iter().collect()
    }
}

impl<K: HashTableKey, V: HashTableValue> FromIterator<(K, V)> for HashTable<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut table = HashTable::new();
        for (k, v) in iter {
            table.insert(k, v);
        }
        table
    }
}

impl<'a, K: HashTableKey, V: HashTableValue> IntoIterator for &'a HashTable<K, V> {
    type Item = (K, V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K: HashTableKey, V: HashTableValue> Extend<(K, V)> for HashTable<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: HashTableKey, V: HashTableValue> Drop for HashTable<K, V> {
    fn drop(&mut self) {
        unsafe {
            ffi::g_hash_table_unref(self.ptr.as_ptr());
        }
    }
}

impl<K: HashTableKey, V: HashTableValue> StaticType for HashTable<K, V> {
    fn static_type() -> crate::Type {
        unsafe { from_glib(ffi::g_hash_table_get_type()) }
    }
}

#[doc(hidden)]
impl<K: HashTableKey, V: HashTableValue> GlibPtrDefault for HashTable<K, V> {
    type GlibType = *mut ffi::GHashTable;
}

#[doc(hidden)]
impl<'a, K: HashTableKey + 'a, V: HashTableValue + 'a> ToGlibPtr<'a, *mut ffi::GHashTable>
    for HashTable<K, V>
{
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GHashTable, Self> {
        Stash(self.ptr.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *mut ffi::GHashTable {
        unsafe {
            ffi::g_hash_table_ref(self.ptr.as_ptr());
            self.ptr.as_ptr()
        }
    }
}

#[doc(hidden)]
impl<K: HashTableKey, V: HashTableValue> FromGlibPtrNone<*mut ffi::GHashTable> for HashTable<K, V> {
    unsafe fn from_glib_none(ptr: *mut ffi::GHashTable) -> Self {
        assert!(!ptr.is_null());
        ffi::g_hash_table_ref(ptr);
        HashTable {
            ptr: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }
}

#[doc(hidden)]
impl<K: HashTableKey, V: HashTableValue> FromGlibPtrFull<*mut ffi::GHashTable> for HashTable<K, V> {
    unsafe fn from_glib_full(ptr: *mut ffi::GHashTable) -> Self {
        assert!(!ptr.is_null());
        HashTable {
            ptr: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }
}

#[doc(hidden)]
impl<K: HashTableKey, V: HashTableValue> FromGlibPtrBorrow<*mut ffi::GHashTable>
    for HashTable<K, V>
{
    unsafe fn from_glib_borrow(ptr: *mut ffi::GHashTable) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(HashTable {
            ptr: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        })
    }
}

// rustdoc-stripper-ignore-next
/// An iterator over the key-value pairs of a `HashTable`.
///
/// This struct is created by the [`iter`](HashTable::iter).
pub struct Iter<'a, K: HashTableKey, V: HashTableValue> {
    raw_iter: ffi::GHashTableIter,
    phantom: PhantomData<(&'a HashTable<K, V>, K, V)>,
}

impl<'a, K: HashTableKey, V: HashTableValue> Iterator for Iter<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut key_ptr: ffi::gpointer = ptr::null_mut();
            let mut value_ptr: ffi::gpointer = ptr::null_mut();

            if from_glib(ffi::g_hash_table_iter_next(
                &mut self.raw_iter,
                &mut key_ptr,
                &mut value_ptr,
            )) {
                // Convert the raw pointers to owned K and V values
                // We need to use from_glib_none because the hash table still owns these
                let key = K::from_glib_key_none(key_ptr);
                let value = V::from_glib_value_none(value_ptr)?;
                Some((key, value))
            } else {
                None
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Storage for borrowed key pointers that keeps temporaries alive.
pub struct BorrowStash<'a, P> {
    ptr: P,
    _storage: Box<dyn FnOnce() + 'a>,
}

impl<'a, P: Copy> BorrowStash<'a, P> {
    #[inline]
    pub fn new<S: 'a>(ptr: P, storage: S) -> Self {
        Self {
            ptr,
            _storage: Box::new(move || drop(storage)),
        }
    }

    #[inline]
    pub fn ptr(&self) -> P {
        self.ptr
    }
}

// rustdoc-stripper-ignore-next
/// Trait for types that can be used as hash table keys.
pub trait HashTableKey {
    // rustdoc-stripper-ignore-next
    /// The FFI pointer type for this key.
    type PtrType: Copy;

    // rustdoc-stripper-ignore-next
    /// The borrowed form of this key type used for lookups.
    type Borrowed: ?Sized;

    // rustdoc-stripper-ignore-next
    /// Returns the GLib hash function for this key type.
    fn hash_func() -> ffi::GHashFunc;

    // rustdoc-stripper-ignore-next
    /// Returns the GLib equality function for this key type.
    fn equal_func() -> ffi::GEqualFunc;

    // rustdoc-stripper-ignore-next
    /// Returns the destroy function for keys of this type.
    fn key_destroy_func() -> ffi::GDestroyNotify;

    // rustdoc-stripper-ignore-next
    /// Converts this key to a GLib pointer for insertion.
    fn to_glib_key_full(&self) -> Self::PtrType;

    // rustdoc-stripper-ignore-next
    /// Converts a borrowed key to a GLib pointer for lookup operations.
    fn borrow_to_glib_ptr(key: &Self::Borrowed) -> BorrowStash<'_, Self::PtrType>;

    // rustdoc-stripper-ignore-next
    /// Converts from a GLib pointer to this key type for iteration.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and the hash table must still own the key.
    unsafe fn from_glib_key_none(ptr: ffi::gpointer) -> Self
    where
        Self: Sized;
}

// rustdoc-stripper-ignore-next
/// Trait for types that can be used as hash table values.
pub trait HashTableValue {
    // rustdoc-stripper-ignore-next
    /// The FFI pointer type for this value.
    type PtrType: Copy;

    // rustdoc-stripper-ignore-next
    /// Returns the destroy function for values of this type.
    fn value_destroy_func() -> ffi::GDestroyNotify;

    // rustdoc-stripper-ignore-next
    /// Converts this value to a GLib pointer for insertion.
    fn to_glib_value_full(&self) -> Self::PtrType;

    // rustdoc-stripper-ignore-next
    /// Converts from a GLib pointer to this value type.
    unsafe fn from_glib_value_none(ptr: ffi::gpointer) -> Option<Self>
    where
        Self: Sized;
}

impl HashTableKey for String {
    type PtrType = *mut libc::c_char;
    type Borrowed = str;

    fn hash_func() -> ffi::GHashFunc {
        Some(ffi::g_str_hash)
    }

    fn equal_func() -> ffi::GEqualFunc {
        Some(ffi::g_str_equal)
    }

    fn key_destroy_func() -> ffi::GDestroyNotify {
        Some(ffi::g_free)
    }

    fn to_glib_key_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut libc::c_char>::to_glib_full(self)
    }

    fn borrow_to_glib_ptr(key: &Self::Borrowed) -> BorrowStash<'_, Self::PtrType> {
        // For &str, create a temporary C string via to_glib_none and keep the Stash alive
        let stash: Stash<'_, *const libc::c_char, str> = key.to_glib_none();
        let ptr = stash.0 as *mut libc::c_char;
        BorrowStash::new(ptr, stash)
    }

    unsafe fn from_glib_key_none(ptr: ffi::gpointer) -> Self {
        from_glib_none(ptr as *const libc::c_char)
    }
}

impl HashTableValue for String {
    type PtrType = *mut libc::c_char;

    fn value_destroy_func() -> ffi::GDestroyNotify {
        Some(ffi::g_free)
    }

    fn to_glib_value_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut libc::c_char>::to_glib_full(self)
    }

    unsafe fn from_glib_value_none(ptr: ffi::gpointer) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(from_glib_none(ptr as *const libc::c_char))
        }
    }
}

impl<T> HashTableKey for T
where
    T: crate::object::ObjectType + FromGlibPtrNone<*mut T::GlibType>,
{
    type PtrType = *mut T::GlibType;
    type Borrowed = T;

    fn hash_func() -> ffi::GHashFunc {
        Some(ffi::g_direct_hash)
    }

    fn equal_func() -> ffi::GEqualFunc {
        Some(ffi::g_direct_equal)
    }

    fn key_destroy_func() -> ffi::GDestroyNotify {
        unsafe extern "C" fn unref_object(ptr: ffi::gpointer) {
            crate::gobject_ffi::g_object_unref(ptr as *mut crate::gobject_ffi::GObject);
        }
        Some(unref_object)
    }

    fn to_glib_key_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut T::GlibType>::to_glib_full(self)
    }

    fn borrow_to_glib_ptr(key: &Self::Borrowed) -> BorrowStash<'_, Self::PtrType> {
        let stash = key.to_glib_none();
        BorrowStash::new(stash.0, stash)
    }

    unsafe fn from_glib_key_none(ptr: ffi::gpointer) -> Self {
        from_glib_none(ptr as *mut T::GlibType)
    }
}

impl<T> HashTableValue for T
where
    T: crate::object::ObjectType + FromGlibPtrNone<*mut T::GlibType>,
{
    type PtrType = *mut T::GlibType;

    fn value_destroy_func() -> ffi::GDestroyNotify {
        unsafe extern "C" fn unref_object(ptr: ffi::gpointer) {
            crate::gobject_ffi::g_object_unref(ptr as *mut crate::gobject_ffi::GObject);
        }
        Some(unref_object)
    }

    fn to_glib_value_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut T::GlibType>::to_glib_full(self)
    }

    unsafe fn from_glib_value_none(ptr: ffi::gpointer) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(from_glib_none(ptr as *mut T::GlibType))
        }
    }
}

// Implement HashTableKey for Variant
impl HashTableKey for crate::Variant {
    type PtrType = *mut ffi::GVariant;
    type Borrowed = crate::Variant;

    fn hash_func() -> ffi::GHashFunc {
        Some(ffi::g_variant_hash)
    }

    fn equal_func() -> ffi::GEqualFunc {
        Some(ffi::g_variant_equal)
    }

    fn key_destroy_func() -> ffi::GDestroyNotify {
        unsafe extern "C" fn unref_variant(ptr: ffi::gpointer) {
            ffi::g_variant_unref(ptr as *mut ffi::GVariant);
        }
        Some(unref_variant)
    }

    fn to_glib_key_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut ffi::GVariant>::to_glib_full(self)
    }

    fn borrow_to_glib_ptr(key: &Self::Borrowed) -> BorrowStash<'_, Self::PtrType> {
        let stash = key.to_glib_none();
        BorrowStash::new(stash.0, stash)
    }

    unsafe fn from_glib_key_none(ptr: ffi::gpointer) -> Self {
        from_glib_none(ptr as *mut ffi::GVariant)
    }
}

// Implement HashTableValue for Variant
impl HashTableValue for crate::Variant {
    type PtrType = *mut ffi::GVariant;

    fn value_destroy_func() -> ffi::GDestroyNotify {
        unsafe extern "C" fn unref_variant(ptr: ffi::gpointer) {
            ffi::g_variant_unref(ptr as *mut ffi::GVariant);
        }
        Some(unref_variant)
    }

    fn to_glib_value_full(&self) -> Self::PtrType {
        ToGlibPtr::<*mut ffi::GVariant>::to_glib_full(self)
    }

    unsafe fn from_glib_value_none(ptr: ffi::gpointer) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(from_glib_none(ptr as *mut ffi::GVariant))
        }
    }
}

macro_rules! impl_primitive_hashtable {
    ($($t:ty),+ $(,)?) => {
        $(
            impl HashTableKey for $t {
                type PtrType = ffi::gpointer;
                type Borrowed = $t;

                fn hash_func() -> ffi::GHashFunc {
                    Some(ffi::g_direct_hash)
                }

                fn equal_func() -> ffi::GEqualFunc {
                    Some(ffi::g_direct_equal)
                }

                fn key_destroy_func() -> ffi::GDestroyNotify {
                    None
                }

                fn to_glib_key_full(&self) -> Self::PtrType {
                    *self as ffi::gpointer
                }

                fn borrow_to_glib_ptr(key: &Self::Borrowed) -> BorrowStash<'_, Self::PtrType> {
                    BorrowStash::new(*key as ffi::gpointer, ())
                }

                unsafe fn from_glib_key_none(ptr: ffi::gpointer) -> Self {
                    ptr as $t
                }
            }

            impl HashTableValue for $t {
                type PtrType = ffi::gpointer;

                fn value_destroy_func() -> ffi::GDestroyNotify {
                    None
                }

                fn to_glib_value_full(&self) -> Self::PtrType {
                    *self as ffi::gpointer
                }

                unsafe fn from_glib_value_none(ptr: ffi::gpointer) -> Option<Self> {
                    if ptr.is_null() {
                        None
                    } else {
                        Some(ptr as $t)
                    }
                }
            }
        )+
    };
}

impl_primitive_hashtable! {
    i8, u8, u16, i16, i32, u32, i64, u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variant::ToVariant;

    #[test]
    fn test_all_features() {
        let mut table = HashTable::<String, String>::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        assert_eq!(table.get("nonexistent"), None);
        assert!(!table.contains_key("nonexistent"));

        let default_table = HashTable::<String, String>::default();
        assert!(default_table.is_empty());

        table.insert("key1".to_string(), "value1".to_string());
        assert!(!table.is_empty());
        assert_eq!(table.len(), 1);
        assert_eq!(table.get("key1"), Some("value1".to_string()));
        assert!(table.contains_key("key1"));

        table.insert("key2".to_string(), "value2".to_string());
        table.insert("key3".to_string(), "value3".to_string());
        assert_eq!(table.len(), 3);
        assert_eq!(table.get("key2"), Some("value2".to_string()));
        assert_eq!(table.get("key3"), Some("value3".to_string()));
        assert!(table.contains_key("key2"));
        assert!(table.contains_key("key3"));

        let was_new = table.replace("key1".to_string(), "replaced_value1".to_string());
        assert!(!was_new); // Key already existed
        assert_eq!(table.len(), 3);
        assert_eq!(table.get("key1"), Some("replaced_value1".to_string()));

        let was_new = table.replace("key4".to_string(), "value4".to_string());
        assert!(was_new); // New key
        assert_eq!(table.len(), 4);

        // Test remove
        assert!(table.remove("key2"));
        assert_eq!(table.len(), 3);
        assert_eq!(table.get("key2"), None);
        assert!(!table.contains_key("key2"));
        assert!(!table.remove("key2")); // Already removed

        table.remove_all();
        table.insert("a".to_string(), "x".to_string());
        table.insert("b".to_string(), "y".to_string());
        table.insert("c".to_string(), "z".to_string());

        let mut collected = std::collections::HashMap::new();
        for (k, v) in table.iter() {
            collected.insert(k, v);
        }
        assert_eq!(collected.len(), 3);
        assert_eq!(collected.get("a"), Some(&"x".to_string()));
        assert_eq!(collected.get("b"), Some(&"y".to_string()));
        assert_eq!(collected.get("c"), Some(&"z".to_string()));

        // Test keys() and values()
        let keys = table.keys();
        let values = table.values();
        assert_eq!(keys.len(), 3);
        assert_eq!(values.len(), 3);
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"b".to_string()));
        assert!(keys.contains(&"c".to_string()));
        assert!(values.contains(&"x".to_string()));
        assert!(values.contains(&"y".to_string()));
        assert!(values.contains(&"z".to_string()));

        table.remove_all();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        assert!(!table.contains_key("key1"));

        assert_eq!(table.iter().count(), 0);

        let empty_keys = table.keys();
        let empty_values = table.values();
        assert_eq!(empty_keys.len(), 0);
        assert_eq!(empty_values.len(), 0);

        // Test HashMap conversions
        let mut map = std::collections::HashMap::new();
        map.insert("a".to_string(), "x".to_string());
        map.insert("b".to_string(), "y".to_string());
        map.insert("c".to_string(), "z".to_string());

        let table2 = HashTable::from(map);
        assert_eq!(table2.len(), 3);
        assert_eq!(table2.get("a"), Some("x".to_string()));
        assert_eq!(table2.get("b"), Some("y".to_string()));
        assert_eq!(table2.get("c"), Some("z".to_string()));

        let map2: std::collections::HashMap<String, String> = table2.into();
        assert_eq!(map2.len(), 3);
        assert_eq!(map2.get("a"), Some(&"x".to_string()));

        // Test FromIterator
        let data = vec![
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ];
        let table3: HashTable<String, String> = data.into_iter().collect();
        assert_eq!(table3.len(), 2);
        assert_eq!(table3.get("k1"), Some("v1".to_string()));

        // Test IntoIterator for &HashTable
        let table4 = HashTable::from(std::collections::HashMap::from([
            ("x".to_string(), "1".to_string()),
            ("y".to_string(), "2".to_string()),
        ]));
        let mut count = 0;
        for (k, v) in &table4 {
            assert!(k == "x".to_string() || k == "y".to_string());
            assert!(v == "1".to_string() || v == "2".to_string());
            count += 1;
        }
        assert_eq!(count, 2);

        // Test Extend
        let mut table5 = HashTable::<String, String>::new();
        table5.insert("a".to_string(), "1".to_string());
        table5.extend(vec![
            ("b".to_string(), "2".to_string()),
            ("c".to_string(), "3".to_string()),
        ]);
        assert_eq!(table5.len(), 3);
        assert_eq!(table5.get("b"), Some("2".to_string()));

        // Test FFI conversion traits
        let mut original = HashTable::<String, String>::new();
        original.insert("ffi_key".to_string(), "ffi_value".to_string());

        let ptr = original.to_glib_none();
        assert!(!ptr.0.is_null());

        let full_ptr = original.to_glib_full();
        assert!(!full_ptr.is_null());
        let from_full: HashTable<String, String> = unsafe { from_glib_full(full_ptr) };
        assert_eq!(from_full.len(), 1);
        assert_eq!(from_full.get("ffi_key"), Some("ffi_value".to_string()));

        let none_ptr = original.to_glib_none().0;
        let from_none: HashTable<String, String> = unsafe { from_glib_none(none_ptr) };
        assert_eq!(from_none.len(), 1);
        assert_eq!(from_none.get("ffi_key"), Some("ffi_value".to_string()));

        let mut table = HashTable::<String, String>::new();
        table.insert("A".into(), "1".into());
        table.insert("B".into(), "2".into());
        table.insert("C".into(), "3".into());
        let ptr: *mut ffi::GHashTable = table.to_glib_full();
        let table_back: HashTable<String, String> = unsafe { from_glib_full(ptr) };
        assert_eq!(table_back.get("A"), Some("1".into()));
        assert_eq!(table_back.get("B"), Some("2".into()));
        assert_eq!(table_back.get("C"), Some("3".into()));
    }

    #[test]
    fn test_object_type() {
        use crate::Object;

        let mut table = HashTable::<Object, Object>::new();
        assert!(table.is_empty());

        let obj1 = Object::new::<Object>();
        let obj2 = Object::new::<Object>();
        let obj3 = Object::new::<Object>();

        // Test insert and get
        table.insert(obj1.clone(), obj2.clone());
        assert_eq!(table.len(), 1);
        assert!(table.contains_key(&obj1));
        assert_eq!(table.get(&obj1).unwrap(), obj2);

        // Test multiple inserts
        table.insert(obj2.clone(), obj3.clone());
        assert_eq!(table.len(), 2);
        assert_eq!(table.get(&obj2).unwrap(), obj3);

        // Test that objects are compared by pointer identity
        let obj1_clone = obj1.clone();
        assert_eq!(table.get(&obj1_clone).unwrap(), obj2);

        // Test remove
        assert!(table.remove(&obj1));
        assert_eq!(table.len(), 1);
        assert!(!table.contains_key(&obj1));
        assert!(!table.remove(&obj1)); // Already removed
    }

    #[test]
    fn test_primitive_type() {
        // Test HashTable<i32, i32>
        let mut table = HashTable::<i32, i32>::new();
        assert!(table.is_empty());

        table.insert(1, 100);
        table.insert(2, 200);
        table.insert(3, 300);
        assert_eq!(table.len(), 3);
        assert_eq!(table.get(&1), Some(100));
        assert_eq!(table.get(&2), Some(200));
        assert!(table.contains_key(&1));
        assert!(!table.contains_key(&999));

        // Test remove
        assert!(table.remove(&2));
        assert_eq!(table.len(), 2);
        assert!(!table.contains_key(&2));
    }

    #[test]
    fn test_mixed_types() {
        use crate::Object;

        // Test HashTable<i32, String>
        let mut table1 = HashTable::<i32, String>::new();
        table1.insert(1, "one".to_string());
        assert_eq!(table1.get(&1), Some("one".to_string()));

        // Test HashTable<String, i32>
        let mut table2 = HashTable::<String, i32>::new();
        table2.insert("one".to_string(), 1);
        assert_eq!(table2.get("one"), Some(1));

        // Test HashTable<i32, Object>
        let obj1 = Object::new::<Object>();
        let mut table3 = HashTable::<i32, Object>::new();
        table3.insert(1, obj1.clone());
        assert_eq!(table3.get(&1).unwrap(), obj1);

        // Test HashTable<Object, i32>
        let key1 = Object::new::<Object>();
        let mut table4 = HashTable::<Object, i32>::new();
        table4.insert(key1.clone(), 100);
        assert_eq!(table4.get(&key1), Some(100));

        // Test HashTable<String, Object>
        let obj2 = Object::new::<Object>();
        let mut table5 = HashTable::<String, Object>::new();
        table5.insert("key1".to_string(), obj2.clone());
        assert_eq!(table5.get("key1").unwrap(), obj2);

        // Test HashTable<Object, String>
        let key2 = Object::new::<Object>();
        let mut table6 = HashTable::<Object, String>::new();
        table6.insert(key2.clone(), "value".to_string());
        assert_eq!(table6.get(&key2), Some("value".to_string()));
        // Test HashTable<String, Variant>
        let mut table7 = HashTable::<String, crate::Variant>::new();
        let variant = 42i32.to_variant();
        table7.insert("key".to_string(), variant.clone());
        assert_eq!(table7.get("key").unwrap(), variant);
    }
}
