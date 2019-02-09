use std::mem;

use imgui_ext::ImGuiExt;

mod nested {
    use imgui::{ImGuiInputTextFlags as Flags, ImString};
    use imgui_ext::ImGuiExt;

    #[derive(ImGuiExt, Debug)]
    pub struct Nested {
        #[imgui(text(catch = "foo"))]
        foo: ImString,
        #[imgui(text(flags = "flags"), button(label = "Reset", catch = "reset"))]
        bar: ImString,
    }

    impl Default for Nested {
        fn default() -> Self {
            Self {
                foo: ImString::with_capacity(64),
                bar: ImString::with_capacity(64),
            }
        }
    }

    fn flags() -> Flags {
        Flags::Password
    }
}

#[derive(ImGuiExt, Debug, Default)]
pub struct Readme {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(checkbox(label = "Turbo mode"), display(label = "Is turbo enabled?"))]
    turbo: bool,

    // Ui can have memory indirections
    #[imgui(nested)]
    nested: Box<nested::Nested>,

    #[imgui(nested)]
    window: Window,
}

#[derive(ImGuiExt, Debug)]
pub struct Window {
    #[imgui(
        color(button(preview = "HalfAlpha")),
        color(edit(preview = "HalfAlpha")),
        color(picker(mode = "HueWheel"))
    )]
    pub back: [f32; 4],
}

impl Default for Window {
    fn default() -> Self {
        Self {
            back: [0.2, 0.2, 0.2, 1.0],
        }
    }
}

impl Readme {
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn reset(&mut self) {
        mem::replace(self, Default::default());
    }
}
