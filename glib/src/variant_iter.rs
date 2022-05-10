// Take a look at the license at the top of the repository in the LICENSE file.

// This is similar to the GVariantIter provided by glib, but that would
// introduce a heap allocation and doesn't provide a way to determine how
// many items are left in the iterator.

use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};

use crate::translate::*;
use crate::variant::Variant;

// rustdoc-stripper-ignore-next
/// Iterator over items in a variant.
#[derive(Debug)]
pub struct VariantIter {
    variant: Variant,
    head: usize,
    tail: usize,
}

impl VariantIter {
    pub(crate) fn new(variant: Variant) -> Self {
        let tail = variant.n_children();
        Self {
            variant,
            head: 0,
            tail,
        }
    }
}

impl Iterator for VariantIter {
    type Item = Variant;

    fn next(&mut self) -> Option<Variant> {
        if self.head == self.tail {
            None
        } else {
            let value = self.variant.child_value(self.head);
            self.head += 1;
            Some(value)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.tail - self.head;
        (size, Some(size))
    }
}

impl DoubleEndedIterator for VariantIter {
    fn next_back(&mut self) -> Option<Variant> {
        if self.head == self.tail {
            None
        } else {
            self.tail -= 1;
            Some(self.variant.child_value(self.tail))
        }
    }
}

impl ExactSizeIterator for VariantIter {}

impl FusedIterator for VariantIter {}

// rustdoc-stripper-ignore-next
/// Iterator over items in a variant of type `as`.
#[derive(Debug)]
pub struct VariantStrIter<'a> {
    variant: &'a Variant,
    head: usize,
    tail: usize,
}

impl<'a> VariantStrIter<'a> {
    pub(crate) fn new(variant: &'a Variant) -> Self {
        let tail = variant.n_children();
        Self {
            variant,
            head: 0,
            tail,
        }
    }

    fn impl_get(&self, i: usize) -> &'a str {
        unsafe {
            let p: *mut libc::c_char = std::ptr::null_mut();
            let s = b"&s\0";
            ffi::g_variant_get_child(
                self.variant.to_glib_none().0,
                i,
                s as *const u8 as *const _,
                &p,
                std::ptr::null::<i8>(),
            );
            let p = std::ffi::CStr::from_ptr(p);
            p.to_str().unwrap()
        }
    }
}

impl<'a> Iterator for VariantStrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.head == self.tail {
            None
        } else {
            let v = self.impl_get(self.head);
            self.head += 1;
            Some(v)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.tail - self.head;
        (size, Some(size))
    }
}

impl<'a> DoubleEndedIterator for VariantStrIter<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        if self.head == self.tail {
            None
        } else {
            self.tail -= 1;
            Some(self.impl_get(self.tail))
        }
    }
}

impl<'a> ExactSizeIterator for VariantStrIter<'a> {}

impl<'a> FusedIterator for VariantStrIter<'a> {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::variant::{DictEntry, Variant};
    use std::collections::HashMap;

    #[test]
    fn test_variant_iter_variant() {
        let v = Variant::from_variant(&"foo".to_string().to_variant());
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string()]);
    }

    #[test]
    fn test_variant_iter_array() {
        let v = Variant::array_from_iter::<String, _>([
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        let a = vec!["foo".to_string(), "bar".to_string()];
        assert_eq!(&vec, &a);
        let vec: Vec<_> = v.array_iter_str().unwrap().collect();
        assert_eq!(&vec, &a);
    }

    #[test]
    fn test_variant_iter_tuple() {
        let v = Variant::tuple_from_iter([
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_variant_iter_dictentry() {
        let v = DictEntry::new("foo", 1337).to_variant();
        println!("{:?}", v.iter().collect::<Vec<_>>());
        assert_eq!(v.iter().count(), 2);
    }

    #[test]
    fn test_variant_iter_map() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 1);
        let v = map.to_variant();
        assert_eq!(v.iter().count(), 2);
    }
}
