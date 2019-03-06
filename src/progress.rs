//!
//! Works on `f32` and `Option<f32>`.
//!
//! # Optional params
//!
//! * `overlay` override default overlay text.
//! * `size` local function that returns the size.
//!
//! ## Example
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Progress {
//!     #[imgui(progress)]
//!     progress: f32,
//!     #[imgui(progress)]
//!     _progress: f32,
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/SyaN1Nt.png
use imgui::{ImStr, ImVec2, ProgressBar, Ui};

#[derive(Copy, Clone)]
pub struct ProgressParams<'p> {
    pub overlay: Option<&'p ImStr>,
    pub size: Option<ImVec2>,
}

pub trait Progress {
    fn build(ui: &Ui, elem: &Self, params: ProgressParams);
}

impl Progress for f32 {
    fn build(ui: &Ui, elem: &Self, params: ProgressParams) {
        let mut pro = ProgressBar::new(ui, *elem);
        if let Some(overlay) = params.overlay {
            pro = pro.overlay_text(overlay);
        }
        if let Some(size) = params.size {
            pro = pro.size(size);
        }
        pro.build();
    }
}

impl<T: Progress> Progress for Box<T> {
    #[inline]
    fn build(ui: &Ui, elem: &Self, params: ProgressParams) {
        T::build(ui, elem, params)
    }
}

impl<T: Progress> Progress for Option<T> {
    #[inline]
    fn build(ui: &Ui, elem: &Self, params: ProgressParams) {
        if let Some(elem) = elem {
            T::build(ui, elem, params)
        }
    }
}
