use imgui::{im_str, ImGuiCond};

use imgui_ext::prelude::*;

mod support;
mod ui;

fn main() {
    let mut readme = ui::Readme::default();

    support::run("Demo", (640, 480), |win, ui| {
        ui.window(im_str!("Window")).build(|| {
            if ui.imgui_ext(readme.window_mut()).back() {
                win.color = readme.window().back;
            }
        });

        ui.window(im_str!("Example"))
            .size((400.0, 200.0), ImGuiCond::FirstUseEver)
            .position((50.0, 50.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.columns(2, im_str!("columns"), true);

                let events: Events<ui::Readme> = ui.imgui_ext(&mut readme);

                if events.nested().reset() {
                    readme.reset();
                }

                if events.window().back() {
                    win.color = readme.window().back;
                }

                ui.next_column();
                ui.text(format!("{:#?}", readme));
            });
    });
}
