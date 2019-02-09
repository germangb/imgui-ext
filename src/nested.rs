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
//!     #[imgui(
//!         text(flags = "passwd_flags"),
//!         button(label = "Login", catch = "login_btn"),
//!     )]
//!     passwd: ImString,
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
//! ## Nested input events
//!
//! You can access input events from nested UIs:
//!
//! ```ignore
//! // initialize imgui (ui) ...
//!
//! let mut example = Example { ... };
//! let events: Events!(Example) = ui.imgui_ext(&mut example);
//!
//! if events.login_form().login_btn() {
//!     validate_user(
//!         &example.login_form.user,
//!         &example.login_form.passwd,
//!     )
//! }
//! ```
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
