//! ## Optional params
//!
//! * `label`
//! * `flags` Name of a local function that returns the input [flags].
//! * `size` Name of a local function that returns the size of the field (multi-line text).
//! * `catch`
//!
//! [flags]: https://docs.rs/imgui/0.0/imgui/struct.ImGuiInputTextFlags.html
//!
//! ## Example
//!
//! ```
//! use imgui::ImString;
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Form {
//!     #[imgui(text)]
//!     name: ImString,
//!     #[imgui(text)]
//!     email: ImString,
//!     #[imgui(
//!         text(size = "size"),
//!         button(label = "submit")
//!     )]
//!     comment: ImString,
//! }
//!
//! fn size() -> (f32, f32) {
//!     (200.0, 100.0)
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/8pEKoPn.png
use imgui::{ImGuiInputTextFlags, ImStr, ImString, ImVec2, InputText, InputTextMultiline, Ui};

pub struct TextParams<'ui> {
    pub label: &'ui ImStr,
    pub flags: Option<ImGuiInputTextFlags>,
    pub size: Option<ImVec2>,
}

pub trait Text {
    fn build(ui: &Ui, elem: &mut Self, params: TextParams) -> bool;
}

impl<T: Text> Text for Box<T> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: TextParams) -> bool {
        T::build(ui, elem, params)
    }
}

impl Text for ImString {
    fn build(ui: &Ui, elem: &mut Self, params: TextParams) -> bool {
        if let Some(size) = params.size {
            let mut input = InputTextMultiline::new(ui, params.label, elem, size);
            if let Some(flags) = params.flags {
                input = input.flags(flags);
            }
            input.build()
        } else {
            let mut input = InputText::new(ui, params.label, elem);
            if let Some(flags) = params.flags {
                input = input.flags(flags);
            }
            input.build()
        }
    }
}

impl Text for Option<ImString> {
    fn build(ui: &Ui, elem: &mut Self, params: TextParams) -> bool {
        if let Some(ref mut elem) = elem {
            Text::build(ui, elem, params)
        } else {
            false
        }
    }
}
