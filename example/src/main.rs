use imgui_ext::imgui_ext;
use imgui_ext::prelude::*;

mod support;

#[derive(Default, ImGuiExt)]
struct Demo {
    //#[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    //#[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D", speed = 0.1))]
    drag_2d: [f32; 2],

    #[imgui(
        checkbox(label = "Turbo mode"),
        separator,
        label(label = "Is turbo enabled?"),
    )]
    turbo: bool,
}

fn main() {
    let mut demo = Demo::default();

    support::run("Demo", (640, 480), |ui| imgui_ext!(ui, &mut demo));
}
