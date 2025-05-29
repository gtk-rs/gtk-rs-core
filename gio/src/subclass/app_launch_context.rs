// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_char;

use crate::{subclass::prelude::*, AppInfo, AppLaunchContext, File};
use glib::{prelude::*, translate::*, GString, List, Variant};

pub trait AppLaunchContextImpl: ObjectImpl {
    #[doc(alias = "get_display")]
    fn display(&self, info: &AppInfo, files: List<File>) -> Option<GString> {
        self.parent_display(info, files)
    }
    #[doc(alias = "get_startup_notify_id")]
    fn startup_notify_id(&self, info: &AppInfo, files: List<File>) -> Option<GString> {
        self.parent_startup_notify_id(info, files)
    }
    fn launch_failed(&self, startup_notify_id: &str) {
        self.parent_launch_failed(startup_notify_id)
    }
    fn launch_started(&self, info: &AppInfo, platform_data: &Variant) {
        self.parent_launch_started(info, platform_data)
    }
    fn launched(&self, info: &AppInfo, platform_data: &Variant) {
        self.parent_launched(info, platform_data)
    }
}

pub trait AppLaunchContextImplExt: ObjectSubclass {
    fn parent_display(&self, info: &AppInfo, files: List<File>) -> Option<GString>;
    fn parent_startup_notify_id(&self, info: &AppInfo, files: List<File>) -> Option<GString>;
    fn parent_launch_failed(&self, startup_notify_id: &str);
    fn parent_launch_started(&self, info: &AppInfo, platform_data: &Variant);
    fn parent_launched(&self, info: &AppInfo, platform_data: &Variant);
}

impl<T: AppLaunchContextImpl> AppLaunchContextImplExt for T {
    fn parent_display(&self, info: &AppInfo, files: List<File>) -> Option<GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GAppLaunchContextClass;
            (*parent_class).get_display.map(|f| {
                from_glib_full(f(
                    self.obj()
                        .unsafe_cast_ref::<AppLaunchContext>()
                        .to_glib_none()
                        .0,
                    info.to_glib_none().0,
                    files.as_ptr() as *mut _,
                ))
            })
        }
    }

    fn parent_startup_notify_id(&self, info: &AppInfo, files: List<File>) -> Option<GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GAppLaunchContextClass;
            (*parent_class).get_startup_notify_id.map(|f| {
                from_glib_full(f(
                    self.obj()
                        .unsafe_cast_ref::<AppLaunchContext>()
                        .to_glib_none()
                        .0,
                    info.to_glib_none().0,
                    files.as_ptr() as *mut _,
                ))
            })
        }
    }

    fn parent_launch_failed(&self, startup_notify_id: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GAppLaunchContextClass;
            if let Some(f) = (*parent_class).launch_failed {
                f(
                    self.obj()
                        .unsafe_cast_ref::<AppLaunchContext>()
                        .to_glib_none()
                        .0,
                    startup_notify_id.to_glib_none().0,
                )
            }
        }
    }

    fn parent_launch_started(&self, info: &AppInfo, platform_data: &Variant) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GAppLaunchContextClass;
            if let Some(f) = (*parent_class).launch_started {
                f(
                    self.obj()
                        .unsafe_cast_ref::<AppLaunchContext>()
                        .to_glib_none()
                        .0,
                    info.to_glib_none().0,
                    platform_data.to_glib_none().0,
                )
            }
        }
    }

    fn parent_launched(&self, info: &AppInfo, platform_data: &Variant) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GAppLaunchContextClass;
            if let Some(f) = (*parent_class).launched {
                f(
                    self.obj()
                        .unsafe_cast_ref::<AppLaunchContext>()
                        .to_glib_none()
                        .0,
                    info.to_glib_none().0,
                    platform_data.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: AppLaunchContextImpl> IsSubclassable<T> for AppLaunchContext {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.get_display = Some(app_launch_context_get_display::<T>);
        klass.get_startup_notify_id = Some(app_launch_context_get_startup_notify_id::<T>);
        klass.launch_failed = Some(app_launch_context_launch_failed::<T>);
        klass.launched = Some(app_launch_context_launched::<T>);
        klass.launch_started = Some(app_launch_context_launch_started::<T>);
    }
}

unsafe extern "C" fn app_launch_context_get_display<T: AppLaunchContextImpl>(
    ptr: *mut ffi::GAppLaunchContext,
    infoptr: *mut ffi::GAppInfo,
    filesptr: *mut glib::ffi::GList,
) -> *mut c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.display(&from_glib_borrow(infoptr), List::from_glib_none(filesptr))
        .to_glib_full()
}

unsafe extern "C" fn app_launch_context_get_startup_notify_id<T: AppLaunchContextImpl>(
    ptr: *mut ffi::GAppLaunchContext,
    infoptr: *mut ffi::GAppInfo,
    filesptr: *mut glib::ffi::GList,
) -> *mut c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.startup_notify_id(&from_glib_borrow(infoptr), List::from_glib_none(filesptr))
        .to_glib_full()
}

unsafe extern "C" fn app_launch_context_launch_failed<T: AppLaunchContextImpl>(
    ptr: *mut ffi::GAppLaunchContext,
    startup_id: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.launch_failed(&GString::from_glib_borrow(startup_id))
}

unsafe extern "C" fn app_launch_context_launched<T: AppLaunchContextImpl>(
    ptr: *mut ffi::GAppLaunchContext,
    infoptr: *mut ffi::GAppInfo,
    platform_ptr: *mut glib::ffi::GVariant,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.launched(&from_glib_borrow(infoptr), &from_glib_borrow(platform_ptr))
}

unsafe extern "C" fn app_launch_context_launch_started<T: AppLaunchContextImpl>(
    ptr: *mut ffi::GAppLaunchContext,
    infoptr: *mut ffi::GAppInfo,
    platform_ptr: *mut glib::ffi::GVariant,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.launch_started(&from_glib_borrow(infoptr), &from_glib_borrow(platform_ptr))
}
