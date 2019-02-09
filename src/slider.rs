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
use imgui::{
    ImStr, SliderFloat, SliderFloat2, SliderFloat3, SliderFloat4, SliderInt, SliderInt2,
    SliderInt3, SliderInt4, Ui,
};

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

impl<T, S: Slider<T>> Slider<T> for Box<S> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<T>) -> bool {
        S::build(ui, elem, params)
    }
}

macro_rules! impl_f32_array {
    ($( $arr:ty => $slider:ident ),*) => {$(
        impl Slider<f32> for $arr {
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) -> bool {
                let mut s = $slider::new(ui, params.label, elem, params.min, params.max);
                if let Some(disp) = params.format { s = s.display_format(disp); }
                if let Some(power) = params.power { s = s.power(power); }
                s.build()
            }
        }
        impl Slider<f32> for Option<$arr> {
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut s = $slider::new(ui, params.label, elem, params.min, params.max);
                    if let Some(disp) = params.format { s = s.display_format(disp); }
                    if let Some(power) = params.power { s = s.power(power); }
                    s.build()
                } else {
                    false
                }
            }
        }
    )*};
}

macro_rules! impl_i32_array {
    ($( $arr:ty => $slider:ident ),*) => {$(
        impl Slider<i32> for $arr {
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) -> bool {
                let mut s = $slider::new(ui, params.label, elem, params.min, params.max);
                if let Some(disp) = params.format { s = s.display_format(disp); }
                s.build()
            }
        }
        impl Slider<i32> for Option<$arr> {
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut s = $slider::new(ui, params.label, elem, params.min, params.max);
                    if let Some(disp) = params.format { s = s.display_format(disp); }
                    s.build()
                } else {
                    false
                }
            }
        }
    )*};
}

impl Slider<f32> for f32 {
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) -> bool {
        let mut s = SliderFloat::new(ui, params.label, elem, params.min, params.max);
        if let Some(disp) = params.format {
            s = s.display_format(disp);
        }
        if let Some(power) = params.power {
            s = s.power(power);
        }
        s.build()
    }
}

impl Slider<i32> for i32 {
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) -> bool {
        let mut s = SliderInt::new(ui, params.label, elem, params.min, params.max);
        if let Some(disp) = params.format {
            s = s.display_format(disp);
        }
        s.build()
    }
}

impl Slider<f32> for Option<f32> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) -> bool {
        if let Some(ref mut elem) = elem {
            f32::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl Slider<i32> for Option<i32> {
    #[inline]
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) -> bool {
        if let Some(ref mut elem) = elem {
            i32::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl_f32_array! {
    [f32; 2] => SliderFloat2,
    [f32; 3] => SliderFloat3,
    [f32; 4] => SliderFloat4
}

impl_i32_array! {
    [i32; 2] => SliderInt2,
    [i32; 3] => SliderInt3,
    [i32; 4] => SliderInt4
}
