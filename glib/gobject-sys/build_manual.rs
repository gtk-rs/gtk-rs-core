#[cfg(not(docsrs))]
use std::{env, fs::File, io::Write, path::Path};

#[cfg(not(docsrs))]
pub fn main() {
    let deps = system_deps::Config::new()
        .probe()
        .expect("if this module is called, this is valid");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("param_spec.rs");

    // Generating a link attribute is necessary in windows
    // (see https://rust-lang.github.io/rfcs/1717-dllimport.html#drawbacks)
    // even if we are already linking with system_deps.
    // We allow configuration of the library name from upstream dependents
    // using the alias variable in the pkgconfig file.
    let lib_name = deps
        .iter()
        .into_iter()
        .filter_map(|(_, l)| pkg_config::get_variable(&l.name, "alias").ok())
        .find(|s| !s.is_empty())
        .unwrap_or("gobject-2.0".into());
    let link = if cfg!(target_os = "windows") {
        format!("#[link(name = \"{}\")]", lib_name)
    } else {
        "".into()
    };

    let code = format!("{link} extern \"C\" {{ pub static g_param_spec_types: *const GType; }}");
    let mut f = File::create(&out_path).unwrap();
    f.write_all(code.as_bytes()).unwrap();
}
