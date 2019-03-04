use imgui_ext::ImGuiExt;
use std::mem;

#[derive(ImGuiExt, Debug, Default)]
pub struct Demo {
    #[imgui(text(lit = "f32 widgets"),
            slider(min = 0.0, max = 1.0, label = "slider_f32_1##a"),
            drag(min = 0.0, max = 1.0, label = "slider_f32_1##b"),
            input(label = "slider_f32_1##c"))]
    slider_f32_1: f32,

    #[imgui(new_line,
            text(lit = "f32 widgets 2D"),
            slider(min = 0.0, max = 1.0, label = "slider_f32_2##a"),
            drag(min = 0.0, max = 1.0, label = "slider_f32_2##b"),
            input(label = "slider_f32_2##c"))]
    slider_f32_2: [f32; 2],

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
    slider_f32_5: (f32, f32, f32, f32, f32),
}
