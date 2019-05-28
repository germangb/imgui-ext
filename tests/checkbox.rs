use imgui_ext::Ui;

#[test]
fn checkbox() {
    #[derive(Ui)]
    struct Test {
        #[imgui(checkbox)]
        a: bool,
        #[imgui(checkbox())]
        b: bool,
        #[imgui(checkbox(label = "foo", catch = "d"))]
        c: bool,
    }
}
