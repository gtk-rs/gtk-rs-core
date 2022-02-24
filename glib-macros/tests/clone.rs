// Take a look at the license at the top of the repository in the LICENSE file.

use glib::clone;
use std::rc::Rc;

#[test]
fn clone() {
    let v = Rc::new(1);
    let _ = clone!(@strong v => @default-return None::<i32>, move || {println!("foo"); 1});
}

const TESTS: &[(&str, &str)] = &[
    ("clone!( => move || {})",
     "If you have nothing to clone, no need to use this macro!"),
    ("clone!(|| {})",
     "If you have nothing to clone, no need to use this macro!"),
    ("clone!(|a, b| {})",
     "If you have nothing to clone, no need to use this macro!"),
    ("clone!(@weak a, @weak b => |x| {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => || {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => |x| println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => || println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => |x| {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => || {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => |x| println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => || println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@strong self => move |x| {})",
     "Can't use `self` as variable name. Try storing it in a temporary variable or rename it using `as`."),
    ("clone!(@strong self.v => move |x| {})",
     "`self.v`: Field accesses are not allowed as is, you must rename it!"),
    ("clone!(@weak v => @default-return false, || {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, || println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, |bla| {})",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, |bla| println!(\"a\"))",
     "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => default-return false, move || {})",
     "Missing `@` before `default-return`"),
    ("clone!(@weak v => @default-return false move || {})",
     "Expected `,` after `@default-return false`, found `,`"),
    ("clone!(@yolo v => move || {})",
     "Unknown keyword `yolo`, only `weak`, `weak-allow-none` and `strong` are allowed"),
    ("clone!(v => move || {})",
     "Unexpected ident `v`: you need to specify if this is a weak or a strong clone."),
    ("clone!(@strong v => {println!(\"foo\");})",
     "Missing `move` and closure declaration"),
    ("clone!(@strong v, @default-return lol => move || {println!(\"foo\");})",
     "`@default-return` should be after `=>`"),
    ("clone!(@default-return lol, @strong v => move || {println!(\"foo\");})",
     "`@default-return` should be after `=>`"),
    // The async part!
    ("clone!(@strong v => async || {println!(\"foo\");})",
     "Expected `move` after `async`, found `|`"),
    ("clone!(@strong v => async {println!(\"foo\");})",
     "Expected `move` after `async`, found `{`"),
    ("clone!(@strong v => move || async {println!(\"foo\");})",
     "Expected `move` after `async`, found `{`"),
    ("clone!(@strong v => move || async println!(\"foo\");)",
     "Expected `move` after `async`, found `println`"),
    ("clone!(@strong v => move || async move println!(\"foo\");)",
     "Expected block after `| async move`"),
];

#[test]
fn clone_failures() {
    let exe = std::env::current_exe().unwrap();
    let out_dir = exe.parent().unwrap();
    let t = trybuild::TestCases::new();
    let test_dir = out_dir.join("clone-tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    for (index, (expr, err)) in TESTS.iter().enumerate() {
        let expr = expr.to_string();
        let file = test_dir.join(format!("failure{}.rs", index));
        let prefix = "fn main() { use glib::clone; let v = std::rc::Rc::new(1); ";
        let suffix = "; }";
        let output = format!("{}{}{}", prefix, expr, suffix);
        std::fs::write(&file, output.to_string().as_bytes()).unwrap();

        let stderr_file = test_dir.join(format!("failure{}.stderr", index));
        let stderr = format!(
            "\
error: proc macro panicked
 --> {}:1:{}
  |
1 | {}
  | {}{}
  |
  = help: message: {}\n",
            file.to_str().unwrap(),
            prefix.len() + 1,
            output,
            " ".repeat(prefix.len()),
            "^".repeat(expr.len()),
            err
        );
        std::fs::write(&stderr_file, stderr.to_string().as_bytes()).unwrap();

        t.compile_fail(file);
    }
}
