// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::Icon;

pub trait IconImpl: ObjectImpl + Hash + PartialEq {
    fn serialize(&self) -> Option<glib::Variant> {
        self.parent_serialize()
    }

    fn to_tokens(&self) -> Option<(Vec<String>, i32)> {
        self.parent_to_tokens()
    }

    fn from_tokens(tokens: &[String], version: i32) -> Result<Icon, Error> {
        Self::parent_from_tokens(tokens, version)
    }
}

pub trait IconImplExt: ObjectSubclass {
    fn parent_serialize(&self) -> Option<glib::Variant>;
    fn parent_to_tokens(&self) -> Option<(Vec<String>, i32)>;
    fn parent_from_tokens(tokens: &[String], version: i32) -> Result<Icon, Error>;
}

impl<T: IconImpl> IconImplExt for T {
    fn parent_serialize(&self) -> Option<glib::Variant> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Icon>() as *const ffi::GIconIface;

            let func = (*parent_iface)
                .serialize
                .expect("No parent iface implementation for \"serialize\"");
            from_glib_full(func(self.obj().unsafe_cast_ref::<Icon>().to_glib_none().0))
        }
    }

    fn parent_to_tokens(&self) -> Option<(Vec<String>, i32)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Icon>() as *const ffi::GIconIface;

            let tokens = std::ptr::null_mut();
            let mut version = std::mem::MaybeUninit::uninit();
            let func = (*parent_iface)
                .to_tokens
                .expect("No parent iface implementation for \"to_tokens\"");
            let result = from_glib(func(
                self.obj().unsafe_cast_ref::<Icon>().to_glib_none().0,
                tokens,
                version.as_mut_ptr(),
            ));
            if result {
                Some((
                    FromGlibPtrArrayContainerAsVec::from_glib_full_as_vec(tokens),
                    version.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn parent_from_tokens(tokens: &[String], version: i32) -> Result<Icon, Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Icon>() as *const ffi::GIconIface;

            let func = (*parent_iface)
                .from_tokens
                .expect("No parent iface implementation for \"from_tokens\"");
            let mut err = std::ptr::null_mut();

            let icon = func(
                tokens.as_ptr() as *mut _,
                tokens.len() as _,
                version,
                &mut err,
            );
            if err.is_null() {
                Ok(Icon::from_glib_full(icon))
            } else {
                Err(from_glib_full(err))
            }
        }
    }
}

unsafe impl<T: IconImpl> IsImplementable<T> for Icon {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.to_tokens = Some(icon_to_tokens::<T>);
        iface.from_tokens = Some(icon_from_tokens::<T>);
        iface.serialize = Some(icon_serialize::<T>);
        iface.equal = Some(icon_equal::<T>);
        iface.hash = Some(icon_hash::<T>);
    }
}

unsafe extern "C" fn icon_hash<T: IconImpl>(icon: *mut ffi::GIcon) -> u32 {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.imp();
    let mut hasher = DefaultHasher::new();
    imp.hash(&mut hasher);
    hasher.finish() as _
}

unsafe extern "C" fn icon_equal<T: IconImpl>(
    icon1: *mut ffi::GIcon,
    icon2: *mut ffi::GIcon,
) -> glib::ffi::gboolean {
    let instance = &*(icon1 as *mut T::Instance);
    let imp1 = instance.imp();
    let instance = &*(icon2 as *mut T::Instance);
    let imp2 = instance.imp();

    imp1.eq(imp2).into_glib()
}

unsafe extern "C" fn icon_serialize<T: IconImpl>(
    icon: *mut ffi::GIcon,
) -> *mut glib::ffi::GVariant {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.imp();

    imp.serialize().to_glib_full()
}

unsafe extern "C" fn icon_to_tokens<T: IconImpl>(
    icon: *mut ffi::GIcon,
    tokens_ptr: *mut glib::ffi::GPtrArray,
    version_ptr: *mut libc::c_int,
) -> glib::ffi::gboolean {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.imp();

    if let Some((tokens, version)) = imp.to_tokens() {
        *version_ptr = version;
        *tokens_ptr =
            *ToGlibContainerFromSlice::<*mut glib::ffi::GPtrArray>::to_glib_full_from_slice(
                &tokens,
            );
        true.into_glib()
    } else {
        false.into_glib()
    }
}

unsafe extern "C" fn icon_from_tokens<T: IconImpl>(
    tokens_ptr: *mut *mut libc::c_char,
    n_tokens: i32,
    version: i32,
    err_ptr: *mut *mut glib::ffi::GError,
) -> *mut ffi::GIcon {
    let tokens = String::from_glib_none_num_as_vec(tokens_ptr, n_tokens as _);
    match T::from_tokens(tokens.as_slice(), version) {
        Ok(icon) => icon.to_glib_full(),
        Err(err) => {
            *err_ptr = err.into_glib_ptr();
            std::ptr::null_mut()
        }
    }
}
