//! A crate to quickly build [imgui] GUIs using a `#[derive]` macro.
//!
//! [imgui]: https://crates.io/crates/imgui
//!
//! ## Basic usage
//!
//! ```
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(slider(min = 0.0, max = 4.0))]
//!     x: f32,
//!     #[imgui(input(step = 2))]
//!     y: i32,
//!     #[imgui(drag(label = "Drag 2D"))]
//!     drag_2d: [f32; 2],
//!
//!     #[imgui(
//!         checkbox(label = "Turbo mode"),
//!         display(label = "Is turbo enabled?"),
//!     )]
//!     turbo: bool,
//! }
//! ```
//!
//! ![ui result][result]
//!
//! ## Static code generation
//!
//! No extra dynamic allocations.
//!
//! [Codegen example][example].
//!
//! [example]: https://github.com/germangb/imgui-ext/blob/master/CODEGEN.md
//!
//! ## Nested UIs
//!
//! See the [`nested`](./nested/index.html) module docs.
//!
//! ## Input events
//!
//! Most annotations can take an optional `catch = "..."` parameter which can be used to identify
//! when a button is pressed, an input has changed, etc.., later on:
//!
//! ```ignore
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(checkbox(catch = "check"))]
//!     input_check: bool,
//!     #[imgui(text(catch = "text"))]
//!     text: ImString,
//! }
//!
//! // init imgui (ui)...
//!
//! let events = ui.imgui_ext(&mut example);
//! if events.check {
//!     println!("New value: {}", example.input_check);
//! }
//! if events.text {
//!     println!("New text value: {:?}", example.text);
//! }
//! ```
//!
//! ## Combining UI and non-UI fields
//!
//! If a field is not annotated, it will be ignored in the UI.
//!
//! ```
//! use imgui_ext::prelude::*;
//!
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(label = "Some i32")]
//!     in_ui: u32,
//!
//!     // since this field is not annotated, it is ignored by the UI
//!     not_in_ui: Vec<u8>,
//! }
//! ```
//!
//! ## Descriptive errors
//!
//! UI correctness is checked at compile time.
//!
//! ```ignore
//! #[derive(ImGuiExt)]
//! struct Example {
//!     #[imgui(slider(min = 0.0))]
//!     foo: f32,
//! }
//! ```
//!
//! ```nonrust
//! error: Parameter `max` missing.
//!   --> example/src/main.rs:10:13
//!    |
//! 10 |     #[imgui(slider(min = 0.0))]
//!    |             ^^^^^^
//! ```
//!
//! [result]: https://i.imgur.com/llyqEFY.png
use imgui::Ui;
pub use imgui_ext_derive::ImGuiExt;

#[doc(hidden)]
pub mod macros;
pub mod prelude {
    pub use super::checkbox::Checkbox;
    pub use super::drag::Drag;
    pub use super::input::Input;
    pub use super::slider::Slider;
    pub use super::{ImGuiExt, UiExt};
}

/// `checkbox(...)` docs.
pub mod checkbox;
/// `drag(...)` docs.
pub mod drag;
/// `input(...)` docs.
pub mod input;
/// `slider(...)` docs.
pub mod slider;
/// `text(...)` docs.
pub mod text;
/// Support for some (basic) layout annotations.
pub mod layout {
    //!
    //! This module is mostly a work in progress. Any suggestions or contributions are very welcome.
    //!
    //! Please file [an issue] if you wish contribute.
    //!
    //! [an issue]: https://github.com/germangb/imgui-ext/issues
    //!
    //! ## Annotations:
    //!
    //! * `#[imgui(separator)]` inserts a separator
    //! * `#[imgui(new_line)]` inserts an empty line
}
/// `display(...)` docs.
pub mod display {
    //!
    //! `display(...)` is used to display a field.
    //!
    //! ## Optional fields
    //!
    //! * `label`
    //! * `display` formatted text.
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui_ext::prelude::*;
    //!
    //! #[derive(ImGuiExt)]
    //! struct Labels {
    //!     #[imgui(display)]
    //!     foo: f32,
    //!
    //!     // Use inner fields to format the text.
    //!     #[imgui(display(label = "Tuple", display = "({}, {}, {})", 0, 1, 2))]
    //!     bar: (f32, bool, usize),
    //!
    //!     // when display() is the only annotation, it can be abbreviated:
    //!     #[imgui(label = "String param")]
    //!     baz: String,
    //! }
    //! ```
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/Wf4Uze7.png
}
/// `nested(...)` docs (used to build nested UIs).
pub mod nested {
    //!
    //! Types that #[derive(ImGuiExt)] can be nested.
    //!
    //! ## Optional fields
    //!
    //! * `catch` *currently unimplemented. See [#][issue]*
    //!
    //! [issue]: #
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui::{ImString, ImGuiInputTextFlags};
    //! use imgui_ext::prelude::*;
    //!
    //! #[derive(ImGuiExt)]
    //! struct Form {
    //!     #[imgui(text)]
    //!     user: ImString,
    //!     #[imgui(text(flags = "passwd_flags"))]
    //!     passwd: ImString,
    //!     #[imgui(button(label = "Login", size = "size"))]
    //!     _btn: (),
    //! }
    //!
    //! fn passwd_flags() -> ImGuiInputTextFlags {
    //!     ImGuiInputTextFlags::Password
    //! }
    //!
    //! fn size() -> (f32, f32) {
    //!     (64.0, 24.0)
    //! }
    //!
    //! #[derive(ImGuiExt)]
    //! struct Example {
    //!     #[imgui(nested)]
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
}
/// `button(...)` docs.
pub mod button {
    //!
    //! `button(...)` is not associated to any particular type, but its position within an annotation
    //! will determine where it is rendered in the final UI.
    //!
    //! ## Fields
    //!
    //! - `label`
    //! - `size` name of a local function that returns the button size.
    //!
    //! ## Optional fields
    //!
    //! - `catch`
    //!
    //! ## Example
    //!
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Buttons {
    //!     #[imgui(
    //!         button(size = "button_size", label = "Click me!", catch = "click"),
    //!         separator,
    //!         display(label = "Clicks"),
    //!     )]
    //!     count: i32,
    //! }
    //!
    //! const fn button_size() -> (f32, f32) {
    //!     (100.0, 20.0)
    //! }
    //!
    //! // initialize ui and Buttons...
    //! let events = ui.imgui_ext(&mut buttons);
    //! if events.click {
    //!     buttons.count += 1;
    //! }
    //!
    //! ```
    //!
    //! ![][image]
    //!
    //! [image]: https://i.imgur.com/PpOcZK8.png
}
/// `bullet(...)` docs.
pub mod bullet {
    //!
    //! Used to build bulleted lists. It has two variants:
    //!
    //! * `bullet(text = "...")`
    //! * `bullet(...)` nests a UI element (currently unimplemented. See [#][issue])
    //!
    //! [issue]: #
    //!
    //! ## Example
    //!
    //! ```ignore
    //! #[derive(ImGuiExt)]
    //! struct Bullet {
    //!     #[imgui(
    //!         bullet(text = "Be nice to others."),
    //!         bullet(text = "Don't repeat your password"),
    //!         bullet(text = "Kill all humans."),
    //!         bullet(slider(min = 0.0, max = 1.0)),
    //!     )]
    //!     foo: f32,
    //! }
    //! ```
    //!
    //! ### Result
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/pe4YstR.png
}

/// Trait implemented by the derive macro.
pub trait ImGuiExt {
    type Events;
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events;
}

/// Extension trait for imgui Ui.
///
/// ```ignore
/// use imgui::Ui;
/// use imgui_ext::prelude::*;
///
/// #[derive(ImGuiExt)]
/// struct Example {
///     // ...
/// }
///
/// // initialize imgui...
/// let ui: Ui<_> = ...;
/// // initialize Example...
/// let mut example: Example = ...;
///
/// ui.imgui_ext(&mut example);
/// ```
pub trait UiExt<'ui> {
    fn imgui_ext<U: ImGuiExt>(&'ui self, ext: &mut U) -> U::Events;
}

impl<'ui> UiExt<'ui> for Ui<'ui> {
    #[inline]
    fn imgui_ext<U: ImGuiExt>(&'ui self, ext: &mut U) -> U::Events {
        imgui_ext(self, ext)
    }
}

/// Render imgui UI and collect all the events
///
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct Example {
///     #[derive(checkbox(catch = "click"))]
///     check_box: bool,
/// }
///
/// // initialize imgui and example...
///
/// let events = imgui_ext(ui, &mut example);
/// if events.click {
///     println!("New value: {}", example.check_box);
/// }
/// ```
///
/// Optionally you can call an extension method on the ui directly by using the [`UiExt`] trait.
///
/// [`UiExt`]: #
///
/// ```ignore
/// // imports the UiExt trait
/// use imgui_ext::prelude::*;
///
/// // initialize ui and example...
///
/// let events = ui.imgui_ext(&mut example);
/// if events.click {
///     // ...
/// }
#[inline]
pub fn imgui_ext<U: ImGuiExt>(ui: &Ui, ext: &mut U) -> U::Events {
    U::imgui_ext(ui, ext)
}
