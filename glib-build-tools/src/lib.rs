// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]

use std::{env, path::Path, process::Command};

// rustdoc-stripper-ignore-next
/// Call to run `glib-compile-resources` to generate compiled gresources to embed
/// in binary with [`gio::resources_register_include`]. `target` is relative to `OUT_DIR`.
///
/// ```no_run
/// glib_build_tools::compile_resources(
///     &["resources"],
///     "resources/resources.gresource.xml",
///     "compiled.gresource",
/// );
/// ```
pub fn compile_resources<P: AsRef<Path>>(source_dirs: &[P], gresource: &str, target: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut command = Command::new("glib-compile-resources");

    for source_dir in source_dirs {
        command.arg("--sourcedir").arg(source_dir.as_ref());
    }

    let output = command
        .arg("--target")
        .arg(out_dir.join(target))
        .arg(gresource)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "glib-compile-resources failed with exit status {} and stderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    println!("cargo:rerun-if-changed={gresource}");
    let mut command = Command::new("glib-compile-resources");

    for source_dir in source_dirs {
        command.arg("--sourcedir").arg(source_dir.as_ref());
    }

    let output = command
        .arg("--generate-dependencies")
        .arg(gresource)
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    for dep in output.split_whitespace() {
        println!("cargo:rerun-if-changed={dep}");
    }
}

// rustdoc-stripper-ignore-next
/// Call to run `glib-compile-schemas` to generate compiled gschemas from `.gschema.xml` schemas
/// files.
///
/// ```no_run
/// glib_build_tools::compile_schemas(
///     &["schemas"]
/// );
/// ```
pub fn compile_schemas(schemas_dir: &[&str]) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let target_dir = out_dir.join("gschemas");

    // Ensure target_dir exists
    std::fs::create_dir_all(&target_dir).expect("Failed to create target directory");

    println!(
        "cargo:rustc-env=GSETTINGS_SCHEMA_DIR={}",
        target_dir.to_str().unwrap()
    );

    // Recursively copy all files with .gschema.xml extension from schema_dir to target_dir
    for schema_dir in schemas_dir {
        let entries = Path::new(schema_dir)
            .read_dir()
            .expect("Failed to read schema directory")
            .flatten();

        for entry in entries {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if path.is_file() && file_name.ends_with(".gschema.xml") {
                let target_path = target_dir.join(path.file_name().unwrap());
                std::fs::copy(&path, &target_path).expect("Failed to copy schema file");
            }
        }
    }

    let mut command = Command::new("glib-compile-schemas");
    command.arg("--strict");
    command.arg(target_dir);

    let output = command
        .output()
        .expect("Failed to execute glib-compile-schemas");

    assert!(
        output.status.success(),
        "glib-compile-schemas failed with exit status {} and stderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    for schema_dir in schemas_dir {
        println!("cargo:rerun-if-changed={}", schema_dir);
    }
}
