//! A crate to quickly build [imgui] GUIs using a `#[derive]` macro.
//!
//! [imgui]: https://crates.io/crates/imgui
//!
//! ## Supported annotations
//!
//! Annotations map to a subset of imgui types and methods:
//!
//! | Annotation                    | Mapped Imgui Types |
//! | ----------------------------- | --- |
//! | [`slider(...)`][slider]       | [`SliderFloat`][SliderFloat], [`SliderFloat2`][SliderFloat2], [`SliderFloat3`][SliderFloat3], [`SliderFloat4`][SliderFloat4], [`SliderInt`][SliderInt], [`SliderInt2`][SliderInt2], [`SliderInt3`][SliderInt3], [`SliderInt4`][SliderInt4] |
//! | [`drag(...)`][drag]           | [`DragFloat`][DragFloat], [`DragFloat2`][DragFloat2], [`DragFloat3`][DragFloat3], [`DragFloat4`][DragFloat4], [`DragInt`][DragInt], [`DragInt2`][DragInt2], [`DragInt3`][DragInt3], [`DragInt4`][DragInt4] |
//! | [`input(...)`][input]         | [`InputFloat`][InputFloat], [`InputFloat2`][InputFloat2], [`InputFloat3`][InputFloat3], [`InputFloat4`][InputFloat4], [`InputInt`][InputInt], [`InputInt2`][InputInt2], [`InputInt3`][InputInt3], [`InputInt4`][InputInt4] |
//! | [`text(...)`][text]           | [`InputText`][InputText], [`InputTextMultiline`][InputTextMultiline] |
//! | [`progress(...)`][progress]   | [`ProgressBar`][ProgressBar] |
//! | [`image(...)`][image]         | [`Image`][ImImage] |
//! | [`button(...)`][button]       | [`Ui::button`][Ui::button], [`Ui::small_button`][Ui::small_button] |
//! | [`checkbox(...)`][checkbox]   | [`Ui::checkbox`][Ui::checkbox] |
//! | [`separator(...)`][separator] | [`Ui::separator`][Ui::separator] |
//! | [`new_line(...)`][new_line]   | [`Ui::new_line`][Ui::new_line] |
//! | [`display(...)`][display]     | [`Ui::label_text`][Ui::label_text] |
//! | [`bullet(...)`][bullet]       | [`Ui::bullet_text`][Ui::bullet_text], [`Ui::bullet`][Ui::bullet] |
//! | [`nested(...)`][nested]       | |
//!
//! [slider]: ./slider/index.html
//! [drag]: ./drag/index.html
//! [input]: ./input/index.html
//! [text]: ./text/index.html
//! [progress]: ./progress/index.html
//! [image]: ./image/index.html
//! [button]: ./button/index.html
//! [checkbox]: ./checkbox/index.html
//! [separator]: ./separator/index.html
//! [new_line]: ./new_line/index.html
//! [display]: ./display/index.html
//! [bullet]: ./bullet/index.html
//! [nested]: ./nested/index.html
//!
//! [SliderFloat]:https://docs.rs/imgui/0.0/imgui/struct.SliderFloat.html
//! [SliderFloat2]:https://docs.rs/imgui/0.0/imgui/struct.SliderFloat2.html
//! [SliderFloat3]:https://docs.rs/imgui/0.0/imgui/struct.SliderFloat3.html
//! [SliderFloat4]:https://docs.rs/imgui/0.0/imgui/struct.SliderFloat4.html
//! [SliderInt]:https://docs.rs/imgui/0.0/imgui/struct.SliderInt.html
//! [SliderInt2]:https://docs.rs/imgui/0.0/imgui/struct.SliderInt2.html
//! [SliderInt3]:https://docs.rs/imgui/0.0/imgui/struct.SliderInt3.html
//! [SliderInt4]:https://docs.rs/imgui/0.0/imgui/struct.SliderInt4.html
//! [DragFloat]:https://docs.rs/imgui/0.0/imgui/struct.DragFloat.html
//! [DragFloat2]:https://docs.rs/imgui/0.0/imgui/struct.DragFloat2.html
//! [DragFloat3]:https://docs.rs/imgui/0.0/imgui/struct.DragFloat3.html
//! [DragFloat4]:https://docs.rs/imgui/0.0/imgui/struct.DragFloat4.html
//! [DragInt]:https://docs.rs/imgui/0.0/imgui/struct.DragInt.html
//! [DragInt2]:https://docs.rs/imgui/0.0/imgui/struct.DragInt2.html
//! [DragInt3]:https://docs.rs/imgui/0.0/imgui/struct.DragInt3.html
//! [DragInt4]:https://docs.rs/imgui/0.0/imgui/struct.DragInt4.html
//! [InputFloat]:https://docs.rs/imgui/0.0/imgui/struct.InputFloat.html
//! [InputFloat2]:https://docs.rs/imgui/0.0/imgui/struct.InputFloat2.html
//! [InputFloat3]:https://docs.rs/imgui/0.0/imgui/struct.InputFloat3.html
//! [InputFloat4]:https://docs.rs/imgui/0.0/imgui/struct.InputFloat4.html
//! [InputInt]:https://docs.rs/imgui/0.0/imgui/struct.InputInt.html
//! [InputInt2]:https://docs.rs/imgui/0.0/imgui/struct.InputInt2.html
//! [InputInt3]:https://docs.rs/imgui/0.0/imgui/struct.InputInt3.html
//! [InputInt4]:https://docs.rs/imgui/0.0/imgui/struct.InputInt4.html
//! [InputText]:https://docs.rs/imgui/0.0/imgui/struct.InputText.html
//! [InputTextMultiline]:https://docs.rs/imgui/0.0/imgui/struct.InputTextMultiline.html
//! [ProgressBar]:https://docs.rs/imgui/0.0/imgui/struct.ProgressBar.html
//! [ImImage]:https://docs.rs/imgui/0.0/imgui/struct.Image.html
//!
//! [Ui::button]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.button
//! [Ui::small_button]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.small_button
//! [Ui::checkbox]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.checkbox
//! [Ui::separator]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.separator
//! [Ui::new_line]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.new_line
//! [Ui::label_text]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.label_text
//! [Ui::bullet_text]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.bullet_text
//! [Ui::bullet]: https://docs.rs/imgui/0.0/imgui/struct.Ui.html#method.bullet
//!
//! ## Basic usage
//!
//! ```
//! use imgui_ext::ImGuiExt;
//!
//! // Make your type derive ImGuiExt and place annotations on the fields you want
//! // to include in the ui
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
//!
//!     // not every field has to be annotated
//!     _not_in_ui: Vec<u8>
//! }
//! ```
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/Xrl1Nt0.png
//!
//! ## Input events
//!
//! Most annotations can take an optional `catch = "..."` parameter which can be used to identify
//! when a button is pressed, an input has changed, etc.., later on:
//!
//! ```ignore
//! use imgui_ext::ImGuiExt;
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
//! ## Contributions
//!
//! Feedback, suggestions, and contributions, are very much welcome!
//!
//! Please file an issue or open a PR to [`germangb/imgui-ext`][repo] if you wish to do so.
//!
//! [repo]: https://github.com/germangb/imgui-ext
use imgui::Ui;
pub use imgui_ext_derive::ImGuiExt;

//#[doc(hidden)]
mod macros;
pub mod prelude {
    pub use super::{ImGuiExt, UiExt};
}

/// `checkbox(...)` docs.
pub mod checkbox;
/// `drag(...)` docs.
pub mod drag;
/// `image(...)` docs.
pub mod image;
/// `input(...)` docs.
pub mod input;
/// `progress(...)` docs.
pub mod progress;
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
    //! use imgui_ext::ImGuiExt;
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
    //! * `catch` *currently unimplemented ([#4][issue])*
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
    //!
    //! ## Optional fields
    //!
    //! - `size` name of a local function that returns the button size.
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
    //! * `bullet(...)` bullets a UI element.
    //!
    //! [issue]: #
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui_ext::ImGuiExt;
    //!
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
    //! [result]: https://i.imgur.com/CLPl993.png
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
///! use imgui_ext::prelude::*;
///
/// #[derive(ImGuiExt)]
/// struct Example {
///     // ...
/// }
///
/// // initialize imgui...
/// let ui: &Ui = ...;
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
/// (If you [`use imgui_ext::prelude::*`][prelude], you might want to use the [`UiExt`][UiExt] trait to do the same thing).
///
/// [UiExt]: ./trait.UiExt.html
/// [prelude]: ./prelude/index.html
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
#[inline]
pub fn imgui_ext<U: ImGuiExt>(ui: &Ui, ext: &mut U) -> U::Events {
    U::imgui_ext(ui, ext)
}
