// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::Cow;
use std::default::Default;

use crate::translate::*;
use crate::variant::*;
use crate::variant_type::*;

glib_wrapper! {
    /// `VariantDict` is a mutable key/value store where the keys are always
    /// strings and the values are [`Variant`s](variant/struct.Variant.html).
    ///
    /// Variant dictionaries can easily be converted to/from `Variant`s of the
    /// appropriate type.  In `glib` terms, this is a variant of the form `"a{sv}"`.
    ///
    /// # Panics
    ///
    /// Note, pretty much all methods on this struct will panic if the
    /// [`end_unsafe()`](#method.end_unsafe) method was called on the instance.
    pub struct VariantDict(Shared<ffi::GVariantDict>);

    match fn {
        ref => |ptr| ffi::g_variant_dict_ref(ptr),
        unref => |ptr| ffi::g_variant_dict_unref(ptr),
        get_type => || ffi::g_variant_dict_get_type(),
    }
}

impl VariantDict {
    /// Create a new `VariantDict` optionally populating it with the given `Variant`
    ///
    /// Since `Variant`s are immutable, this does not couple the `VariantDict` with
    /// the input `Variant`, instead the contents are copied into the `VariantDict`.
    ///
    /// # Panics
    ///
    /// This function will panic if the given `Variant` is not of the correct type.
    pub fn new(from_asv: Option<&Variant>) -> Self {
        if let Some(var) = from_asv {
            assert_eq!(var.type_(), VariantDict::static_variant_type());
        }
        unsafe { from_glib_full(ffi::g_variant_dict_new(from_asv.to_glib_none().0)) }
    }

    /// Check if this `VariantDict` contains the given key.
    ///
    /// Look up whether or not the given key is present, returning `true` if it
    /// is present in `self`.
    pub fn contains(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_variant_dict_contains(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    /// Look up and return a value from this `VariantDict`.
    ///
    /// The given `key` is looked up in `self`.  If `expected_type` is not
    /// `None` then it will be matched against the type of any found value.
    ///
    /// This will return `None` if the `key` is not present in the dictionary
    /// or if it is present but the type of the value does not match a given
    /// `expected_type`.  Otherwise `Some(value)` will be returned where
    /// the `value` is an instance of [`Variant`](variant/struct.Variant.html).
    pub fn lookup_value(&self, key: &str, expected_type: Option<&VariantTy>) -> Option<Variant> {
        unsafe {
            from_glib_none(ffi::g_variant_dict_lookup_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
                expected_type.to_glib_none().0,
            ))
        }
    }

    /// Insert a variant into the dictionary.
    ///
    /// The given `key`/`value` pair is inserted into `self`.  If a value
    /// was previously associated with `key` then it is overwritten.
    ///
    /// For convenience, you may use the [`insert()`](#method.insert) if
    /// you have a value which implements [`ToVariant`](variant/trait.ToVariant.html).
    pub fn insert_value(&self, key: &str, value: &Variant) {
        unsafe {
            ffi::g_variant_dict_insert_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            )
        }
    }

    /// Insert a value into the dictionary
    ///
    /// The given `key`/`value` pair is inserted into `self`.  If a value
    /// was previously associated with `key` then it is overwritten.
    ///
    /// This is a convenience method which automatically calls
    /// [`to_variant()`](variant/trait.ToVariant.html#method.to_variant) for you
    /// on the given value.
    ///
    /// If, on the other hand, you have a [`Variant`](variant/struct.Variant.html)
    /// instance already, you should use the [`insert_value()`](#method.insert_value)
    /// method instead.
    pub fn insert<T: ToVariant>(&self, key: &str, value: &T) {
        unsafe {
            ffi::g_variant_dict_insert_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_variant().to_glib_none().0,
            )
        }
    }

    /// Remove the given `key` from the dictionary.
    ///
    /// This removes the given `key` from the dictionary, releasing the reference
    /// on the associated value if one is present.
    ///
    /// If a `key`/`value` pair was removed from the dictionary, `true` is
    /// returned.  If `key` was not present then `false` is returned instead.
    pub fn remove(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_variant_dict_remove(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    /// Convert this dictionary to a [`Variant`](variant/struct.Variant.html)
    ///
    /// This method converts `self` into an instance of [`Variant`](variant/struct.Variant.html)
    /// but in doing so renders it very unsafe to use.
    ///
    /// # Safety
    ///
    /// After calling this, the underlying `GVariantDict` is in a state where
    /// the only valid operations to perform as reference ones.  As such
    /// any attempt to read/update the dictionary *will* fail and emit warnings
    /// of such.
    ///
    /// You should only use this function if the extra cost of the safe function
    /// is too much for your performance critical codepaths
    pub unsafe fn end_unsafe(&self) -> Variant {
        from_glib_none(ffi::g_variant_dict_end(self.to_glib_none().0))
    }

    /// Convert this dictionary to a [`Variant`](variant/struct.Variant.html)
    ///
    /// This method converts `self` into an instance of [`Variant`](variant/struct.Variant.html)
    /// and then reinitialises itself in order to be safe for further use.
    ///
    /// If you are certain that nothing other than disposing of references will
    /// be done subsequent to ending the instance, you can call the
    /// [`end_unsafe()`](#method.end_unsafe) method instead to avoid the unnecessary
    /// reinitialisation of the dictionary.
    pub fn end(&self) -> Variant {
        unsafe {
            let ret = self.end_unsafe();
            // Reinitialise the dict so that we can continue safely
            ffi::g_variant_dict_init(self.to_glib_none().0, None::<Variant>.to_glib_none().0);
            ret
        }
    }
}

impl Default for VariantDict {
    fn default() -> Self {
        Self::new(None)
    }
}

impl StaticVariantType for VariantDict {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("a{sv}").into() }
    }
}

impl ToVariant for VariantDict {
    fn to_variant(&self) -> Variant {
        self.end()
    }
}

impl FromVariant for VariantDict {
    fn from_variant(variant: &Variant) -> Option<Self> {
        if variant.type_() == VariantDict::static_variant_type() {
            Some(Self::new(Some(variant)))
        } else {
            None
        }
    }
}

impl From<Variant> for VariantDict {
    fn from(other: Variant) -> Self {
        Self::new(Some(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_destroy() {
        let _dict = VariantDict::new(None);
    }

    #[test]
    fn create_roundtrip() {
        let dict = VariantDict::default();
        let var: Variant = dict.to_variant();
        let _dict2: VariantDict = var.into();
    }

    #[test]
    fn create_populate_destroy() {
        let dict = VariantDict::default();
        dict.insert_value("one", &(1u8.to_variant()));
        assert_eq!(dict.lookup_value("one", None), Some(1u8.to_variant()));
    }

    #[test]
    fn create_populate_roundtrip() {
        let dict = VariantDict::default();
        dict.insert_value("one", &(1u8.to_variant()));
        let var: Variant = dict.to_variant();
        let dict = VariantDict::from_variant(&var).expect("Not a dict?");
        assert_eq!(dict.lookup_value("one", None), Some(1u8.to_variant()));
    }

    #[test]
    fn create_populate_remove() {
        let dict = VariantDict::default();
        let empty_var = dict.to_variant();
        dict.insert("one", &1u64);
        assert!(dict.remove("one"));
        assert!(!dict.remove("one"));
        let var2 = dict.to_variant();
        assert_eq!(empty_var, var2);
    }
}
