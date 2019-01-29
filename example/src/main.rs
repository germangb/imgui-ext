use imgui_ext::prelude::*;

mod support;
//mod ui;

#[derive(Default, ImGuiExt)]
struct Demo {
    #[imgui(input)]
    x: f32,
    //#[imgui(input(min = 0.0, max = 4.0))]
    y: f32,

    /*
    /// multiple widgets per field are allowed as long as they're
    /// compatible with the type
    #[imgui(input)]
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    */

    #[imgui(separator)]
    #[imgui(label = "Turbo mode", display = "{:?}", turbo)]
    turbo: bool,

    #[imgui(display = "{:?}", bytes)]
    bytes: Vec<u8>,
}

fn main() {
    let mut demo = Demo::default();

    support::run("Demo", (640, 480), |ui| ui.ui_ext(&mut demo));
}
