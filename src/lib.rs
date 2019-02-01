//! A crate to quickly build [imgui] GUIs using a `#[derive]` macro.
//!
//! [imgui]: https://crates.io/crates/imgui
//!
//! ## Basic usage
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
//! ## Static
//!
//! The generated code won't contain extra dynamic allocations.
//!
//! You can see what the generated code looks like in [this example].
//!
//! [this example]: #
//!
//! ## Handle button presses and other inputs
//!
//! Most annotations can take an optional `catch = "..."` parameter that can be used to identify
//! when a given button is pressed or an input changes later on:
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(checkbox(catch = "check"))]
//!     input_check: bool,
//! }
//!
//! let events = imgui_ext(ui, &mut example);
//! if events.check {
//!     println!("New value: {}", example.input_check);
//! }
//! ```
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
//! ## Descriptive errors
//!
//! The correctness of the UI definition is checked at compile time. Thus, if something is misdefined,
//! the compiler will raise an error.
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
//! If a field is not annotated, it will be ignored in the UI.
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
//! [result]: https://i.imgur.com/llyqEFY.png
//! [nested_example]: https://i.imgur.com/Us8bNdE.png
use imgui::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, DragInt, DragInt2, DragInt3, DragInt4, ImString,
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, Ui,
};

use checkbox::CheckboxParams;
use drag::DragParams;
pub use imgui_ext_derive::ImGuiExt;
use input::InputParams;
use slider::SliderParams;

#[doc(hidden)]
pub mod macros;
pub mod prelude {
    pub use super::checkbox::Checkbox;
    pub use super::drag::Drag;
    pub use super::input::Input;
    pub use super::slider::Slider;
    pub use super::{ImGuiExt, UiExt};
}

/// `checkbox(...)` docs.
pub mod checkbox;
/// `drag(...)` docs.
pub mod drag;
/// `input(...)` docs.
pub mod input;
/// `slider(...)` docs.
pub mod slider;
/// Support for some (basic) layout annotations.
pub mod layout {}
/// `label(...)` docs.
pub mod label {
    //!
    //! `label(...)` is used to display the contents of a field:
    //!
    //! It has two optional fields:
    //!
    //! * `label = "..."` to override the label title.
    //! * `display = "..."` to format text.
    //!
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Labels {
    //!     #[imgui(label)]
    //!     foo: f32,
    //!
    //!     // Use inner fields to format the text.
    //!     #[imgui(label(label = "Tuple", display = "({}, {}, {})", 0, 1, 2))]
    //!     bar: (f32, bool, usize),
    //!
    //!     // if label() is the only annotation, you can avoid writting the "label()" part:
    //!     #[imgui(label = "String param")]
    //!     baz: String,
    //! }
    //! ```
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/Wf4Uze7.png
}
/// `nested(...)` docs (used to build nested UIs).
pub mod nested {}
/// `button(...)` docs.
pub mod button {
    //!
    //! `button(...)` is not associated to any particular type, but its position within an annotation
    //! will determine where it is rendered in the final UI.
    //!
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Buttons {
    //!     #[imgui(
    //!         button(size = "btn_size", label = "Click me!", catch = "click"),
    //!         separator,
    //!         label(label = "Clicks"),
    //!     )]
    //!     count: i32,
    //! }
    //!
    //! fn btn_size() -> (f32, f32) {
    //!     (100.0, 20.0)
    //! }
    //!
    //! // initialize ui and Buttons...
    //! let events = ui.imgui_ext(&mut buttons);
    //! if events.click {
    //!     buttons.count += 1;
    //! }
    //!
    //! ```
    //!
    //! ![][image]
    //!
    //! [image]: https://i.imgur.com/PpOcZK8.png
}
/// `bullet(...)` docs.
pub mod bullet {
    //!
    //! `bullet(...)` is used to define bullet lists. It has two variants:
    //!
    //! * `bullet(text = "...")` defines a bullet'd text.
    //! * `bullet(...)` bullets whatever is inside of ...
    //!
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Bullet {
    //!     #[imgui(
    //!         bullet(text = "Kill all humans"),
    //!         bullet(slider(min = 0.0, max = 1.0)),
    //!     )]
    //!     foo: f32,
    //! }
    //! ```
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/pe4YstR.png
}

/// Trait implemented by the derive macro.
pub trait ImGuiExt {
    type Events;
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events;
}

/// Extension trait for an imgui Ui.
pub trait UiExt<'ui> {
    fn imgui_ext<U: ImGuiExt>(&'ui self, ext: &mut U) -> U::Events;
}

impl<'ui> UiExt<'ui> for Ui<'ui> {
    #[inline]
    fn imgui_ext<U: ImGuiExt>(&'ui self, ext: &mut U) -> U::Events {
        imgui_ext(self, ext)
    }
}

/// Render imgui UI and collect all the events
///
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct Example {
///     #[derive(checkbox(catch = "click"))]
///     check_box: bool,
/// }
///
/// // initialize imgui and example...
///
/// let events = imgui_ext(ui, &mut example);
/// if events.click {
///     println!("New value: {}", example.check_box);
/// }
/// ```
///
/// Optionally you can call an extension method on the ui directly by using the [`UiExt`] trait.
///
/// [`UiExt`]: #
///
/// ```ignore
/// // imports the UiExt trait
/// use imgui_ext::prelude::*;
///
/// // initialize ui and example...
///
/// let events = ui.imgui_ext(&mut example);
/// if events.click {
///     // ...
/// }
#[inline]
pub fn imgui_ext<U: ImGuiExt>(ui: &Ui, ext: &mut U) -> U::Events {
    U::imgui_ext(ui, ext)
}

use drag::Drag;
use input::Input;
use slider::Slider;

impl Input<f32> for ImString {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
        let mut text = InputText::new(ui, params.label, elem);
        if let Some(flags) = params.flags {
            text = text.flags(flags);
        }
        text.build()
    }
}

macro_rules! impl_slider {
    ( $( $t:ty , f32 => $fun:ident , )+ ) => {$(
        impl Slider<f32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<f32>) -> bool {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.format { s = s.display_format(disp); }
                if let Some(power) = params.power { s = s.power(power); }
                s.build()
            }
        })+
    };
    ( $( $t:ty , i32 => $fun:ident , )+ ) => {$(
        impl Slider<i32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<i32>) -> bool {
                let mut s = ui.$fun(params.label, elem, params.min, params.max);
                if let Some(disp) = params.format { s = s.display_format(disp); }
                s.build()
            }
        })+
    }
}

macro_rules! impl_input {
    ($( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<$t> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$t>) -> bool {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.step { input = input.step(value) }
                if let Some(value) = params.step_fast { input = input.step_fast(value) }
                if let Some(value) = params.precision { input = input.decimal_precision(value) }
                if let Some(value) = params.flags { input = input.flags(value) }
                input.build()
            }
        })+
    }
}

macro_rules! impl_input_i32 {
    ($( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<$t> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$t>) -> bool {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.step { input = input.step(value) }
                if let Some(value) = params.step_fast { input = input.step_fast(value) }
                input.build()
            }
        })+
    }
}

macro_rules! impl_input_d {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<f32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
                let mut input = $fun::new(ui, params.label, elem);
                if let Some(value) = params.precision { input = input.decimal_precision(value) }
                input.build()
            }
        })+
    }
}

macro_rules! impl_input_i32_d {
    ( $( $t:ty => $fun:ident , )+ ) => {$(
        impl Input<i32> for $t {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) -> bool {
                let input = $fun::new(ui, params.label, elem);
                input.build()
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
