use imgui::im_str;

use imgui_ext::prelude::*;

mod support;

#[derive(ImGuiExt)]
struct MacroUi {
    //#[imgui(slider(min = 0.0, max = 16.0))]
    speed: f32,
    //#[imgui(input(precission = 4, label = "2D Position"))]
    position_2d: [f32; 2],
    #[imgui(label = "FooBar")]
    foo: bool,
    //#[imgui(progress(width = 128, height = 16))]
    progress: f32,

    // fields without the `#[imgui]` tags aren't part of the Ui
    not_ui: Vec<u8>,

    #[imgui(label = "Super Awesome Mode")]
    awesome_mode: bool,
    #[imgui]
    debug_mode: bool,
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
    };

    support::run(|ui| {
        ui.window(im_str!("window"))
            .build(|| ui.ui_ext(&mut custom));
        true
    });
}
