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
//! * `map` Applies a mapping function to `&mut Self`.
//!
//! ## Example
//!
//! ```
//! #[derive(imgui_ext::Ui)]
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

pub struct SliderParams<'a, T> {
    pub min: T,
    pub max: T,
    pub label: &'a ImStr,
    pub format: Option<&'a ImStr>,
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

imgui_slider_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, sys::ImGuiDataType::Float }
imgui_slider_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, sys::ImGuiDataType::Double }
imgui_slider_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, sys::ImGuiDataType::U32 }
imgui_slider_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, sys::ImGuiDataType::S32 }

// matrix types
// TODO macro Y expansion

imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 8, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 7, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 6, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 5, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 4, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 3, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 2, sys::ImGuiDataType::Float }
imgui_slider_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 1, sys::ImGuiDataType::Float }

imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 8, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 7, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 6, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 5, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 4, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 3, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 2, sys::ImGuiDataType::Double }
imgui_slider_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 1, sys::ImGuiDataType::Double }

imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 8, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 7, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 6, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 5, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 4, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 3, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 2, sys::ImGuiDataType::U32 }
imgui_slider_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 1, sys::ImGuiDataType::U32 }

imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 8, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 7, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 6, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 5, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 4, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 3, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 2, sys::ImGuiDataType::S32 }
imgui_slider_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 1, sys::ImGuiDataType::S32 }
