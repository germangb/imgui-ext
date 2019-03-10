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
use std::pin::Pin;

#[derive(Copy, Clone)]
pub struct DragParams<'ui, T> {
    pub label: &'ui ImStr,
    pub format: Option<&'ui ImStr>,
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

impl<T, D: Drag<T> + Unpin> Drag<T> for Pin<Box<D>> {
    fn build(ui: &Ui, elem: &mut Self, params: DragParams<T>) -> bool {
        D::build(ui, elem.as_mut().get_mut(), params)
    }
}

imgui_drag_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, ), 16, sys::ImGuiDataType::Float }
imgui_drag_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, ), 16, sys::ImGuiDataType::Double }
imgui_drag_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, ), 16, sys::ImGuiDataType::U32 }
imgui_drag_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, ), 16, sys::ImGuiDataType::S32 }
