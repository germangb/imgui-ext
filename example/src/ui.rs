use imgui_ext::ImGuiExt;
use std::mem;

#[derive(ImGuiExt, Default)]
pub struct Example {
    #[imgui(text("Input num:"), slider(min = "-1.0", max = 1.0), button(label = "Submit"))]
    number: f32,
}

#[derive(ImGuiExt, Debug, Default)]
pub struct Demo {
    //#[imgui(tree(label = "Tree Node", node(progress)))]
    #[imgui(tree(label = "tree0", node(progress)))]
    _progress: Box<f32>,

    #[imgui(text("f32 widgets"),
            slider(min = "-1.0", max = 1.0, label = "slider_f32_1##a"),
            drag(min = 0.0, max = 1.0, label = "slider_f32_1##b"),
            input(label = "slider_f32_1##c"))]
    slider_f32_1: Box<f32>,

    #[imgui(tree(node(new_line,
                      text(lit = "f32 widgets 2D"),
                      slider(min = 0.0, max = 1.0, label = "slider_f32_2##a"),
                      drag(min = 0.0, max = 1.0, label = "slider_f32_2##b"),
                      input(label = "slider_f32_2##c"))))]
    slider_f32_2: [f32; 2],

    #[imgui(slider(min = "-4", max = 4))]
    i32_1: i32,

    #[imgui(new_line,
            text(lit = "u32 widgets 3D"),
            slider(min = 0, max = 64, label = "slider_u32_3##a"),
            drag(min = 0, max = 64, label = "slider_u32_3##b"),
            input(label = "slider_u32_3##c"))]
    slider_u32_3: [u32; 3],

    #[imgui(new_line,
            slider(min = 0.0, max = 1.0, label = "slider_f32_4##a"),
            drag(min = 0.0, max = 1.0, label = "slider_f32_4##b"),
            input(label = "slider_f32_4##c"))]
    slider_f32_4: [f32; 4],

    #[imgui(new_line,
            slider(min = 0.0, max = 1.0, label = "slider_f32_5##a"),
            drag(min = 0.0, max = 1.0, label = "slider_f32_5##b"),
            input(label = "slider_f32_5##c"))]
    slider_f32_5: Box<(f32, f32, f32, f32, f32)>,

    #[imgui(input)]
    text: imgui::ImString,

    #[imgui(color(edit))]
    color: [f32; 3],
}

impl Demo {
    pub fn new() -> Self {
        Self { text: imgui::ImString::with_capacity(1024), ..Default::default() }
    }

    pub fn set_progress(&mut self, p: f32) {
        *self._progress = p;
    }

    pub fn len(&self) -> usize {
        self.text.to_str().len()
    }
}
