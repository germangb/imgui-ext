//!
//! [text]: ../text/index.html
//!
//! ## Optional fields
//!
//! * `label`
//! * `step`
//! * `step_fast`
//! * `flags` Name of a local function that returns the input [flags].
//! * `size` Text box size (Applies to text input)
//! * `catch`
//! * `map` Applies a mapping function to `&mut Self` (see [example](#mapping)).
//!
//! [flags]: https://docs.rs/imgui/0.0/imgui/struct.ImGuiInputTextFlags.html
//!
//! ## Example
//!
//! The input trait is implemented for numeric types (`f32` and `i32`) and their
//! corresponding array types of up to 8 elements.
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(input)]
//!     input_0: f32,
//!
//!     #[imgui(input)]
//!     input_1: [f32; 2],
//!
//!     #[imgui(input(step = 4, step_fast = 42))]
//!     input_2: i32,
//! }
//! ```
//!
//! ### Result
//!
//! ![result][result]
//!
//! ## Input flags
//!
//! You can load input flags from a local function:
//!
//! ```
//! use imgui::ImGuiInputTextFlags;
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(input(flags = "my_flags"))]
//!     n: f32,
//! }
//!
//! fn my_flags() -> ImGuiInputTextFlags {
//!     ImGuiInputTextFlags::Password
//! }
//! ```
//!
//! [result]: https://i.imgur.com/BPvMGAp.png
//!
//! # Mapping
//!
//! The attribute `map` points to a local function that performs a map operation
//! on the attribute:
//!
//! ```
//! // Note: Foo doesn't implement the ImGuiExt macro
//! struct Foo {
//!     inner: [f32; 4],
//! }
//!
//! #[derive(imgui_ext::Ui)]
//! struct Bar {
//!     // Even though Foo is not compatible with the input()
//!     // annotation, its inner attribute does, therefore we
//!     // can map from one type to the other.
//!     #[imgui(input(map = "foo_to_array"))]
//!     foo: Foo,
//! }
//!
//! fn foo_to_array(foo: &mut Foo) -> &mut [f32; 4] {
//!     &mut foo.inner
//! }
//! ```
use imgui::sys;
use imgui::{ImGuiInputTextFlags, ImStr, ImString, ImVec2, InputText, InputTextMultiline, Ui};

#[derive(Copy, Clone)]
pub struct InputParams<'a, T> {
    pub label: &'a ImStr,
    pub step: Option<T>,
    pub step_fast: Option<T>,
    pub flags: Option<ImGuiInputTextFlags>,
    pub size: Option<ImVec2>,
}

pub trait Input<T> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>) -> bool;
}

impl<T, I: Input<T>> Input<T> for Box<I> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>) -> bool {
        I::build(ui, elem, params)
    }
}

impl<T, I: Input<T>> Input<T> for Option<I> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>) -> bool {
        if let Some(ref mut elem) = elem {
            I::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl Input<()> for ImString {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<()>) -> bool {
        if let Some(size) = params.size {
            let mut input = InputTextMultiline::new(ui, params.label, elem, size);
            if let Some(flags) = params.flags {
                input = input.flags(flags);
            }
            input.build()
        } else {
            let mut input = InputText::new(ui, params.label, elem);
            if let Some(flags) = params.flags {
                input = input.flags(flags);
            }
            input.build()
        }
    }
}

imgui_input_scalar! { (f32, f32, f32, f32, f32, f32, f32, f32, ), 8, sys::ImGuiDataType::Float }
imgui_input_scalar! { (f64, f64, f64, f64, f64, f64, f64, f64, ), 8, sys::ImGuiDataType::Double }
imgui_input_scalar! { (u32, u32, u32, u32, u32, u32, u32, u32, ), 8, sys::ImGuiDataType::U32 }
imgui_input_scalar! { (i32, i32, i32, i32, i32, i32, i32, i32, ), 8, sys::ImGuiDataType::S32 }

// matrix types

imgui_input_matrix! { (f32, f32, f32, f32, f32, f32, f32, f32 ), 8, sys::ImGuiDataType::Float }
imgui_input_matrix! { (f64, f64, f64, f64, f64, f64, f64, f64 ), 8, sys::ImGuiDataType::Double }
imgui_input_matrix! { (u32, u32, u32, u32, u32, u32, u32, u32 ), 8, sys::ImGuiDataType::U32 }
imgui_input_matrix! { (i32, i32, i32, i32, i32, i32, i32, i32 ), 8, sys::ImGuiDataType::S32 }
