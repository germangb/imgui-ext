//!
//! (Text input has its own module: [text])
//!
//! [text]: ../text/index.html
//!
//! ## Optional fields
//!
//! * `label`
//! * `precision` Decimal precision.
//! * `step`
//! * `step_fast`
//! * `flags` Name of a local function that returns the input [flags].
//! * `catch`
//!
//! [flags]: https://docs.rs/imgui/0.0/imgui/struct.ImGuiInputTextFlags.html
//!
//! ## Example
//!
//! The input trait is implemented for numeric types (`f32` and `i32`) and their corresponding
//! array types of up to 4 elements.
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(input)]
//!     input_0: f32,
//!
//!     #[imgui(input(precision = 2))]
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
//! You can specify a local function from where to load any input flags:
//!
//! ```
//! use imgui_ext::ImGuiExt;
//! use imgui::ImGuiInputTextFlags;
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
use imgui::{
    ImGuiInputTextFlags, ImStr, InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt,
    InputInt2, InputInt3, InputInt4, Ui,
};

#[derive(Copy, Clone)]
pub struct InputParams<'ui, T> {
    pub label: &'ui ImStr,
    pub precision: Option<i32>,
    pub step: Option<T>,
    pub step_fast: Option<T>,
    pub flags: Option<ImGuiInputTextFlags>,
}

pub trait Input<T> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<T>) -> bool;
}

macro_rules! impl_f32_array {
    ( $($arr:ty => $input:ident),* ) => {$(
        impl Input<f32> for $arr {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
                let mut input = $input::new(ui, params.label, elem);
                if let Some(value) = params.precision { input = input.decimal_precision(value) }
                if let Some(value) = params.flags { input = input.flags(value) }
                input.build()
            }
        }

        impl Input<f32> for Option<$arr> {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut input = $input::new(ui, params.label, elem);
                    if let Some(value) = params.precision { input = input.decimal_precision(value) }
                    if let Some(value) = params.flags { input = input.flags(value) }
                    input.build()
                } else {
                    false
                }
            }
        }
    )*}
}

macro_rules! impl_i32_array {
    ( $($arr:ty => $input:ident),* ) => {$(
        impl Input<i32> for $arr {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) -> bool {
                let mut input = $input::new(ui, params.label, elem);
                if let Some(value) = params.flags { input = input.flags(value) }
                input.build()
            }
        }

        impl Input<i32> for Option<$arr> {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) -> bool {
                if let Some(ref mut elem) = elem {
                    let mut input = $input::new(ui, params.label, elem);
                    if let Some(value) = params.flags { input = input.flags(value) }
                    input.build()
                } else {
                    false
                }
            }
        }
    )*}
}

impl Input<f32> for f32 {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
        let mut input = InputFloat::new(ui, params.label, elem);
        if let Some(value) = params.step {
            input = input.step(value)
        }
        if let Some(value) = params.step_fast {
            input = input.step_fast(value)
        }
        if let Some(value) = params.precision {
            input = input.decimal_precision(value)
        }
        if let Some(value) = params.flags {
            input = input.flags(value)
        }
        input.build()
    }
}

impl Input<f32> for Option<f32> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<f32>) -> bool {
        if let Some(ref mut elem) = elem {
            f32::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl Input<i32> for i32 {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) -> bool {
        let mut input = InputInt::new(ui, params.label, elem);
        if let Some(value) = params.step {
            input = input.step(value)
        }
        if let Some(value) = params.step_fast {
            input = input.step_fast(value)
        }
        if let Some(value) = params.flags {
            input = input.flags(value)
        }
        input.build()
    }
}

impl Input<i32> for Option<i32> {
    fn build(ui: &Ui, elem: &mut Self, params: InputParams<i32>) -> bool {
        if let Some(ref mut elem) = elem {
            i32::build(ui, elem, params)
        } else {
            false
        }
    }
}

impl_f32_array! {
    [f32; 2] => InputFloat2,
    [f32; 3] => InputFloat3,
    [f32; 4] => InputFloat4
}

impl_i32_array! {
    [i32; 2] => InputInt2,
    [i32; 3] => InputInt3,
    [i32; 4] => InputInt4
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate as imgui_ext;
    use crate::ImGuiExt;
    use imgui::ImGuiInputTextFlags as Flags;

    fn flags() -> Flags {
        Flags::Password
    }

    #[test]
    fn input_text() {
        #[derive(ImGuiExt)]
        struct Foo {
            #[imgui(input)]
            a: i32,
        }
    }

    #[test]
    fn input_f32() {
        #[derive(ImGuiExt)]
        struct Foo {
            #[imgui(input)]
            a: f32,
            #[imgui(input())]
            b: [f32; 2],
            #[imgui(input(flags = "flags"))]
            c: [f32; 3],
            #[imgui(input(step = 0.1, step_fast = 10.0, flags = "flags"))]
            d: f32,
            #[imgui(
                input(step = 0.1, step_fast = 2.0, flags = "flags"),
                input(step = 0.1, step_fast = 2.0, flags = "flags"),
                input(step = 0.1, step_fast = 2.0)
            )]
            e: f32,
        }
    }

    #[test]
    fn input_i32() {
        #[derive(ImGuiExt)]
        struct Foo {
            #[imgui(input)]
            a: i32,
            #[imgui(input())]
            b: [i32; 2],
            #[imgui(input(flags = "flags"))]
            c: [i32; 3],
            #[imgui(input(step = 0, step_fast = 10, flags = "flags"))]
            d: i32,
            #[imgui(
                input(step = 0, step_fast = 2, flags = "flags"),
                input(step = 0, step_fast = 2, flags = "flags"),
                input(step = 0, step_fast = 2)
            )]
            e: i32,

            _ignore: Vec<u8>,
        }
    }
}
