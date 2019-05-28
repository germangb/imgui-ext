use imgui::{ImGuiTreeNodeFlags, ImString};

#[derive(imgui_ext::Gui, Default)]
pub struct Demo {
    #[imgui(vars(
        style = "style",
        content(tree(label = "Input widgets", flags = "flags", node(nested)))
    ))]
    input: Input,
    #[imgui(tree(label = "Drag widgets", flags = "flags", node(nested)))]
    drag: Drag,
    #[imgui(tree(label = "Slider widgets", flags = "flags", node(nested)))]
    slider: Slider,
    #[imgui(tree(label = "Color widgets", flags = "flags", node(nested)))]
    color: Color,
    #[imgui(tree(label = "Colors & style vars", flags = "flags", node(nested)))]
    styles: Styles,
}

impl Demo {
    pub fn background_color(&self) -> [f32; 4] {
        self.color.background
    }
}

fn flags() -> ImGuiTreeNodeFlags {
    ImGuiTreeNodeFlags::Framed
}

#[derive(imgui_ext::Gui, Default)]
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

#[derive(imgui_ext::Gui, Default)]
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

#[derive(imgui_ext::Gui, Default)]
pub struct Slider {
    #[imgui(slider(min = "-1.0", max = "1.0"))]
    slider_1: f32,
}

#[derive(imgui_ext::Gui)]
pub struct Color {
    #[imgui(
        text(lit = "NOTE: Window opacity is not available in all platforms"),
        tree(label = "button", node(color(button))),
        tree(label = "edit", node(color(edit))),
        tree(label = "picker", node(color(picker)))
    )]
    background: [f32; 4],
}

impl Default for Color {
    fn default() -> Self {
        Self {
            background: [0.2, 0.2, 0.2, 1.0],
        }
    }
}

#[derive(imgui_ext::Gui, Default)]
pub struct Styles {
    #[imgui(
        button(label = "Button"),
        vars(color = "color", style = "style", content(button(label = "Button")))
    )]
    _ph: (),
}

use imgui::{ImGuiCol, ImVec2, StyleVar};

fn style() -> &'static [StyleVar] {
    &[
        StyleVar::FrameRounding(4.0),
        StyleVar::WindowPadding(ImVec2 { x: 4.0, y: 4.0 }),
    ]
}

fn color() -> &'static [(ImGuiCol, [f32; 4])] {
    &[(ImGuiCol::Button, [1.0, 0.0, 1.0, 1.0])]
}

#[derive(imgui_ext::Gui, Default)]
pub struct VarsExample {
    #[imgui(vars(
        style = "example_style",
        color = "example_color",
        content(
            input(label = "foo##input"),
            drag(label = "foo##drag"),
            slider(label = "foo##slider", min = "-1.0", max = "1.0")
        )
    ))]
    foo: f32,
}

fn example_style() -> &'static [StyleVar] {
    &[StyleVar::FrameRounding(4.0)]
}

fn example_color() -> &'static [(ImGuiCol, [f32; 4])] {
    &[(ImGuiCol::Button, [1.0, 0.0, 1.0, 1.0])]
}

#[derive(imgui_ext::Gui, Default)]
pub struct ExampleText {
    #[imgui(text_wrap("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc metus sem, facilisis hendrerit elementum et, egestas."),
    separator(),
    text("Input num:"),
    slider(min = "-1.0", max = 1.0),
    button(label = "Submit"))]
    number: f32,
}
