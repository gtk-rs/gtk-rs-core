(function() {
    var type_impls = Object.fromEntries([["cairo_sys",[]],["glib_sys",[]],["pango",[]],["pango_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[16,16,13,17]}