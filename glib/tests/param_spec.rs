#[test]
fn generated_builder() {
    let spec = glib::ParamSpecCharBuilder::new()
        .name("custom-char")
        .build();
    assert_eq!(spec.nick(), "custom-char");
}
