use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;

mod support;

#[derive(ImGuiExt, Default, Debug)]
struct Readme {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(
        checkbox(label = "Turbo mode"),
        display(label = "Is turbo enabled?"),
        button(label = "Reset", catch = "reset")
    )]
    turbo: bool,
}

impl Readme {
    pub fn reset(&mut self) {
        std::mem::replace(self, Default::default());
    }
}

fn main() {
    let mut readme = Readme::default();

    support::run("Demo", (640, 480), |ui| {
        ui.window(im_str!("README.md"))
            .size((400.0, 200.0), ImGuiCond::FirstUseEver)
            .position((50.0, 50.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.columns(2, im_str!("columns"), true);

                if ui.imgui_ext(&mut readme).reset {
                    readme.reset();
                }

                ui.next_column();
                ui.text(format!("{:#?}", readme));
            });
    });
}
