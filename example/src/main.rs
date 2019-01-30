use imgui_ext::prelude::*;

mod support;
//mod ui;

#[derive(Default, Debug, ImGuiExt)]
struct Mouse {
    x: f32,
    y: f32,
}

#[derive(ImGuiExt)]
struct Login {
    #[imgui(input(label = "User"))]
    username: imgui::ImString,
    #[imgui(input(flags = "pass_flags", label = "Pass"))]
    password: imgui::ImString,
}

fn pass_flags() -> imgui::ImGuiInputTextFlags {
    imgui::ImGuiInputTextFlags::Password
}

impl Default for Login {
    fn default() -> Self {
        Self {
            username: imgui::ImString::with_capacity(128),
            password: imgui::ImString::with_capacity(128),
        }
    }
}

#[derive(Default, ImGuiExt)]
struct Demo {
    #[imgui(slider(min = 0.0, max = 10.0, format = "slider %.02f"))]
    #[imgui(drag(label = "Alt"))]
    x: [f32; 2],
    #[imgui(slider(min = 0.0, max = 8.0))]
    #[imgui(input(label = "Alt y", step = 2.0, step_fast = 10.0, precission = 4))]
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

    #[imgui(slider(min = 0, max = 10))]
    #[imgui(drag(min = 0, max = 10, format = "haha"))]
    #[imgui(input(step = 1, step_fast = 2))]
    int_input: i32,
}

fn main() {
    let mut demo = Demo::default();

    support::run("Demo", (640, 480), |ui| {
        let (x, y) = ui.imgui().mouse_pos();
        demo.mouse.x = x;
        demo.mouse.y = y;
        ui.ui_ext(&mut demo)
    });
}
