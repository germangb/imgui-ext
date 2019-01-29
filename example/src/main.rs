use imgui_ext::prelude::*;

mod support;
//mod ui;

#[derive(Default)]
struct Mouse {
    x: f32,
    y: f32,
}

#[derive(Default, ImGuiExt)]
struct Demo {
    //#[imgui(input(precission = 2))]
    //#[imgui(label = "X param")]
    #[imgui(slider(min = 0.0, max = 10.0))]
    #[imgui(drag(label = "Alt"))]
    x: [f32; 2],
    //#[imgui(slider(min = 0.0, max = 4.0))]
    y: f32,

    #[imgui(checkbox)]
    #[imgui(label = "Is turbo enabled?")]
    turbo: bool,

    #[imgui(separator)]
    #[imgui(label = "Mouse", display = "{:.1}, {:.1}", x, y)]
    mouse: Mouse,

    bytes: Vec<u8>,
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
