use imgui::Ui;

mod sdl;

pub fn run<F: FnMut(&Ui)>(title: &str, size: (u32, u32), ui: F) {
    sdl::run(title, size, ui).unwrap();
}
