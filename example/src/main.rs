use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;
use imgui_ext::Events;

mod support;
mod ui;

fn main() {
    let mut demo = ui::Demo::new();

    support::run("Demo", (640, 480), |win, ui| {
        ui.imgui_ext(&mut demo);
    });
}
