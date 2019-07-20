//!
//! ## Optional params
//!
//! * `min` maximum value
//! * `max` minmum value
//! * `speed`
//! * `power`
//! * `format` (format string in `printf` format)
//! * `catch` override widget label.
//! * `map` Applies a mapping function to `&mut Self` (works the same as in the
//!   [input example](../input/index.html#mapping))
//!
use imgui::sys;
use imgui::{ImStr, Ui};

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

imgui_drag_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32, ), 8, sys::ImGuiDataType_Float }
imgui_drag_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64, ), 8, sys::ImGuiDataType_Double }
imgui_drag_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32, ), 8, sys::ImGuiDataType_U32 }
imgui_drag_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32, ), 8, sys::ImGuiDataType_S32 }

// matrix

imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 8, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 7, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 6, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 5, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 4, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 3, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 2, sys::ImGuiDataType_Float }
imgui_drag_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32), 8, 1, sys::ImGuiDataType_Float }

imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 8, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 7, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 6, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 5, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 4, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 3, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 2, sys::ImGuiDataType_Double }
imgui_drag_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64), 8, 1, sys::ImGuiDataType_Double }

imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 8, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 7, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 6, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 5, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 4, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 3, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 2, sys::ImGuiDataType_U32 }
imgui_drag_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32), 8, 1, sys::ImGuiDataType_U32 }

imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 8, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 7, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 6, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 5, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 4, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 3, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 2, sys::ImGuiDataType_S32 }
imgui_drag_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32), 8, 1, sys::ImGuiDataType_S32 }
