// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use crate::TabArray;

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
impl std::str::FromStr for TabArray {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}
