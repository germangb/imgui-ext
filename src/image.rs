//! ## Params
//!
//! * `size` local function returning image size
//!
//! ## Optional params
//!
//! * `border` local function returning the border color
//! * `tint` local function returning tint color
//! * `uv0` local function returning the first uv coordinate. The default value is `[0.0, 0.0]`.
//! * `uv0` local function returning the second uv coordinate. The default value is `[1.0, 1.0]`.
//!
//! ## Limitations
//!
//! * Parameters cannot be set at runtime (including `uv`s). This may be a deal breaker for most applications that deal with texture atlases.
//!
//! ## Example
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Image {
//!     #[imgui(image(size = "size", uv0 = "uv0", uv1 = "uv1"))]
//!     texture: usize,
//!     #[imgui(image(size = "size", tint = "tint", border = "border"))]
//!     texture_tint: usize,
//! }
//!
//! fn size() -> [f32; 2] {
//!     [512.0, 64.0]
//! }
//!
//! fn tint() -> [f32; 4] {
//!     [1.0, 0.0, 1.0, 1.0]
//! }
//!
//! fn border() -> [f32; 4] {
//!     [1.0, 1.0, 1.0, 1.0]
//! }
//!
//! fn uv0() -> [f32; 2] {
//!     [0.0, 0.0]
//! }
//!
//! fn uv1() -> [f32; 2] {
//!     [1.0, 1.0]
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
    pub uv0: Option<ImVec2>,
    pub uv1: Option<ImVec2>,
}

pub trait Image {
    fn build(ui: &Ui, elem: Self, params: ImageParams);
}

impl<T> Image for T where T: Copy + Into<ImTexture>
{
    fn build(ui: &Ui, elem: Self, params: ImageParams) {
        let mut image = ui.image(elem.into(), params.size);
        if let Some(tint) = params.tint {
            image = image.tint_col(tint);
        }
        if let Some(border) = params.border {
            image = image.border_col(border);
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
