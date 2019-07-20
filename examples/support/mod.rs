use imgui::{im_str, Ui};
use imgui_ext::{Gui, UiExt};
use std::fmt::Debug;

mod sdl;

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

    pub fn run<T, F>(self, mut input: F)
    where
        T: imgui_ext::Gui + Default,
        F: FnMut(&T, T::Events),
    {
        let title = self
            .win_title
            .as_ref()
            .map(String::as_str)
            .unwrap_or("Window");

        let mut gui = T::default();

        sdl::run(title, self.win_size, |_, ui| {
            if let Some(title) = &self.inner_win_title {
                ui.window(&im_str!("{}", title)).build(|| {
                    let events = ui.draw_gui(&mut gui);
                    input(&gui, events);
                });
            } else {
                let events = ui.draw_gui(&mut gui);
                input(&gui, events);
            }
        })
        .unwrap();
    }

    pub fn run_debug<T, F>(self, mut input: F)
    where
        T: imgui_ext::Gui + Default + Debug,
        F: FnMut(&T, T::Events),
    {
        let title = self
            .win_title
            .as_ref()
            .map(String::as_str)
            .unwrap_or("Window");
        let mut gui = T::default();
        run_custom(title, self.win_size, |_, ui| {
            ui.columns(2, im_str!("columns"), true);
            let events = ui.draw_gui(&mut gui);
            input(&gui, events);
            ui.next_column();
            ui.text_wrapped(&im_str!("{:#?}", gui));
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
