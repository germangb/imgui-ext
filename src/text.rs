//! ## Optional params
//!
//! * `label`
//! * `flags` identifier to a local function that returns the input flags.
//! * `size` identifier to a local function that returns the size of the input field (for multi-line text).
//!
//! ## Example
//!
//! ```
//! use imgui::{ImString, ImGuiInputTextFlags as Flags};
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Form {
//!     #[imgui(text)]
//!     user: ImString,
//!     #[imgui(text(flags = "flags"))]
//!     passwd: ImString,
//! }
//!
//! fn flags() -> Flags {
//!     Flags::Password
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
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
