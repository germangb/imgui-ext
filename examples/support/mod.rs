use imgui::Ui;

mod sdl;

pub fn run<F: FnMut(&Ui) -> bool>(ui: F) {
    sdl::run(ui).unwrap();
}
