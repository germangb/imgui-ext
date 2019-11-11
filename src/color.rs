//!
//! It has three variants:
//!
//! ## Color Button
//!
//! `color(button(...))`
//!
//! ### Optional params
//!
//! * `label`
//! * `flags` Function identifier that returns a
//!   [`ColorEditFlags`][ColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `size` Function identifier that returns the button size.
//! * `catch`
//! * `map` Applies a mapping function to `&mut Self`.
//!
//! ## Color Edit
//!
//! `color(edit(...))`
//!
//! ### Optional params
//!
//! * `label`
//! * `flags` Function identifier that returns a
//!   [`ColorEditFlags`][ColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `mode` Allowed values: `"RGB"`, `"HSV"`, `"HEX"`
//!   ([`ColorEditMode`][ColorEditMode] variants).
//! * `format` Allowed values: `"Float"`, `"U8"` ([`ColorFormat`][ColorFormat]
//!   variants).
//! * `catch`
//! * `map` Applies a mapping function to `&mut Self`.
//!
//! ## Color Picker
//!
//! `color(picker(...))`
//!
//! ### Optional params
//!
//! * `label`
//! * `flags` Function identifier that returns a
//!   [`ColorEditFlags`][ColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `mode` Allowed values: `"HueBar"`, `"HueWheel"`
//!   ([`ColorPickerMode`][ColorEditMode] variants).
//! * `format` Allowed values: `"Float"`, `"U8"` ([`ColorFormat`][ColorFormat]
//!   variants).
//! * `catch`
//! * `map` Applies a mapping function to `&mut Self`.
//!
//! ## Example
//!
//! ```
//! #[derive(imgui_ext::Gui)]
//! struct Example {
//!     // you could also nest all the modes inside of the same `color(...)`
//!     #[imgui(
//!         color(button(preview = "Alpha")),
//!         color(edit(preview = "HalfAlpha")),
//!         color(picker(mode = "HueWheel"))
//!     )]
//!     color: [f32; 4],
//! }
//! ```
//!
//! ### Result:
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/hWD08K0.png?1
//! [ColorEditFlags]: https://docs.rs/imgui/0.0/imgui/struct.ColorEditFlags.html
//! [ColorPreview]: https://docs.rs/imgui/0.0/imgui/enum.ColorPreview.html
//! [ColorFormat]: https://docs.rs/imgui/0.0/imgui/enum.ColorFormat.html
//! [ColorEditMode]: https://docs.rs/imgui/0.0/imgui/enum.ColorEditMode.html
//! [ColorPickerMode]: https://docs.rs/imgui/0.0/imgui/enum.ColorPickerMode.html
use imgui::{
    ColorButton as ImColorButton, ColorEdit as ImColorEdit, ColorEditInputMode, ColorEditDisplayMode, ColorFormat,
    ColorPicker as ImColorPicker, ColorPickerMode, ColorPreview, EditableColor,
    ColorEditFlags, ImStr, Ui,
};

pub struct ColorButtonParams<'a> {
    pub label: &'a ImStr,
    pub flags: Option<ColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub input_mode: Option<ColorEditInputMode>,
    pub size: Option<[f32; 2]>,
}

pub struct ColorEditParams<'a> {
    pub label: &'a ImStr,
    pub flags: Option<ColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub format: Option<ColorFormat>,
    pub input_mode: Option<ColorEditInputMode>,
    pub display_mode: Option<ColorEditDisplayMode>,
}

pub struct ColorPickerParams<'a> {
    pub label: &'a ImStr,
    pub flags: Option<ColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub format: Option<ColorFormat>,
    pub mode: Option<ColorPickerMode>,
    pub input_mode: Option<ColorPickerMode>,
}

pub trait ColorButton {
    fn build(ui: &Ui, elem: Self, params: ColorButtonParams) -> bool;
}

pub trait ColorEdit {
    fn build(ui: &Ui, elem: Self, params: ColorEditParams) -> bool;
}

pub trait ColorPicker {
    fn build(ui: &Ui, elem: Self, params: ColorPickerParams) -> bool;
}

impl<C: Into<[f32; 4]>> ColorButton for C {
    fn build(ui: &Ui, elem: Self, params: ColorButtonParams) -> bool {
        let mut button = ImColorButton::new(params.label, elem.into());
        if let Some(flags) = params.flags {
            button = button.flags(flags);
        }
        if let Some(preview) = params.preview {
            button = button.preview(preview);
        }
        if let Some(input_mode) = params.input_mode {
            button = button.input_mode(input_mode);
        }
        if let Some(size) = params.size {
            button = button.size(size);
        }
        button.build(ui)
    }
}

impl<'a, C: Into<EditableColor<'a>>> ColorEdit for C {
    fn build(ui: &Ui, elem: Self, params: ColorEditParams) -> bool {
        let mut edit = ImColorEdit::new(params.label, elem.into());
        if let Some(flags) = params.flags {
            edit = edit.flags(flags);
        }
        if let Some(preview) = params.preview {
            edit = edit.preview(preview);
        }
        if let Some(display_mode) = params.display_mode {
            edit = edit.display_mode(display_mode);
        }
        if let Some(input_mode) = params.input_mode {
            edit = edit.input_mode(input_mode);
        }
        if let Some(format) = params.format {
            edit = edit.format(format);
        }
        edit.build(ui)
    }
}

impl<'a, C: Into<EditableColor<'a>>> ColorPicker for C {
    fn build(ui: &Ui, elem: Self, params: ColorPickerParams) -> bool {
        let mut picker = ImColorPicker::new(params.label, elem.into());
        if let Some(flags) = params.flags {
            picker = picker.flags(flags);
        }
        if let Some(preview) = params.preview {
            picker = picker.preview(preview);
        }
        if let Some(mode) = params.mode {
            picker = picker.mode(mode);
        }
        if let Some(format) = params.format {
            picker = picker.format(format);
        }
        picker.build(ui)
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate as imgui_ext;

    use imgui::ColorEditFlags as Flags;

    #[test]
    fn color_test() {
        #[derive(imgui_ext::Gui)]
        struct Example {
            #[imgui(color(edit), color(picker(flags = "flags")), color(button()))]
            a: [f32; 4],
            #[imgui(color(edit(mode = "HSV"), picker, button(size = "size"),))]
            b: [f32; 4],
        }

        fn size() -> [f32; 2] {
            [42.0, 42.0]
        }

        fn flags() -> Flags {
            Flags::all()
        }
    }
}
