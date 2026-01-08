// Take a look at the license at the top of the repository in the LICENSE file.

use std::sync::LazyLock;

use glib::{
    ffi::{gboolean, GVariant, GVariantType},
    prelude::*,
    subclass::prelude::*,
    translate::*,
};

use crate::{ffi, Action};

pub trait ActionImpl: ObjectImpl + ObjectSubclass<Type: IsA<Action>> {
    fn name(&self) -> glib::GString {
        self.parent_name()
    }

    fn parameter_type(&self) -> Option<glib::VariantType> {
        self.parent_parameter_type()
    }

    fn state_type(&self) -> Option<glib::VariantType> {
        self.parent_state_type()
    }

    fn state_hint(&self) -> Option<glib::Variant> {
        self.parent_state_hint()
    }

    fn is_enabled(&self) -> bool {
        self.parent_enabled()
    }

    fn state(&self) -> Option<glib::Variant> {
        self.parent_state()
    }

    fn change_state(&self, value: glib::Variant) {
        self.parent_change_state(value);
    }

    fn activate(&self, parameter: Option<glib::Variant>) {
        self.parent_activate(parameter);
    }
}

pub trait ActionImplExt: ActionImpl {
    fn parent_name(&self) -> glib::GString {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_name
                .expect("no parent \"get_name\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            from_glib_none(ret)
        }
    }

    fn parent_parameter_type(&self) -> Option<glib::VariantType> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_parameter_type
                .expect("no parent \"get_parameter_type\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            from_glib_none(ret)
        }
    }

    fn parent_state_type(&self) -> Option<glib::VariantType> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_state_type
                .expect("no parent \"get_state_type\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            from_glib_none(ret)
        }
    }

    fn parent_state_hint(&self) -> Option<glib::Variant> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_state_hint
                .expect("no parent \"get_state_hint\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            from_glib_none(ret)
        }
    }

    fn parent_enabled(&self) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_enabled
                .expect("no parent \"get_enabled\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            ret != 0
        }
    }

    fn parent_state(&self) -> Option<glib::Variant> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .get_state
                .expect("no parent \"get_state\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<Action>().to_glib_none().0);
            from_glib_none(ret)
        }
    }

    fn parent_change_state(&self, value: glib::Variant) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .change_state
                .expect("no parent \"change_state\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Action>().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn parent_activate(&self, parameter: Option<glib::Variant>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Action>() as *const ffi::GActionInterface;

            let func = (*parent_iface)
                .activate
                .expect("no parent \"activate\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Action>().to_glib_none().0,
                parameter.to_glib_none().0,
            );
        }
    }

    fn delegate_get_property(
        &self,
        prop_id: usize,
        _pspec: &glib::ParamSpec,
    ) -> Option<glib::Value> {
        let type_: glib::Type = self.obj().type_();
        let property = ActionProperty::from_type(type_, prop_id)?;
        Some(match property {
            ActionProperty::Name => self.name().to_value(),
            ActionProperty::ParameterType => self.parameter_type().to_value(),
            ActionProperty::StateType => self.state_type().to_value(),
            ActionProperty::Enabled => self.is_enabled().to_value(),
            ActionProperty::State => self.state().to_value(),
        })
    }

    fn delegate_set_property(
        &self,
        _prop_id: usize,
        _value: &glib::Value,
        _pspec: &glib::ParamSpec,
    ) -> bool {
        false
    }
}

impl<T: ActionImpl> ActionImplExt for T {}

static FIRST_PROPERTY_ID_QUARK: LazyLock<glib::Quark> =
    LazyLock::new(|| glib::Quark::from_str("gtk-rs-subclass-action-first-prop"));

fn set_first_prop(type_: glib::Type, first_prop: usize) {
    let data = Box::new(first_prop);
    unsafe {
        glib::gobject_ffi::g_type_set_qdata(
            type_.into_glib(),
            FIRST_PROPERTY_ID_QUARK.into_glib(),
            Box::into_raw(data) as *mut _,
        );
    }
}

fn get_first_prop(type_: glib::Type) -> Option<usize> {
    unsafe {
        let ptr = glib::gobject_ffi::g_type_get_qdata(
            type_.into_glib(),
            FIRST_PROPERTY_ID_QUARK.into_glib(),
        ) as *mut usize;
        if ptr.is_null() {
            None
        } else {
            Some(*ptr)
        }
    }
}

#[repr(C)]
enum ActionProperty {
    Name = 0,
    ParameterType,
    StateType,
    Enabled,
    State,
}

impl ActionProperty {
    fn from_id(first_prop: usize, id: usize) -> Option<Self> {
        match id.checked_sub(first_prop)? {
            0 => Some(Self::Name),
            1 => Some(Self::ParameterType),
            2 => Some(Self::StateType),
            3 => Some(Self::Enabled),
            4 => Some(Self::State),
            _ => None,
        }
    }

    fn from_type(mut type_: glib::Type, id: usize) -> Option<Self> {
        loop {
            if let Some(first_prop) = get_first_prop(type_) {
                break ActionProperty::from_id(first_prop, id);
            }
            type_ = type_.parent()?;
        }
    }
}

unsafe impl<T: ActionImpl> IsImplementable<T> for Action {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let instance_type = iface.instance_type();
        let iface = iface.as_mut();

        iface.get_name = Some(action_get_name::<T>);
        iface.get_parameter_type = Some(action_get_parameter_type::<T>);
        iface.get_state_type = Some(action_get_state_type::<T>);
        iface.get_state_hint = Some(action_get_state_hint::<T>);
        iface.get_enabled = Some(action_get_enabled::<T>);
        iface.get_state = Some(action_get_state::<T>);
        iface.change_state = Some(action_change_state::<T>);
        iface.activate = Some(action_activate::<T>);

        unsafe {
            let class_ref = glib::object::Class::<glib::Object>::from_type(instance_type).unwrap();
            let object_class =
                class_ref.as_ref() as *const _ as *mut glib::gobject_ffi::GObjectClass;

            let mut first_prop = std::mem::MaybeUninit::uninit();
            let properties = glib::gobject_ffi::g_object_class_list_properties(
                object_class,
                first_prop.as_mut_ptr(),
            );
            glib::ffi::g_free(properties as *mut _);
            let first_prop = first_prop.assume_init() + 1;

            set_first_prop(instance_type, first_prop as usize);

            glib::gobject_ffi::g_object_class_override_property(
                object_class,
                first_prop + ActionProperty::Name as u32,
                c"name".as_ptr() as *const _,
            );
            glib::gobject_ffi::g_object_class_override_property(
                object_class,
                first_prop + ActionProperty::ParameterType as u32,
                c"parameter-type".as_ptr() as *const _,
            );
            glib::gobject_ffi::g_object_class_override_property(
                object_class,
                first_prop + ActionProperty::StateType as u32,
                c"state-type".as_ptr() as *const _,
            );
            glib::gobject_ffi::g_object_class_override_property(
                object_class,
                first_prop + ActionProperty::Enabled as u32,
                c"enabled".as_ptr() as *const _,
            );
            glib::gobject_ffi::g_object_class_override_property(
                object_class,
                first_prop + ActionProperty::State as u32,
                c"state".as_ptr() as *const _,
            );
        }
    }
}

unsafe extern "C" fn action_get_name<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
) -> *const libc::c_char {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    let instance = imp.obj();
    static QUARK: LazyLock<glib::Quark> =
        LazyLock::new(|| glib::Quark::from_str("gtk-rs-subclass-action-get-name"));

    if let Some(old_name_ptr) = instance.qdata::<glib::GString>(*QUARK) {
        old_name_ptr.as_ref().as_ptr()
    } else {
        instance.set_qdata(*QUARK, imp.name());
        instance
            .qdata::<glib::GString>(*QUARK)
            .unwrap()
            .as_ref()
            .as_ptr()
    }
}

unsafe extern "C" fn action_get_parameter_type<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
) -> *const GVariantType {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    let instance = imp.obj();
    static QUARK: LazyLock<glib::Quark> =
        LazyLock::new(|| glib::Quark::from_str("gtk-rs-subclass-action-get-parameter-type"));

    if let Some(prev) = instance.qdata::<Option<glib::VariantType>>(*QUARK) {
        prev.as_ref().to_glib_none().0
    } else {
        let parameter_type = imp.parameter_type();
        let parameter_type_ptr = parameter_type.to_glib_none().0;
        instance.set_qdata(*QUARK, parameter_type);
        parameter_type_ptr
    }
}

unsafe extern "C" fn action_get_state_type<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
) -> *const GVariantType {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    let instance = imp.obj();
    static QUARK: LazyLock<glib::Quark> =
        LazyLock::new(|| glib::Quark::from_str("gtk-rs-subclass-action-get-state-type"));

    if let Some(prev) = instance.qdata::<Option<glib::VariantType>>(*QUARK) {
        prev.as_ref().to_glib_none().0
    } else {
        let state_type = imp.state_type();
        let state_type_ptr = state_type.to_glib_none().0;
        instance.set_qdata(*QUARK, state_type);
        state_type_ptr
    }
}

unsafe extern "C" fn action_get_state_hint<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
) -> *mut GVariant {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    imp.state_hint().to_glib_full()
}

unsafe extern "C" fn action_get_enabled<T: ActionImpl>(actionptr: *mut ffi::GAction) -> gboolean {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    imp.is_enabled().into_glib()
}

unsafe extern "C" fn action_get_state<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
) -> *mut GVariant {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();

    imp.state().to_glib_full()
}

unsafe extern "C" fn action_change_state<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
    value: *mut GVariant,
) {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();
    let value: glib::Variant = from_glib_none(value);

    imp.change_state(value);
}

unsafe extern "C" fn action_activate<T: ActionImpl>(
    actionptr: *mut ffi::GAction,
    parameterptr: *mut GVariant,
) {
    let instance = &*(actionptr as *mut T::Instance);
    let imp = instance.imp();
    let parameter: Option<glib::Variant> = from_glib_none(parameterptr);

    imp.activate(parameter);
}
