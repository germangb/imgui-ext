#[test]
fn checkbox() {
    #[derive(imgui_ext::Gui)]
    struct Test {
        #[imgui(checkbox)]
        a: bool,
        #[imgui(checkbox())]
        b: bool,
        #[imgui(checkbox(label = "foo", catch = "d"))]
        c: bool,
    }
}
