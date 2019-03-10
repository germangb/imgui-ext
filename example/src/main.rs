use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;
use imgui_ext::Events;

mod support;
mod ui;

fn main() {
    let mut demo = ui::Demo::default();

    support::run("Demo", (800, 600), |win, ui| {
        ui.imgui_ext(&mut demo);
    });
}
