// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ProxyResolver;
use crate::SimpleProxyResolver;
use glib::object::IsA;
use glib::translate::*;

impl SimpleProxyResolver {
    #[doc(alias = "g_simple_proxy_resolver_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(default_proxy: Option<&str>, ignore_hosts: &[&str]) -> ProxyResolver {
        unsafe {
            from_glib_full(ffi::g_simple_proxy_resolver_new(
                default_proxy.to_glib_none().0,
                ignore_hosts.to_glib_none().0,
            ))
        }
    }
}

pub trait SimpleProxyResolverExtManual: 'static {
    #[doc(alias = "g_simple_proxy_resolver_set_ignore_hosts")]
    fn set_ignore_hosts(&self, ignore_hosts: &[&str]);
}

impl<O: IsA<SimpleProxyResolver>> SimpleProxyResolverExtManual for O {
    fn set_ignore_hosts(&self, ignore_hosts: &[&str]) {
        unsafe {
            ffi::g_simple_proxy_resolver_set_ignore_hosts(
                self.as_ref().to_glib_none().0,
                ignore_hosts.to_glib_none().0,
            );
        }
    }
}
