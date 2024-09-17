use glib_sys::GType;

#[cfg(not(target_os = "windows"))]
type GParamSpecType = *const GType;

#[cfg(target_os = "windows")]
type GParamSpecType = *const *const GType;

extern "C" {
    #[cfg_attr(target_os = "windows", link_name = "__imp_g_param_spec_types")]
    static g_param_spec_types: GParamSpecType;
}

/// # Safety
/// This should be safe as long as the offset added to g_param_spec_types is in bounds.
pub unsafe fn g_param_spec_types_get_type(offset: usize) -> GType {
    #[cfg(not(target_os = "windows"))]
    let ptr = g_param_spec_types;

    // One more step of indirection on windows because of the __imp_ prefix
    #[cfg(target_os = "windows")]
    let ptr = *g_param_spec_types;

    *ptr.add(offset)
}
