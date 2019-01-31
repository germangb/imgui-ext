use imgui_ext::imgui_ext;
use imgui_ext::prelude::*;

use imgui::ImGuiInputTextFlags;

mod support;

#[derive(Default, ImGuiExt)]
struct Demo {
    //#[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(slider(min = 0.0, max = 1.0))]
    foo: f32,
    #[imgui(drag(label = "Drag 2D", speed = 0.1))]
    drag_2d: [f32; 2],

    #[imgui(
        checkbox(label = "Turbo mode"),
        separator,
        label(label = "Is turbo enabled?")
    )]
    turbo: bool,
}

#[derive(Default, ImGuiExt)]
struct Form {
    #[imgui(input)]
    user: imgui::ImString,
    #[imgui(input(flags = "passwd_flags"))]
    passwd: imgui::ImString,
}

fn passwd_flags() -> ImGuiInputTextFlags {
    ImGuiInputTextFlags::Password
}

#[derive(Default, ImGuiExt)]
struct Example {
    #[imgui(nested)]
    login_form: Form,
    #[imgui(checkbox(label = "Remember login?"))]
    remember: bool,
}

fn main() {
    let mut demo = Example::default();

    demo.login_form.user = imgui::ImString::with_capacity(64);
    demo.login_form.passwd = imgui::ImString::with_capacity(64);

    support::run("Demo", (640, 480), |ui| imgui_ext!(ui, &mut demo));
}
