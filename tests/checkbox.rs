use imgui_ext::ImGuiExt;

#[test]
fn checkbox() {
    #[derive(ImGuiExt)]
    struct Test {
        #[imgui(checkbox)]
        a: bool,
        #[imgui(checkbox())]
        b: bool,
        #[imgui(checkbox(label = "foo", catch = "d"))]
        c: bool,
    }
}
