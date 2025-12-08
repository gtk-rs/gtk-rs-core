(function() {
    var type_impls = Object.fromEntries([["gdk_pixbuf_sys",[]],["gio_sys",[]],["glib_sys",[]],["glib_unix_sys",[]],["glib_win32_sys",[]],["graphene_sys",[]],["pango_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[21,15,16,21,22,20,17]}