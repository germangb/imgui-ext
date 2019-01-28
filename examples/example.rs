use imgui::im_str;

use imgui_ext::prelude::*;

mod support;

#[derive(ImGuiExt)]
struct MacroUi {
    #[imgui(slider(min = 0.0, max = 100.0, label = "Speed param", display = "%.02f / 100"))]
    speed: f32,
    #[imgui(slider(min = 0.0, max = 1.0))]
    position_2d: [f32; 2],
    #[imgui(label = "FooBar")]
    foo: bool,
    #[imgui(input(label = "Progress", precission = 4, step = 0.5, step_fast = 2.0))]
    progress: f32,
    #[imgui(input(label = "2D input", precission = 2))]
    input_2d: [f32; 2],
    #[imgui(input(precission = 3))]
    input_3d: [f32; 3],

    #[imgui(input)]
    progress_bar: f32,

    // fields without the `#[imgui]` tags aren't part of the Ui
    not_ui: Vec<u8>,

    #[imgui(label = "Super Awesome Mode")]
    awesome_mode: bool,
    #[imgui]
    debug_mode: bool,

    #[imgui(label = "String literal")]
    literal: &'static str,

    #[imgui(drag)]
    drag: f32,
}

fn main() {
    let mut custom = MacroUi {
        speed: 42.0,
        position_2d: [0.0, 0.0],
        progress: 0.5,
        debug_mode: false,
        not_ui: vec![],
        awesome_mode: true,
        foo: false,
        input_2d: [0.0, 0.0],
        input_3d: [0.0, 0.0, 0.0],
        progress_bar: 0.5,
        literal: "hello world",
        drag: 0.0,
    };

    support::run(|ui| {
        ui.window(im_str!("window"))
            .build(|| ui.ui_ext(&mut custom));
        true
    });
}
