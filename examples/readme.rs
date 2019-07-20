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
    support::demo().run::<Example, _>(|gui, input| {
        if input.x() {
            println!("x: {}", gui.x)
        }
        if input.y() {
            println!("y: {}", gui.y)
        }
        if input.drag_2d() {
            println!("drag_2d: {:?}", gui.drag_2d)
        }
        if input.turbo() {
            println!("turbo: {}", gui.turbo)
        }
    });
}
