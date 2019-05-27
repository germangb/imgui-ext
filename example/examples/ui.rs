mod ui_types;

use example::imgui;
use example::imgui_ext::prelude::*;
use example::support;

use ui_types::{Demo, ExampleText};

fn main() {
    let mut vars_example = ExampleText::default();
    let mut demo = Demo::default();

    support::run("Demo", (800, 600), |win, ui| {
        ui.window(imgui::im_str!("##1")).build(|| {
            ui.imgui_ext(&mut vars_example);
        });

        ui.window(imgui::im_str!("##2")).build(|| {
            let event = ui.imgui_ext(&mut demo);
            if event.color().background() {
                win.color = demo.background_color()
            }
        });
    });
}
