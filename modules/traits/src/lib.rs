use params::*;

use imgui::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, ImStr, ImString, InputFloat, InputFloat2,
    InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4, Ui, InputText,
};

pub mod params;

/// An extension trait for [`Ui`] that adds support for [`ImGuiExt`]
///
/// [`Ui`]: #
/// [`ImGuiExt`]: #
pub trait UiExt {
    fn ui_ext<U: ImGuiExt>(&self, ui: &mut U);
}

impl<'ui> UiExt for Ui<'ui> {
    fn ui_ext<U: ImGuiExt>(&self, ui: &mut U) {
        U::imgui_ext(self, ui);
    }
}

/// A trait to implement custom imgui UIs on user structs.
///
/// This trait is meant to be used through a `#[derive(ImGuiExt)]`
///
/// # Example
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct MyStruct {
///     #[imgui(label = "Foo variable")]
///     foo: f32,
/// }
/// ```
pub trait ImGuiExt {
    fn imgui_ext(ui: &Ui, ext: &mut Self);
}

#[doc(hidden)]
pub trait Slider<T> {
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams<T>);
}

#[doc(hidden)]
pub trait Input<T> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>);
}

#[doc(hidden)]
pub trait Drag<T> {
    fn build(ui: &Ui, elem: &mut Self, params: DragParams<T>);
}

#[doc(hidden)]
pub trait Simple {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams);
}

#[doc(hidden)]
pub trait Text {
    fn build(ui: &Ui, elem: &mut Self, params: TextParams);
}

#[doc(hidden)]
pub trait Checkbox {
    fn build(ui: &Ui, elem: &mut Self, params: CheckboxParams);
}

impl Checkbox for bool {
    fn build(ui: &Ui, elem: &mut Self, params: CheckboxParams) {
        ui.checkbox(params.label, elem);
    }
}

impl Text for ImString {
    fn build(ui: &Ui, elem: &mut Self, params: TextParams) {
        InputText::new(ui, params.label, elem).build();
    }
}

impl Simple for bool {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams) {
        ui.checkbox(params.label, elem);
    }
}

impl<'a> Simple for &'a str {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams) {
        ui.label_text(params.label, &ImString::new(*elem));
    }
}

impl Simple for String {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams) {
        ui.label_text(params.label, &ImString::new(&elem[..]));
    }
}

impl<'a> Simple for &'a ImStr {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams) {
        ui.label_text(params.label, *elem);
    }
}

macro_rules! impl_slider {
    ( $( $t:ty , f32 => $fun:ident , )+ ) => {$(
        impl Slider<f32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.display { s = s.display_format(disp); }
                if let Some(power) = params.power { s = s.power(power); }
                s.build();
            }
        })+
    };
    ( $( $t:ty , i32 => $fun:ident , )+ ) => {$(
        impl Slider<i32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.display { s = s.display_format(disp); }
                s.build();
            }
        })+
    }
}

macro_rules! impl_input {
    ($( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<$t> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$t>) {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.step { input = input.step(value) }
                if let Some(value) = params.step_fast { input = input.step_fast(value) }
                if let Some(value) = params.precission { input = input.decimal_precision(value) }
                input.build();
            }
        })+
    }
}

macro_rules! impl_input_i32 {
    ($( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<$t> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$t>) {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.step { input = input.step(value) }
                if let Some(value) = params.step_fast { input = input.step_fast(value) }
                input.build();
            }
        })+
    }
}

macro_rules! impl_input_d {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<f32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.precission { input = input.decimal_precision(value) }
                input.build();
            }
        })+
    }
}

macro_rules! impl_input_i32_d {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<i32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) {
                let mut input = $fun::new(ui, params.label, elem);
                input.build();
            }
        })+
    }
}

macro_rules! impl_drag {
    ( $( $t:ty , f32 => $fun:ident , )+ ) => {$(
        impl Drag<f32> for $t {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<f32>) {
                let mut drag = $fun::new(ui, params.label, elem);
                if let Some(val) = params.max { drag = drag.max(val); }
                if let Some(val) = params.min { drag = drag.min(val); }
                if let Some(val) = params.speed { drag = drag.speed(val); }
                if let Some(val) = params.power { drag = drag.power(val); }
                drag.build();
            }
        }
    )+}
}

impl_slider! {
    f32 , f32 => slider_float,
    [f32; 2] , f32 => slider_float2,
    [f32; 3] , f32 => slider_float3,
    [f32; 4] , f32 => slider_float4,
}

impl_slider! {
    i32 , i32 => slider_int,
    [i32; 2] , i32 => slider_int2,
    [i32; 3] , i32 => slider_int3,
    [i32; 4] , i32 => slider_int4,
}

impl_input! {
    f32 => InputFloat,
}

impl_input_i32! {
    i32 => InputInt,
}

impl_input_d! {
    [f32; 2] => InputFloat2,
    [f32; 3] => InputFloat3,
    [f32; 4] => InputFloat4,
}

impl_input_i32_d! {
    [i32; 2] => InputInt2,
    [i32; 3] => InputInt3,
    [i32; 4] => InputInt4,
}

impl_drag! {
    f32, f32 => DragFloat,
    [f32; 2] , f32 => DragFloat2,
    [f32; 3] , f32 => DragFloat3,
    [f32; 4] , f32 => DragFloat4,
}
