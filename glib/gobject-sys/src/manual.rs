// Take a look at the license at the top of the repository in the LICENSE file.

use glib_sys::GType;

extern "C" {
    #[cfg_attr(target_os = "windows", link_name = "__imp_g_param_spec_types")]
    pub static g_param_spec_types: *const GType;
}

// SAFETY: This should be safe as long as the offset added to g_param_spec_types is in bounds.
pub unsafe fn g_param_spec_types_get_type(offset: usize) -> GType {
    *g_param_spec_types.add(offset)
}
