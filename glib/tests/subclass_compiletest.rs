#[test]
pub fn test() {
    let t = trybuild2::TestCases::new();

    t.pass("tests/subclass_compiletest/01-auto-send-sync.rs");
    t.compile_fail("tests/subclass_compiletest/02-no-auto-send-sync.rs");
    t.compile_fail("tests/subclass_compiletest/03-object-no-auto-send-sync.rs");
}
