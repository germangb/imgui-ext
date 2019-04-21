//! ## Params
//!
//! * `size` local function returning image size
//!
//! ## Optional params
//!
//! * `background` local function that returns the background color
//! * `tint` local function returning tint color
//! * `frame_padding` an `i32`.
//! * `uv0` local function returning the first uv coordinate. The default value is `[0.0, 0.0]`.
//! * `uv0` local function returning the second uv coordinate. The default value is `[1.0, 1.0]`.
//!
//! ## Example
//!
//! ### Result
//!
use imgui::{ImTexture, ImVec2, ImVec4, Ui};

#[derive(Clone, Copy)]
pub struct ImageButtonParams {
    pub size: ImVec2,
    pub background: Option<ImVec4>,
    pub tint: Option<ImVec4>,
    pub uv0: Option<ImVec2>,
    pub uv1: Option<ImVec2>,
    pub frame_padding: Option<i32>,
}

pub trait ImageButton {
    fn build(ui: &Ui, elem: Self, params: ImageButtonParams);
}

impl<T> ImageButton for T where T: Copy + Into<ImTexture>
{
    fn build(ui: &Ui, elem: Self, params: ImageButtonParams) {
        let mut image = ui.image_button(elem.into(), params.size);
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
        image.build();
    }
}
