//! ## fields
//!
//! * `min`
//! * `max`
//!
//! ## Optional fields
//!
//! * `label`
//! * `format`
//! * `power`
//! * `catch`
//!
//! ## Example
//!
//! ```
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Sliders {
//!     #[imgui(slider(min = 0.0, max = 1.0))]
//!     foo: f32,
//!     #[imgui(slider(min = 0, max = 16, format = "bar = %.02f"))]
//!     bar: [i32; 2]
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/X2ue0dS.png
use imgui::{ImStr, Ui};

#[derive(Copy, Clone)]
pub struct SliderParams<'ui, T> {
    pub min: T,
    pub max: T,
    pub label: &'ui imgui::ImStr,
    pub format: Option<&'ui imgui::ImStr>,
    pub power: Option<f32>,
}
pub trait Slider<T> {
    fn build(ui: &imgui::Ui, elem: &mut Self, params: SliderParams<T>) -> bool;
}
