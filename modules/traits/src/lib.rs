use imgui::{DragFloat, DragFloat2, DragFloat3, DragFloat4, ImStr, ImString, InputFloat, InputFloat2, InputFloat3, InputFloat4, Ui};

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
#[derive(Copy, Clone)]
pub struct SliderParams<'ui> {
    pub min: f32,
    pub max: f32,
    pub label: &'ui ImStr,
    pub display: Option<&'ui ImStr>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct InputParams<'ui> {
    pub label: &'ui ImStr,
    pub precission: Option<i32>,

    // fields ignored for multidimensional input
    pub step: Option<f32>,
    pub step_fast: Option<f32>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct DragParams<'ui> {
    pub label: &'ui ImStr,
    pub display: Option<&'ui ImStr>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub speed: Option<f32>,
    pub power: Option<f32>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct SimpleParams<'ui> {
    pub label: &'ui ImStr,
}

#[doc(hidden)]
pub trait Slider {
    fn build(ui: &Ui, elem: &mut Self, params: SliderParams);
}

#[doc(hidden)]
pub trait Input {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams);
}

#[doc(hidden)]
pub trait Drag {
    fn build(ui: &Ui, elem: &mut Self, params: DragParams);
}

/// Trait for types that can be tagged with a simple `#[imgui]`
///
/// Examples:
///   - `bool`
#[doc(hidden)]
pub trait Simple {
    fn build(ui: &Ui, elem: &mut Self, params: SimpleParams);
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
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Slider for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams) {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.display { s = s.display_format(disp); }
                s.build();
            }
        })+
    }
}

macro_rules! impl_input {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams) {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.step { input = input.step(value) }
                if let Some(value) = params.step_fast { input = input.step_fast(value) }
                if let Some(value) = params.precission { input = input.decimal_precision(value) }
                input.build();
            }
        })+
    }
}

macro_rules! impl_input_d {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams) {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.precission { input = input.decimal_precision(value) }
                input.build();
            }
        })+
    }
}

impl_slider! {
    f32 => slider_float,
    [f32; 2] => slider_float2,
    [f32; 3] => slider_float3,
    [f32; 4] => slider_float4,
}

impl_input! {
    f32 => InputFloat,
}

impl_input_d! {
    [f32; 2] => InputFloat2,
    [f32; 3] => InputFloat3,
    [f32; 4] => InputFloat4,
}

impl Drag for f32 {
    fn build(ui: &Ui, elem: &mut Self, params: DragParams) {
        let mut drag = DragFloat::new(ui, params.label, elem);
        if let Some(val) = params.max { drag = drag.max(val); }
        if let Some(val) = params.min { drag = drag.min(val); }
        if let Some(val) = params.speed { drag = drag.speed(val); }
        if let Some(val) = params.power { drag = drag.power(val); }
        drag.build();
    }
}
