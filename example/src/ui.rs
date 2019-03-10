use imgui::{ImGuiTreeNodeFlags, ImString};
use imgui_ext::ImGuiExt;

#[derive(ImGuiExt, Default)]
pub struct Demo {
    #[imgui(tree(label = "Input widgets", flags = "flags", node(nested)))]
    input: Input,
    #[imgui(tree(label = "Drag widgets", flags = "flags", node(nested)))]
    drag: Drag,
    #[imgui(tree(label = "Slider widgets", flags = "flags", node(nested)))]
    slider: Slider,
}

fn flags() -> ImGuiTreeNodeFlags {
    ImGuiTreeNodeFlags::Framed
}

#[derive(ImGuiExt, Default)]
pub struct Input {
    #[imgui(input(label = "Float 1D"))]
    input_1_f32: f32,
    #[imgui(input(label = "Float 2D"))]
    input_2_f32: [f32; 2],
    #[imgui(input(label = "Float 3D"))]
    input_3_f32: [f32; 3],
    #[imgui(input(label = "Signed int 1D"))]
    input_1_i32: i32,
    #[imgui(input(label = "Signed int 2D"))]
    input_2_i32: [i32; 2],
    #[imgui(input(label = "Signed int 3D"))]
    input_3_i32: [i32; 3],
}

#[derive(ImGuiExt, Default)]
pub struct Drag {
    #[imgui(drag(label = "Float 1D"))]
    drag_1_f32: f32,
    #[imgui(input(label = "Float 2D"))]
    drag_2_f32: [f32; 2],
    #[imgui(drag(label = "Float 3D"))]
    drag_3_f32: [f32; 3],
    #[imgui(drag(label = "Signed int 1D"))]
    drag_1_i32: i32,
    #[imgui(drag(label = "Signed int 2D"))]
    drag_2_i32: [i32; 2],
    #[imgui(drag(label = "Signed (bounded) int 3D", min = "-8", max = 8))]
    drag_3_i32: [i32; 3],
}

#[derive(ImGuiExt, Default)]
pub struct Slider {}
