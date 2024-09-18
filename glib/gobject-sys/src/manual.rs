use glib_sys::GType;

#[cfg(not(msvc_dll))]
type GParamSpecType = *const GType;

#[cfg(msvc_dll)]
type GParamSpecType = *const *const GType;

// When using MSVC, variables marked with dllexport only have the load time linking version
// (prefixed by `__imp_`) publicly exported in the library. This means that it is necessary to either
// call dllimport those symbols (in Rust, this only happens when using the `#[link]` attribute), or
// to manually link to the `__imp_` version when using dynamic libraries. Since we need more customization
// of the library name than the link attribute currently allows, we choose the second option.
extern "C" {
    #[cfg_attr(msvc_dll, link_name = "__imp_g_param_spec_types")]
    static g_param_spec_types: GParamSpecType;
}

/// # Safety
/// This should be safe as long as the offset added to g_param_spec_types is in bounds.
pub unsafe fn g_param_spec_types_get_type(offset: usize) -> GType {
    #[cfg(not(msvc_dll))]
    let ptr = g_param_spec_types;

    // One more step of indirection on windows because `__imp_` signifies a pointer to the
    // underlying symbol https://learn.microsoft.com/en-us/windows/win32/dlls/load-time-dynamic-linking.
    #[cfg(msvc_dll)]
    let ptr = *g_param_spec_types;

    *ptr.add(offset)
}
