//! A crate to quickly build [imgui] GUIs using a `#[derive]` macro.
//!
//! ## Basic usage
//!
//! For a more in-depth example, check out the [`imgui_ext!`] macro.
//!
//! ```ignore
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(slider(min = 0.0, max = 4.0))]
//!     x: f32,
//!     #[imgui(input(step = 2))]
//!     y: i32,
//!     #[imgui(drag(label = "Drag 2D"))]
//!     drag_2d: [f32; 2],
//!
//!     #[imgui(
//!         checkbox(label = "Turbo mode"),
//!         label(label = "Is turbo enabled?"),
//!     )]
//!     turbo: bool,
//! }
//! ```
//!
//! ![ui result][result]
//!
//! ## Nested UIs
//!
//! Types that `#[derive(ImGuiExt)]` can be nested:
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Form {
//!     #[imgui(input)]
//!     user: ImString,
//!     #[imgui(input(flags = "passwd_flags"))]
//!     passwd: ImString,
//! }
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(nested)]
//!     login_form: Form,
//!     #[imgui(checkbox(label = "Remember login?"))]
//!     remember: bool,
//! }
//!
//! ```
//!
//! ![][nested_example]
//!
//! ## Rich compiler errors
//!
//! You get descriptive compiler errors whenever UI is misdefined.
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(slider(min = 0.0))]
//!     foo: f32,
//! }
//! ```
//!
//! ```nonrust
//! error: Parameter `max` missing.
//!   --> example/src/main.rs:10:13
//!    |
//! 10 |     #[imgui(slider(min = 0.0))]
//!    |             ^^^^^^
//! ```
//!
//! ## Combining UI and non-UI fields
//!
//! Not every field needs to be annotated. If a field doesn't have an `#[imgui]` annotation, it
//! will be ignored by the UI.
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Mix {
//!     #[imgui(label = "Some i32")]
//!     in_ui: u32,
//!
//!     // since this field is not annotated, it is ignored by the UI
//!     not_in_ui: Vec<u8>,
//! }
//! ```
//!
//! ## What this crate is not
//!
//! A general purpose imgui library (for all cases anyway).
//!
//! Instead it's meant to compliment [imgui] in order to remove some of the boilerplate. You may also
//! find that the annotations are not flexible enough to produce [highly complex and intricate layouts].
//!
//!
//! [imgui]: https://crates.io/crates/imgui
//! [`imgui_ext!`]: ./macro.imgui_ext.html
//! [result]: https://i.imgur.com/llyqEFY.png
//! [highly complex and intricate layouts]: https://github.com/ocornut/imgui/issues/2265
//! [nested_example]: https://i.imgur.com/Us8bNdE.png
use imgui::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, DragInt, DragInt2, DragInt3, DragInt4, ImString,
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, Ui,
};

pub use checkbox::Checkbox;
use checkbox::CheckboxParams;
pub use drag::Drag;
use drag::DragParams;
pub use input::Input;
use input::InputParams;
pub use slider::Slider;
use slider::SliderParams;

#[doc(hidden)]
pub mod macros;
pub mod prelude {
    pub use imgui_ext_derive::ImGuiExt;

    pub use super::{Checkbox, Drag, Input, Slider};
}

/// `slider(...)` docs.
pub mod slider {
    #[derive(Copy, Clone)]
    pub struct SliderParams<'ui, T> {
        pub min: T,
        pub max: T,
        pub label: &'ui imgui::ImStr,
        pub format: Option<&'ui imgui::ImStr>,
        pub power: Option<f32>,
    }
    pub trait Slider<T> {
        fn build(ui: &imgui::Ui, elem: &mut Self, params: SliderParams<T>);
    }
}
/// `input(...)` docs.
pub mod input {
    //!
    //! The input trait is implemented for numeric types (`f32` and `i32`) and their corresponding
    //! array types of up to 4 elements, and [`ImString`]
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Example {
    //!     // parameters in input() are all optional
    //!     #[imgui(input)]
    //!     input_0: f32,
    //!
    //!     // `precision = ..` specifies the decimal precision.
    //!     // This parameter only has an effect in f32 types.
    //!     #[imgui(input(precision = 2))]
    //!     input_1: [f32; 2],
    //!
    //!     // `step` and `step_fast`
    //!     #[imgui(input(step = 4, step_fast = 42))]
    //!     input_2: i32,
    //! }
    //! ```
    //!
    //! ### Result
    //!
    //! ![result][result]
    //!
    //! ## Custom input flags
    //!
    //! You can specify a local function from where to load any input flags.
    //!
    //! The only is that these flags cannot be changed at runtime.
    //!
    //! ```ignore
    //! use imgui::ImGuiInputTextFlags;
    //!
    //! #[derive(ImGuiExt)]
    //! struct Example {
    //!     #[imgui(input(flags = "my_flags"))]
    //!     input_2: i32,
    //! }
    //!
    //! fn my_flags() -> ImGuiInputTextFlags {
    //!     ImGuiInputTextFlags::Password
    //! }
    //! ```
    //!
    //! [`ImString`]: #
    //! [result]: https://i.imgur.com/BPvMGAp.png
    #[derive(Copy, Clone)]
    pub struct InputParams<'ui, T> {
        pub label: &'ui imgui::ImStr,
        pub precision: Option<i32>,
        pub step: Option<T>,
        pub step_fast: Option<T>,
        pub flags: Option<imgui::ImGuiInputTextFlags>,
    }
    pub trait Input<T> {
        fn build(ui: &imgui::Ui, elem: &mut Self, params: InputParams<T>);
    }
}
/// `drag(...)` docs.
pub mod drag {
    #[derive(Copy, Clone)]
    pub struct DragParams<'ui, T> {
        pub label: &'ui imgui::ImStr,
        pub format: Option<&'ui imgui::ImStr>,
        pub min: Option<T>,
        pub max: Option<T>,
        pub speed: Option<f32>,
        pub power: Option<f32>,
    }
    pub trait Drag<T> {
        fn build(ui: &imgui::Ui, elem: &mut Self, params: DragParams<T>);
    }
}
/// `checkbox(...)` docs.
pub mod checkbox {
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Checkboxes {
    //!     // All parameters are optional.
    //!     #[imgui(checkbox)]
    //!     turbo: bool,
    //!
    //!     // Optionally, you can override the label:
    //!     #[imgui(checkbox(label = "Checkbox!"))]
    //!     check: bool,
    //! }
    //! ```
    //!
    //! ### Result
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/1hTR89V.png

    /// Structure generated by the annoration.
    #[derive(Copy, Clone)]
    pub struct CheckboxParams<'ui> {
        pub label: &'ui imgui::ImStr,
    }

    /// Trait for types that can be represented with a checkbox.
    pub trait Checkbox {
        fn build(ui: &imgui::Ui, elem: &mut Self, params: CheckboxParams);
    }
}
/// Support for some (basic) layout annotations.
pub mod layout {}
/// `label(...)` docs.
pub mod label {}
/// `nested(...)` docs (used to build nested UIs).
pub mod nested {}
/// `button(...)` docs.
pub mod button {}

#[doc(hidden)]
pub trait ImGuiExt {
    fn imgui_ext(ui: &Ui, ext: &mut Self);
}

impl Checkbox for bool {
    fn build(ui: &Ui, elem: &mut Self, params: CheckboxParams) {
        ui.checkbox(params.label, elem);
    }
}

impl Input<f32> for ImString {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) {
        let mut text = InputText::new(ui, params.label, elem);
        if let Some(flags) = params.flags {
            text = text.flags(flags);
        }
        text.build();
    }
}

macro_rules! impl_slider {
    ( $( $t:ty , f32 => $fun:ident , )+ ) => {$(
        impl Slider<f32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.format { s = s.display_format(disp); }
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
                if let Some(disp) = params.format { s = s.display_format(disp); }
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
                if let Some(value) = params.precision { input = input.decimal_precision(value) }
                if let Some(value) = params.flags { input = input.flags(value) }
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
                if let Some(value) = params.precision { input = input.decimal_precision(value) }
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
                let input = $fun::new(ui, params.label, elem);
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
                if let Some(disp) = params.format { drag = drag.display_format(disp); }
                drag.build();
            }
        }
    )+};

    ( $( $t:ty , i32 => $fun:ident , )+ ) => {$(
        impl Drag<i32> for $t {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<i32>) {
                let mut drag = $fun::new(ui, params.label, elem);
                if let Some(val) = params.max { drag = drag.max(val); }
                if let Some(val) = params.min { drag = drag.min(val); }
                if let Some(val) = params.speed { drag = drag.speed(val); }
                if let Some(disp) = params.format { drag = drag.display_format(disp); }
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

impl_drag! {
    i32, i32 => DragInt,
    [i32; 2] , i32 => DragInt2,
    [i32; 3] , i32 => DragInt3,
    [i32; 4] , i32 => DragInt4,
}
