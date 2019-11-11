//! ## Params
//!
//! * `size` path to a function that returns the size of the image.
//!
//! ## Optional params
//!
//! * `background` path to a function that returns the background color to be
//!   used.
//! * `tint` path to a function that returns a color to tint the image with.
//! * `frame_padding` an `i32`.
//! * `uv0` path to a function that returns the first uv coordinate to be used.
//!   The default value is `[0.0, 0.0]`.
//! * `uv0` path to a function that returns the second uv coordinate. The
//!   default value is `[1.0, 1.0]`.
//!
//! ## Example
//!
//! ### Result
//!
use imgui::{TextureId, Ui};

pub struct ImageButtonParams {
    pub size: [f32; 2],
    pub background: Option<[f32; 4]>,
    pub tint: Option<[f32; 4]>,
    pub uv0: Option<[f32; 2]>,
    pub uv1: Option<[f32; 2]>,
    pub frame_padding: Option<i32>,
}

pub trait ImageButton {
    fn build(ui: &Ui, elem: Self, params: ImageButtonParams);
}

impl<T> ImageButton for T
where
    T: Copy + Into<TextureId>,
{
    fn build(ui: &Ui, elem: Self, params: ImageButtonParams) {
        let mut image = imgui::ImageButton::new(elem.into(), params.size);
        if let Some(tint) = params.tint {
            image = image.tint_col(tint);
        }
        if let Some(padding) = params.frame_padding {
            image = image.frame_padding(padding);
        }
        if let Some(background) = params.background {
            image = image.background_col(background);
        }
        if let Some(uv0) = params.uv0 {
            image = image.uv0(uv0);
        }
        if let Some(uv1) = params.uv1 {
            image = image.uv1(uv1);
        }
        image.build(ui);
    }
}
