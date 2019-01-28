use imgui::Ui;

mod sdl;

pub fn run<F: FnMut(&Ui)>(ui: F) {
    sdl::run(ui).unwrap();
}
