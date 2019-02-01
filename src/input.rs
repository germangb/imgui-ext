//!
//! The input trait is implemented for numeric types (`f32` and `i32`) and their corresponding
//! array types of up to 4 elements, and [`ImString`]
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Example {
//!     // parameters in input() are all optional
//!     #[imgui(input)]
//!     input_0: f32,
//!
//!     // `precision = ..` specifies the decimal precision.
//!     // This parameter only has an effect in f32 types.
//!     #[imgui(input(precision = 2))]
//!     input_1: [f32; 2],
//!
//!     // `step` and `step_fast`
//!     #[imgui(input(step = 4, step_fast = 42))]
//!     input_2: i32,
//! }
//! ```
//!
//! ### Result
//!
//! ![result][result]
//!
//! ## Custom input flags
//!
//! You can specify a local function from where to load any input flags.
//!
//! The only is that these flags cannot be changed at runtime.
//!
//! ```ignore
//! use imgui::ImGuiInputTextFlags;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(input(flags = "my_flags"))]
//!     input_2: i32,
//! }
//!
//! fn my_flags() -> ImGuiInputTextFlags {
//!     ImGuiInputTextFlags::Password
//! }
//! ```
//!
//! [`ImString`]: #
//! [result]: https://i.imgur.com/BPvMGAp.png
use imgui::{ImGuiInputTextFlags, ImStr, Ui};

#[derive(Copy, Clone)]
pub struct InputParams<'ui, T> {
    pub label: &'ui ImStr,
    pub precision: Option<i32>,
    pub step: Option<T>,
    pub step_fast: Option<T>,
    pub flags: Option<ImGuiInputTextFlags>,
}

pub trait Input<T> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>) -> bool;
}
