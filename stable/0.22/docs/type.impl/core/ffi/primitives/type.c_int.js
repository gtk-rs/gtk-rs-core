(function() {
    var type_impls = Object.fromEntries([["cairo_sys",[]],["gdk_pixbuf_sys",[]],["gio_sys",[]],["gio_unix_sys",[]],["gio_win32_sys",[]],["glib_sys",[]],["glib_unix_sys",[]],["glib_win32_sys",[]],["graphene_sys",[]],["pango_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[16,22,15,20,21,16,21,22,20,17]}