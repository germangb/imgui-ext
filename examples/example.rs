use imgui_ext::prelude::*;

mod support;
mod ui;

fn main() {
    let mut demo = ui::Demo::default();

    support::run(|ui| ui.ui_ext(&mut demo));
}
