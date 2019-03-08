use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;
use imgui_ext::Events;

mod support;
mod ui;

fn main() {
    let mut demo = ui::Demo::new();
    let mut tree = ui::Tree::default();

    demo.set_progress(0.24);

    let mut a = 1;
    let mut b = 0;

    support::run("Demo", (640, 480), |win, ui| {
        //if ui.imgui_ext(&mut demo).color() {
        //    win.color = demo.color();
        //}
        ui.imgui_ext(&mut tree);
    });
}
