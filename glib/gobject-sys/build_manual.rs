#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    match system_deps::Config::new().probe() {
        Ok(deps) => {
            let msvc = std::env::var("CARGO_CFG_TARGET_ENV")
                .expect("Cargo should set this variable")
                == "msvc";
            let lib = deps
                .get_by_name("gobject_2_0")
                .expect("The dependency key for gobject in its Cargo.toml should not change");
            if msvc && !lib.statik {
                println!("cargo:rustc-cfg=msvc_dll");
            }
        }
        Err(s) => {
            println!("cargo:warning={s}");
            std::process::exit(1);
        }
    }
}
