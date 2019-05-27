//!
//! ## Optional params
//!
//! * `min`
//! * `max`
//! * `speed`
//! * `power`
//! * `format` (printf format)
//! * `catch`
use imgui::sys;
use imgui::{ImStr, Ui};

#[derive(Copy, Clone)]
pub struct DragParams<'a, T> {
    pub label: &'a ImStr,
    pub format: Option<&'a ImStr>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub speed: Option<f32>,
    pub power: Option<f32>,
}

pub trait Drag<T> {
    fn build(ui: &imgui::Ui, elem: &mut Self, params: DragParams<T>) -> bool;
}

impl<T, D: Drag<T>> Drag<T> for Option<D> {
    fn build(ui: &Ui, elem: &mut Self, params: DragParams<T>) -> bool {
        if let Some(ref mut elem) = elem {
            D::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl<T, D: Drag<T>> Drag<T> for Box<D> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: DragParams<T>) -> bool {
        D::build(ui, elem, params)
    }
}

imgui_drag_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32, ), 8, sys::ImGuiDataType::Float }
imgui_drag_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64, ), 8, sys::ImGuiDataType::Double }
imgui_drag_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32, ), 8, sys::ImGuiDataType::U32 }
imgui_drag_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32, ), 8, sys::ImGuiDataType::S32 }
