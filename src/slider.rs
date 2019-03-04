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
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Sliders {
//!     #[imgui(slider(min = 0.0, max = 1.0))]
//!     foo: f32,
//!     #[imgui(slider(min = 0, max = 16, format = "bar = %.02f"))]
//!     bar: [i32; 2],
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/X2ue0dS.png
use imgui::sys;
use imgui::{ImStr, Ui};

#[derive(Copy, Clone)]
pub struct SliderParams<'ui, T> {
    pub min: T,
    pub max: T,
    pub label: &'ui ImStr,
    pub format: Option<&'ui ImStr>,
    pub power: Option<f32>,
}

pub trait Slider<T> {
    fn build(ui: &imgui::Ui, elem: &mut Self, params: SliderParams<T>) -> bool;
}

impl<T, S: Slider<T>> Slider<T> for Option<S> {
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<T>) -> bool {
        if let Some(ref mut elem) = elem {
            S::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl<T, S: Slider<T>> Slider<T> for Box<S> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<T>) -> bool {
        S::build(ui, elem, params)
    }
}

imgui_slider_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, ), 16, sys::ImGuiDataType::Float }
imgui_slider_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, ), 16, sys::ImGuiDataType::Double }
imgui_slider_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, ), 16, sys::ImGuiDataType::U32 }
imgui_slider_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, ), 16, sys::ImGuiDataType::S32 }
