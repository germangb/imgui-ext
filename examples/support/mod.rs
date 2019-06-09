use imgui::Ui;
use imgui_ext::{Gui, UiExt};
use std::fmt::Debug;

mod sdl;

#[macro_export]
macro_rules! run {
    ($demo:expr) => {
        support::demo().window_title(file!()).run($demo);
    };
}

#[macro_export]
macro_rules! run_debug {
    ($demo:expr) => {
        support::demo().window_title(file!()).run_debug($demo);
    };
}

pub struct Demo {
    win_size: (u32, u32),
    win_title: Option<String>,
    inner_win_title: Option<String>,
}

impl Demo {
    pub fn window_title(mut self, title: &str) -> Self {
        self.win_title = Some(title.to_string());
        self
    }

    pub fn inner_window_title(mut self, title: &str) -> Self {
        self.inner_win_title = Some(title.to_string());
        self
    }

    pub fn window_size(mut self, size: (u32, u32)) -> Self {
        self.win_size = size;
        self
    }

    pub fn run<T: imgui_ext::Gui>(self, mut demo: T) {
        let title = self
            .win_title
            .as_ref()
            .map(String::as_str)
            .unwrap_or("Window");

        sdl::run(title, self.win_size, |_, ui| {
            if let Some(title) = &self.inner_win_title {
                ui.window(imgui::im_str!("{}", title)).build(|| {
                    ui.draw_gui(&mut demo);
                });
            } else {
                ui.draw_gui(&mut demo);
            }
        })
        .unwrap();
    }

    pub fn run_debug<T: imgui_ext::Gui + Debug>(self, mut demo: T) {
        let title = self
            .win_title
            .as_ref()
            .map(String::as_str)
            .unwrap_or("Window");
        run_custom(title, self.win_size, |_, ui| {
            ui.columns(2, imgui::im_str!("columns"), true);
            ui.draw_gui(&mut demo);
            ui.next_column();
            ui.text_wrapped(imgui::im_str!("{:#?}", demo));
        })
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub color: [f32; 4],
    running: bool,
}

impl Window {
    fn default() -> Self {
        Self {
            color: [0.2, 0.2, 0.2, 1.0],
            running: true,
        }
    }

    pub fn close(&mut self) {
        self.running = false;
    }
}

pub fn demo() -> Demo {
    Demo {
        win_size: (640, 480),
        win_title: None,
        inner_win_title: None,
    }
}

pub fn run_custom<F: FnMut(&mut Window, &Ui)>(title: &str, size: (u32, u32), app: F) {
    sdl::run(title, size, app).unwrap();
}
