use imgui::im_str;

use imgui_ext::UiExt;
use ui_types::{Demo, ExampleText};

mod ui_types;

fn main() {
    let mut vars_example = ExampleText::default();
    let mut demo = Demo::default();

    example::support::run_custom("Demo", (800, 600), |win, ui| {
        ui.window(im_str!("##1")).build(|| {
            ui.render_gui(&mut vars_example);
        });

        ui.window(im_str!("##2")).build(|| {
            let event = ui.render_gui(&mut demo);
            if event.color().background() {
                win.color = demo.background_color()
            }
        });
    });
}
