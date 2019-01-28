use imgui_ext::prelude::*;

use imgui::ImString;

#[derive(ImGuiExt)]
pub struct Demo {
    // by default, all items tagged with `#[imgui(..)]` use the attribute name as the label.
    // You can optionally define a `label = "..."` attribute to override this with a custom label.

    /// To add a checkbox widget with a custom label:
    #[imgui(label = "Checkbox UI")]
    checkbox: bool,

    /// structs can also contain fields that *aren't* part of the UI.
    /// Only fields annotated with `#[imgui(...)` will be part of the UI.
    _not_in_ui: Vec<i16>,
    _also_not_in_ui: Option<Box<[u8]>>,


    // numeric types `f32` and `i32` and its arrays types of size up to 4 can be annotated with either:
    //   - `#[imgui(slider( ... ))]`
    //   - `#[imgui(input( ... )]`
    //   - `#[imgui(drag( ... )]`

    /// Here, only `min` and `max` are mandatory.
    /// `power`, `label` and `display` are optional.
    #[imgui(slider(power = 1.0, min = 0.0, max = 100.0, display = "%.02f %%"))]
    slider_1d: f32,

    /// It works for both f32 and i32
    #[imgui(slider(min = 0, max = 4))]
    slider_4d: [i32; 4],

    /// with `#[input(input( ... ))]` can optionally take `step` and `step_fast` parameters,
    /// and in the case of `f32` types, an extra optional (decimal) `precission` attribute.
    #[imgui(input(label = "Integer input 1D", step = 4, step_fast = 16))]
    input_1d_int: i32,
    #[imgui(input(precission = 4, step = 0.1, step_fast = 10.0))]
    input_2d_float: [f32; 2],

    /// `#[imgui(drag( ... ))]` attributes are all optional
    #[imgui(drag(display = "%.04f", min = 0.0, max = 8.0, power = 0.8, speed = 1.0, label = "Drag 1D"))]
    drag_1d: f32,
    #[imgui(drag(min = 0.0, max = 16.0))]
    drag_3d_float: [f32; 3],

    // `drag(...)` is still not implemented for `i32` types
    //#[imgui(drag]
    //drag_2d_int: [f32; 2],

    /// `String`, `&str` and `&ImStr` types will be displayed as text labels
    #[imgui(label = "&str label")]
    label_str: &'static str,
    #[imgui(label = "String label")]
    label_string: String,

    /// InputText is only implemented for `imgui-rs`'s own `ImString` type.
    #[imgui(text(label = "Input text"))]
    input_text: ImString,
}

impl Demo {
    pub fn default() -> Self {
        Self {
            input_text: ImString::new("Hello world"),
            _not_in_ui: vec![],
            _also_not_in_ui: None,
            checkbox: false,
            slider_1d: 0.0,
            slider_4d: [0, 1, 2, 3],
            input_1d_int: 0,
            input_2d_float: [1.0, 1.0],
            drag_1d: 0.0,
            drag_3d_float: [4.0, 5.0, 6.0],
            label_str: "Hello `&str`!",
            label_string: "Hello `String`!".to_string()
        }
    }
}

