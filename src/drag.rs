//!
//! ## Optional params
//!
//! * `min`
//! * `max`
//! * `speed`
//! * `power`
//! * `format` (printf format)
//! * `catch`
use imgui::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, DragInt, DragInt2, DragInt3, DragInt4, ImStr, Ui,
};

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

impl<T, D: Drag<T>> Drag<T> for Box<D> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: DragParams<T>) -> bool {
        D::build(ui, elem, params)
    }
}

macro_rules! impl_drag {
    ( $( $t:ty , f32 => $fun:ident , )+ ) => {$(
        impl Drag<f32> for $t {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<f32>) -> bool {
                let mut drag = $fun::new(ui, params.label, elem);
                if let Some(val) = params.max { drag = drag.max(val); }
                if let Some(val) = params.min { drag = drag.min(val); }
                if let Some(val) = params.speed { drag = drag.speed(val); }
                if let Some(val) = params.power { drag = drag.power(val); }
                if let Some(disp) = params.format { drag = drag.display_format(disp); }
                drag.build()
            }
        }
        impl Drag<f32> for Option<$t> {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<f32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut drag = $fun::new(ui, params.label, elem);
                    if let Some(val) = params.max { drag = drag.max(val); }
                    if let Some(val) = params.min { drag = drag.min(val); }
                    if let Some(val) = params.speed { drag = drag.speed(val); }
                    if let Some(val) = params.power { drag = drag.power(val); }
                    if let Some(disp) = params.format { drag = drag.display_format(disp); }
                    drag.build()
                } else {
                    false
                }
            }
        }
    )+};

    ( $( $t:ty , i32 => $fun:ident , )+ ) => {$(
        impl Drag<i32> for $t {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<i32>) -> bool {
                let mut drag = $fun::new(ui, params.label, elem);
                if let Some(val) = params.max { drag = drag.max(val); }
                if let Some(val) = params.min { drag = drag.min(val); }
                if let Some(val) = params.speed { drag = drag.speed(val); }
                if let Some(disp) = params.format { drag = drag.display_format(disp); }
                drag.build()
            }
        }
        impl Drag<i32> for Option<$t> {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<i32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut drag = $fun::new(ui, params.label, elem);
                    if let Some(val) = params.max { drag = drag.max(val); }
                    if let Some(val) = params.min { drag = drag.min(val); }
                    if let Some(val) = params.speed { drag = drag.speed(val); }
                    if let Some(disp) = params.format { drag = drag.display_format(disp); }
                    drag.build()
                } else {
                    false
                }
            }
        }
    )+}
}

impl_drag! {
    f32, f32 => DragFloat,
    [f32; 2] , f32 => DragFloat2,
    [f32; 3] , f32 => DragFloat3,
    [f32; 4] , f32 => DragFloat4,
}

impl_drag! {
    i32, i32 => DragInt,
    [i32; 2] , i32 => DragInt2,
    [i32; 3] , i32 => DragInt3,
    [i32; 4] , i32 => DragInt4,
}
