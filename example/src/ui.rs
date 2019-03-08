use imgui::{ImGuiTreeNodeFlags, ImString};
use imgui_ext::ImGuiExt;
use std::mem;

#[derive(ImGuiExt, Default)]
pub struct Example {
    #[imgui(text("Input num:"), slider(min = "-1.0", max = 1.0), button(label = "Submit"))]
    number: f32,
}

#[derive(ImGuiExt, Default)]
pub struct Tree {
    #[imgui(tree(label = "Sliders", cond = "FirstUseEver", flags = "flags", node(nested)))]
    sliders: Sliders,
    #[imgui(tree(label = "Inputs", flags = "flags", node(nested)))]
    inputs: Inputs,
    #[imgui(tree(label = "Color picker", flags = "flags", node(color(picker))))]
    color: [f32; 3],
}

fn flags() -> ImGuiTreeNodeFlags {
    ImGuiTreeNodeFlags::Framed
}

#[derive(ImGuiExt, Default)]
pub struct Sliders {
    #[imgui(text("Slider widgets:"), slider(min = 0.0, max = 3.0))]
    s1: f32,
    #[imgui(slider(min = "-4", max = 4))]
    s2: [i32; 3],
    #[imgui(slider(min = "-1.0", max = 1.0))]
    s3: [f64; 2],
}

#[derive(ImGuiExt, Default)]
pub struct Inputs {
    #[imgui(text("Input widgets:"), input)]
    i1: f32,
    #[imgui(input)]
    i2: imgui::ImString,
    #[imgui(input)]
    i3: [f32; 8],
}

#[derive(ImGuiExt, Debug, Default)]
pub struct Demo {
    #[imgui(tree(label = "progress bar", node(progress, slider(min = 0.0, max = 1.0))))]
    _progress: Box<f32>,

    #[imgui(tree(node(text("f32 widgets"),
                      slider(min = "-1.0", max = 1.0, label = "slider_f32_1##a"),
                      drag(min = 0.0, max = 1.0, label = "slider_f32_1##b"),
                      input(label = "slider_f32_1##c"))))]
    slider_f32_1: Box<f32>,

    #[imgui(tree(node(new_line,
                      text(lit = "f32 widgets 2D"),
                      slider(min = 0.0, max = 1.0, label = "slider_f32_2##a"),
                      drag(min = 0.0, max = 1.0, label = "slider_f32_2##b"),
                      input(label = "slider_f32_2##c"))))]
    slider_f32_2: [f32; 2],

    #[imgui(tree(node(slider(min = "-4", max = 4))))]
    i32_1: i32,

    #[imgui(tree(node(new_line,
                      text(lit = "u32 widgets 3D"),
                      slider(min = 0, max = 64, label = "slider_u32_3##a"),
                      drag(min = 0, max = 64, label = "slider_u32_3##b"),
                      input(label = "slider_u32_3##c"))))]
    slider_u32_3: [u32; 3],

    #[imgui(tree(node(new_line,
                      slider(min = 0.0, max = 1.0, label = "slider_f32_4##a"),
                      drag(min = 0.0, max = 1.0, label = "slider_f32_4##b"),
                      input(label = "slider_f32_4##c"))))]
    slider_f32_4: [f32; 4],

    #[imgui(tree(node(new_line,
                      slider(min = 0.0, max = 1.0, label = "slider_f32_5##a"),
                      drag(min = 0.0, max = 1.0, label = "slider_f32_5##b"),
                      input(label = "slider_f32_5##c"))))]
    slider_f32_5: Box<(f32, f32, f32, f32, f32)>,

    #[imgui(input)]
    text: imgui::ImString,

    #[imgui(color(edit))]
    color: [f32; 4],
}

impl Demo {
    pub fn new() -> Self {
        Self { text: imgui::ImString::with_capacity(1024),
               color: [0.2, 0.2, 0.2, 1.0],
               ..Default::default() }
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_progress(&mut self, p: f32) {
        *self._progress = p;
    }

    pub fn len(&self) -> usize {
        self.text.to_str().len()
    }
}
