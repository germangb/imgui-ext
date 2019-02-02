//! ## Optional params
//!
//! * `label`
//! * `flags` identifier to a local function that returns the input flags.
//! * `size` identifier to a local function that returns the size of the input field (for multi-line text).
//! * `catch`
//!
//! ## Example
//!
//! ```
//! use imgui::ImString;
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Form {
//!     #[imgui(text)]
//!     name: ImString,
//!     #[imgui(text)]
//!     email: ImString,
//!     #[imgui(
//!         text(size = "size"),
//!         button(label = "submit", size = "btn_size")
//!     )]
//!     comment: ImString,
//! }
//!
//! const fn size() -> (f32, f32) {
//!     (200.0, 100.0)
//! }
//!
//! const fn btn_size() -> (f32, f32) {
//!     (50.0, 20.0)
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
