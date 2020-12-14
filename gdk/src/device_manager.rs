// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DeviceManager;

impl DeviceManager {
    pub fn disable_multidevice() {
        assert_not_initialized!();
        unsafe { ffi::gdk_disable_multidevice() }
    }
}
