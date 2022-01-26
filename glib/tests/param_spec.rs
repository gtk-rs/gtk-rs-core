// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
fn generated_builder() {
    let spec = glib::ParamSpecCharBuilder::new()
        .name("custom-char")
        .flags(glib::ParamFlags::READABLE)
        .build();
    assert_eq!(spec.nick(), "custom-char");
    assert_eq!(spec.flags(), glib::ParamFlags::READABLE);
}
