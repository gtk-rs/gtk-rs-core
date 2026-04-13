(function() {
    var type_impls = Object.fromEntries([["cairo_sys",[]],["gdk_pixbuf_sys",[]],["gio_sys",[]],["glib_sys",[]],["gobject_sys",[]],["pango_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[16,22,15,16,19,17]}