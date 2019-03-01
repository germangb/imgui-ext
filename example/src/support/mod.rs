use imgui::Ui;

#[cfg(feature = "ci")]
mod dummy {
    use imgui::Ui;
    pub fn run<F: FnMut(&Ui)>(_: &str, _: (u32, u32), _: F) -> Result<(), ()> {
        Ok(())
    }
}
#[cfg(not(feature = "ci"))]
mod sdl;

#[cfg(feature = "ci")]
use dummy as backend;
#[cfg(not(feature = "ci"))]
use sdl as backend;

#[derive(Debug, Clone)]
pub struct Window {
    pub color: [f32; 4],
    running: bool,
}

impl Window {
    fn default() -> Self {
        Self { color: [0.2, 0.2, 0.2, 1.0], running: true }
    }

    pub fn close(&mut self) {
        self.running = false;
    }
}

pub fn run<F: FnMut(&mut Window, &Ui)>(title: &str, size: (u32, u32), ui: F) {
    backend::run(title, size, ui).unwrap();
}
