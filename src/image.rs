//! ## Params
//!
//! * `size` local function returning image size
//!
//! ## Optional params
//!
//! * `border` local function returning the border color
//! * `tint` local function returning tint color
//!
//! ## Limitations
//!
//! * `border`, `tint` and `size` cannot be set at runtime.
//! * No support to edit UVs yet.
//!
//! ## Example
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Image {
//!     #[imgui(image(size = "img_size"))]
//!     texture: usize,
//!     #[imgui(image(size = "img_size", tint = "img_tint", border = "img_border"))]
//!     texture_tint: usize,
//! }
//!
//! const fn img_size() -> (f32, f32) {
//!     (512.0, 64.0)
//! }
//!
//! const fn img_tint() -> (f32, f32, f32, f32) {
//!     (1.0, 0.0, 1.0, 1.0)
//! }
//!
//! const fn img_border() -> (f32, f32, f32, f32) {
//!     (1.0, 1.0, 1.0, 1.0)
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/RoJdyGR.png
//!
use imgui::{ImTexture, ImVec2, ImVec4, Ui};

#[derive(Clone, Copy)]
pub struct ImageParams {
    pub size: ImVec2,
    pub border: Option<ImVec4>,
    pub tint: Option<ImVec4>,
}

pub trait Image {
    fn build(ui: &Ui, elem: Self, params: ImageParams);
}

impl<T> Image for T
where
    T: Copy + Into<ImTexture>,
{
    fn build(ui: &Ui, elem: Self, params: ImageParams) {
        let mut image = ui.image(elem.into(), params.size);
        if let Some(tint) = params.tint {
            image = image.tint_col(tint);
        }
        if let Some(border) = params.border {
            image = image.border_col(border);
        }
        image.build();
    }
}
