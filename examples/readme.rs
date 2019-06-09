mod support;

#[derive(imgui_ext::Gui, Default, Debug)]
struct Example {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(checkbox(label = "Turbo mode"))]
    turbo: bool,
}

fn main() {
    let example = Example::default();

    support::demo()
        .window_title(file!())
        .inner_window_title("README.md")
        .run(example);
}
