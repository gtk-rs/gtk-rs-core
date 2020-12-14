// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Keymap;
use crate::KeymapKey;
use crate::ModifierType;
use glib::translate::*;
use std::mem;
use std::ptr;

impl Keymap {
    pub fn get_entries_for_keycode(&self, hardware_keycode: u32) -> Vec<(KeymapKey, u32)> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_get_entries_for_keycode(
                self.to_glib_none().0,
                hardware_keycode,
                &mut keys,
                &mut keyvals,
                n_entries.as_mut_ptr(),
            ));
            if ret {
                let n_entries = n_entries.assume_init() as usize;
                let mut entries = Vec::with_capacity(n_entries);
                for i in 0..n_entries {
                    entries.push((from_glib_none(keys.add(i)), ptr::read(keyvals.add(i))));
                }
                glib::ffi::g_free(keys as *mut _);
                glib::ffi::g_free(keyvals as *mut _);
                entries
            } else {
                Vec::new()
            }
        }
    }

    pub fn get_entries_for_keyval(&self, keyval: u32) -> Vec<KeymapKey> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_get_entries_for_keyval(
                self.to_glib_none().0,
                keyval,
                &mut keys,
                n_keys.as_mut_ptr(),
            ));
            if ret {
                let n_keys = n_keys.assume_init() as usize;
                let mut r_keys = Vec::with_capacity(n_keys);
                for i in 0..n_keys {
                    r_keys.push(from_glib_none(keys.add(i)));
                }
                glib::ffi::g_free(keys as *mut _);
                r_keys
            } else {
                Vec::new()
            }
        }
    }

    pub fn add_virtual_modifiers(&self, state: &mut ModifierType) {
        unsafe {
            let mut s = state.to_glib();
            ffi::gdk_keymap_add_virtual_modifiers(self.to_glib_none().0, &mut s);
            *state = from_glib(s);
        }
    }

    pub fn map_virtual_modifiers(&self, state: &mut ModifierType) -> bool {
        unsafe {
            let mut s = state.to_glib();
            let ret = from_glib(ffi::gdk_keymap_map_virtual_modifiers(
                self.to_glib_none().0,
                &mut s,
            ));
            *state = from_glib(s);
            ret
        }
    }
}
