(function() {
    const implementors = Object.fromEntries([["gio_unix",[]],["gio_win32",[]],["glib",[]],["glib_unix",[]],["glib_win32",[]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":59,"fragment_lengths":[15,17,12,17,18]}