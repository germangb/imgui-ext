//!
//! Types that #[derive(ImGuiExt)] can be nested.
//!
//! ## Optional fields
//!
//! * `catch`
//!
//! [issue]: #
//!
//! ## Example
//!
//! ```
//! use imgui::{ImString, ImGuiInputTextFlags};
//! use imgui_ext::ImGuiExt;
//!
//! #[derive(ImGuiExt)]
//! struct Form {
//!     #[imgui(text)]
//!     user: ImString,
//!     #[imgui(text(flags = "passwd_flags"))]
//!     passwd: ImString,
//!     #[imgui(button(label = "Login"))]
//!     _btn: (),
//! }
//!
//! fn passwd_flags() -> ImGuiInputTextFlags {
//!     ImGuiInputTextFlags::Password
//! }
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(nested, separator)]
//!     login_form: Form,
//!     #[imgui(checkbox(label = "Remember login?"))]
//!     remember: bool,
//! }
//! ```
//!
//! ### Result
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/l6omyf4.png
//!
use std::ops::Deref;

use super::ImGuiExt;

// Used in codegen.
#[doc(hidden)]
pub struct NestedCatch<T: ImGuiExt>(pub T::Events);

impl<T: ImGuiExt> Deref for NestedCatch<T> {
    type Target = T::Events;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
