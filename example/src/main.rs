use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;
use imgui_ext::Events;

mod support;
mod ui;

fn main() {
    let mut vars_example = ui::ExampleText::default();
    let mut demo = ui::Demo::default();

    support::run("Demo", (800, 600), |win, ui| {
        ui.window(im_str!("##1")).build(|| {
                                     ui.imgui_ext(&mut vars_example);
                                 });

        ui.window(im_str!("##2")).build(|| {
                                     let event = ui.imgui_ext(&mut demo);
                                     if event.color().background() {
                                         win.color = demo.background_color()
                                     }
                                 });
    });
}
