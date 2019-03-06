//! ## Optional params
//!
//! * `lit` Literal text. If this value is set, this value is displayed instead of the annotated type.
//!
//! You can also write this annotation as:
//!
//! * `#[imgui(text("literal..."))]`
//!
//! which is a shorthand form for `text(lit = "literal...")`.
//!
//! ## Example
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(text("Input num:"),
//!             slider(min = "-1.0", max = 1.0),
//!             button(label = "Submit"))]
//!     number: f32,
//! }
//! ```
//!
//! ### Result
//!
//! ![](https://i.imgur.com/axt41Zp.png)
//!
use imgui::Ui;

pub trait Text {
    fn build(ui: &Ui, elem: &Self);
}

impl<S: AsRef<str>> Text for S {
    fn build(ui: &Ui, elem: &Self) {
        ui.text(elem)
    }
}
