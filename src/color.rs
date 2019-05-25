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
//!   [`ImGuiColorEditFlags`][ImGuiColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `size` Function identifier that returns the button size.
//! * `catch`
//!
//! ## Color Edit
//!
//! `color(edit(...))`
//!
//! ### Optional params
//!
//! * `label`
//! * `flags` Function identifier that returns a
//!   [`ImGuiColorEditFlags`][ImGuiColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `mode` Allowed values: `"RGB"`, `"HSV"`, `"HEX"`
//!   ([`ColorEditMode`][ColorEditMode] variants).
//! * `format` Allowed values: `"Float"`, `"U8"` ([`ColorFormat`][ColorFormat]
//!   variants).
//! * `catch`
//!
//! ## Color Picker
//!
//! `color(picker(...))`
//!
//! ### Optional params
//!
//! * `label`
//! * `flags` Function identifier that returns a
//!   [`ImGuiColorEditFlags`][ImGuiColorEditFlags].
//! * `preview` Allowed values: `"Opaque"`, `"HalfAlpha"`, `"Alpha"`
//!   ([`ColorPreview`][ColorPreview] variants).
//! * `mode` Allowed values: `"HueBar"`, `"HueWheel"`
//!   ([`ColorPickerMode`][ColorEditMode] variants).
//! * `format` Allowed values: `"Float"`, `"U8"` ([`ColorFormat`][ColorFormat]
//!   variants).
//! * `catch`
//!
//! ## Example
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
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
//! [ImGuiColorEditFlags]: https://docs.rs/imgui/0.0/imgui/struct.ImGuiColorEditFlags.html
//! [ColorPreview]: https://docs.rs/imgui/0.0/imgui/enum.ColorPreview.html
//! [ColorFormat]: https://docs.rs/imgui/0.0/imgui/enum.ColorFormat.html
//! [ColorEditMode]: https://docs.rs/imgui/0.0/imgui/enum.ColorEditMode.html
//! [ColorPickerMode]: https://docs.rs/imgui/0.0/imgui/enum.ColorPickerMode.html
use imgui::{
    ColorButton as ImColorButton, ColorEdit as ImColorEdit, ColorEditMode, ColorFormat,
    ColorPicker as ImColorPicker, ColorPickerMode, ColorPreview, EditableColor,
    ImGuiColorEditFlags, ImStr, ImVec2, ImVec4, Ui,
};

#[derive(Copy, Clone)]
pub struct ColorButtonParams<'p> {
    pub label: &'p ImStr,
    pub flags: Option<ImGuiColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub size: Option<ImVec2>,
}

#[derive(Copy, Clone)]
pub struct ColorEditParams<'p> {
    pub label: &'p ImStr,
    pub flags: Option<ImGuiColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub format: Option<ColorFormat>,
    pub mode: Option<ColorEditMode>,
}

#[derive(Copy, Clone)]
pub struct ColorPickerParams<'p> {
    pub label: &'p ImStr,
    pub flags: Option<ImGuiColorEditFlags>,
    pub preview: Option<ColorPreview>,
    pub format: Option<ColorFormat>,
    pub mode: Option<ColorPickerMode>,
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

impl<C: Into<ImVec4>> ColorButton for C {
    fn build(ui: &Ui, elem: Self, params: ColorButtonParams) -> bool {
        let mut button = ImColorButton::new(ui, params.label, elem.into());
        if let Some(flags) = params.flags {
            button = button.flags(flags);
        }
        if let Some(preview) = params.preview {
            button = button.preview(preview);
        }
        if let Some(size) = params.size {
            button = button.size(size);
        }
        button.build()
    }
}

impl<'p, C: Into<EditableColor<'p>>> ColorEdit for C {
    fn build(ui: &Ui, elem: Self, params: ColorEditParams) -> bool {
        let mut edit = ImColorEdit::new(ui, params.label, elem.into());
        if let Some(flags) = params.flags {
            edit = edit.flags(flags);
        }
        if let Some(preview) = params.preview {
            edit = edit.preview(preview);
        }
        if let Some(mode) = params.mode {
            edit = edit.mode(mode);
        }
        if let Some(format) = params.format {
            edit = edit.format(format);
        }
        edit.build()
    }
}

impl<'p, C: Into<EditableColor<'p>>> ColorPicker for C {
    fn build(ui: &Ui, elem: Self, params: ColorPickerParams) -> bool {
        let mut picker = ImColorPicker::new(ui, params.label, elem.into());
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
        picker.build()
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate as imgui_ext;
    use crate::ImGuiExt;

    use imgui::ImGuiColorEditFlags as Flags;

    #[test]
    fn color_test() {
        #[derive(ImGuiExt)]
        struct Example {
            #[imgui(color(edit), color(picker(flags = "flags")), color(button()))]
            a: [f32; 4],
            #[imgui(color(edit(mode = "HSV"), picker, button(size = "size"),))]
            b: [f32; 4],
        }

        fn size() -> (f32, f32) {
            (42.0, 42.0)
        }

        fn flags() -> Flags {
            Flags::all()
        }
    }
}
