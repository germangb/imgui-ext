use imgui_ext::imgui_ext;
use imgui_ext::prelude::*;

mod support;
//mod ui;

fn pass_flags() -> imgui::ImGuiInputTextFlags {
    imgui::ImGuiInputTextFlags::Password
}

#[derive(ImGuiExt)]
struct Login {
    #[imgui(input(label = "User"))]
    username: imgui::ImString,
    #[imgui(input(flags = "pass_flags", label = "Pass"))]
    password: imgui::ImString,
}

#[derive(Default, ImGuiExt)]
struct Demo {
    #[imgui(new_line)]
    #[imgui(slider(min = 0.0, max = 10.0, format = "slider %.02f"))]
    #[imgui(drag(label = "Alt"))]
    x: [f32; 2],

    #[imgui(slider(min = 8.0, max = 8.0))]
    #[imgui(input(label = "Y variable", step = 2.0, step_fast = 10.0, precission = 1))]
    y: f32,

    #[imgui(checkbox)]
    #[imgui(label = "Is turbo enabled?")]
    turbo: bool,

    #[imgui(separator)]
    #[imgui(label = "Mouse", display = "x={:.1}, y={:.1}", x, y)]
    mouse: Mouse,

    _bytes: Vec<u8>,

    #[imgui(nested)]
    #[imgui(separator)]
    #[imgui(label = "", display = "Username: {:?}", username)]
    #[imgui(label = "", display = "Password: {:?}", password)]
    creds: Login,

    #[imgui(separator)]
    #[imgui(slider(min = 0, max = 10, label = "in"))]
    #[imgui(drag(min = 0, max = 10, format = "haha", label = "HAHAHA"))]
    #[imgui(input(step = 1, step_fast = 2))]
    int_input: i32,
}

#[derive(Default, Debug, ImGuiExt)]
struct Mouse {
    x: f32,
    y: f32,
}

fn main() {
    let mut demo = Demo::default();

    support::run("Demo", (640, 480), |ui| imgui_ext!(ui, &mut demo));
}

impl Default for Login {
    fn default() -> Self {
        Self {
            username: imgui::ImString::with_capacity(128),
            password: imgui::ImString::with_capacity(128),
        }
    }
}
