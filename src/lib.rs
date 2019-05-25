//! Derive macro that allows you to quickly build immediate mode UIs (based on
//! the [imgui] crate).
//!
//! [imgui]: https://crates.io/crates/imgui
//!
//! ## Basic usage
//!
//! ```
//! #[derive(imgui_ext::ImGuiExt)]
//! struct Example {
//!     #[imgui(slider(min = 0.0, max = 4.0))]
//!     x: f32,
//!     #[imgui(input(step = 2))]
//!     y: i32,
//!     #[imgui(drag(label = "Drag 2D"))]
//!     drag_2d: [f32; 2],
//!     #[imgui(checkbox(label = "Turbo mode"), display(label = "Is turbo enabled?"))]
//!     turbo: bool,
//! }
//! ```
//!
//! ![][result]
//!
//! [result]: https://i.imgur.com/Xrl1Nt0.png
//!
//! ## Input events
//!
//! Rendering a UI with `imgui` & `imgui-ext` returns a type with all the
//! triggered input events (which are generally stored as booleans):
//!
//! ```
//! use imgui_ext::prelude::*;
//!
//! #[derive(imgui_ext::ImGuiExt)]
//! struct Example {
//!     #[imgui(checkbox(label = "Checkbox"))]
//!     check: bool,
//! }
//!
//! # struct A;
//! # struct B;
//! # impl A { fn imgui_ext<T>(&self, _: &mut T) -> B { B } }
//! # impl B { fn check(&self) -> bool { true } }
//! # let ui = A;
//! let mut example = Example { check: false };
//!
//! let events = ui.imgui_ext(&mut example);
//!
//! if events.check() {
//!     println!("checkbox state changed.");
//! }
//! ```
//!
//! The checkbox event is mapped to the method `check` on the returned type. The
//! name of the field & method on the returned type matches the name of the
//! field from the UI type.
//!
//! You can override this default naming by defining the "catch" attribute on
//! the annotation (all widgets support this attribute, not just checkbox):
//!
//! ```no_run
//! use imgui_ext::prelude::*;
//!
//! #[derive(imgui_ext::ImGuiExt)]
//! struct Example {
//!     #[imgui(checkbox(label = "Checkbox", catch = "checkbox_event"))]
//!     check: bool,
//! }
//!
//! # struct A;
//! # struct B;
//! # impl A { fn imgui_ext<T>(&self, _: &mut T) -> B { B } }
//! # impl B { fn checkbox_event(&self) -> bool { true } }
//! # let ui = A;
//! let mut example = Example { check: false };
//!
//! let events = ui.imgui_ext(&mut example);
//!
//! if events.checkbox_event() {
//!     println!("checkbox state changed.");
//! }
//! ```
//!
//! [repo]: https://github.com/germangb/imgui-ext
#![deny(warnings)]
use imgui::Ui;

pub use imgui_ext_derive::ImGuiExt;

include!("macros/slider.rs");
include!("macros/input.rs");
include!("macros/drag.rs");

pub mod prelude {
    pub use super::{ImGuiExt, UiExt};
}
/// `vars(...)` docs.
pub mod vars {
    //!
    //! Pushes style and color parameters to the stack, so they can be applied
    //! to the widgets contained in the annotation.
    //!
    //! This is a rather complex annotation. It's not meant to be used
    //! extensively though..
    //!
    //! ## Params
    //!
    //! - `content(...)` widgets inside this annotation will have the style and
    //!   color params applied.
    //!
    //! ## Optional params
    //!
    //! - `style = "..."` identifier of a local function that returns the style
    //!   parameters.
    //! - `color = "..."` identifier of a local function that returns the color
    //!   parameters.
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui::{ImGuiCol, StyleVar};
    //! use imgui_ext::ImGuiExt;
    //!
    //! #[derive(ImGuiExt)]
    //! struct Example {
    //!     #[imgui(vars(
    //!         style = "example_style",
    //!         color = "example_color",
    //!         content(
    //!             input(label = "foo##input"),
    //!             drag(label = "foo##drag"),
    //!             slider(label = "foo##slider", min = "-1.0", max = "1.0")
    //!         )
    //!     ))]
    //!     foo: f32,
    //! }
    //!
    //! fn example_style() -> &'static [StyleVar] {
    //!     &[StyleVar::FrameRounding(4.0)]
    //! }
    //!
    //! fn example_color() -> &'static [(ImGuiCol, [f32; 4])] {
    //!     &[(ImGuiCol::Button, [1.0, 0.0, 1.0, 1.0])]
    //! }
    //! ```
    //!
    //! ### Result
    //!
    //! ![](https://i.imgur.com/TMmjOUg.png)
}
/// `tree(...)` docs.
pub mod tree {
    //!
    //! Annotation to build static Tree UIs.
    //!
    //! This is a rather complex annotation. It's not meant to be used
    //! extensively though..
    //!
    //! ## Optional params
    //!
    //! - `label = ".."` Node label
    //! - `flags = ".."` Local function returning [`ImGuiTreeNodeFlags`]
    //! - `node(..)` Nested content (any of the supported annotations).
    //! - `cond` One of the [`ImGuiCond`] variants.
    //!
    //! [`ImGuiCond`]: https://docs.rs/imgui/*/imgui/struct.ImGuiCond.html
    //! [`ImGuiTreeNodeFlags`]: https://docs.rs/imgui/*/imgui/struct.ImGuiTreeNodeFlags.html
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui::{ImGuiTreeNodeFlags, ImString};
    //!
    //! #[derive(imgui_ext::ImGuiExt)]
    //! pub struct Tree {
    //!     #[imgui(tree(
    //!         label = "Sliders",
    //!         cond = "FirstUseEver",
    //!         flags = "flags",
    //!         node(nested)
    //!     ))]
    //!     sliders: Sliders,
    //!     #[imgui(tree(label = "Inputs", flags = "flags", node(nested)))]
    //!     inputs: Inputs,
    //!     #[imgui(tree(label = "Color picker", flags = "flags", node(color(picker))))]
    //!     color: [f32; 3],
    //! }
    //!
    //! fn flags() -> ImGuiTreeNodeFlags {
    //!     ImGuiTreeNodeFlags::Framed
    //! }
    //!
    //! #[derive(imgui_ext::ImGuiExt)]
    //! pub struct Sliders {
    //!     #[imgui(text("Slider widgets:"), slider(min = 0.0, max = 3.0))]
    //!     s1: f32,
    //!     #[imgui(slider(min = "-4", max = 4))]
    //!     s2: [i32; 3],
    //!     #[imgui(slider(min = "-1.0", max = 1.0))]
    //!     s3: [f64; 2],
    //! }
    //!
    //! #[derive(imgui_ext::ImGuiExt)]
    //! pub struct Inputs {
    //!     #[imgui(text("Input widgets:"), input)]
    //!     i1: f32,
    //!     #[imgui(input)]
    //!     i2: imgui::ImString,
    //!     #[imgui(input)]
    //!     i3: [f32; 8],
    //! }
    //! ```
    //!
    //! ## Result
    //!
    //! ![](https://i.imgur.com/Rn2RJJG.png)
}
/// `checkbox(...)` docs.
pub mod checkbox;
/// `color(...)` docs.
pub mod color;
/// `drag(...)` docs.
pub mod drag;
/// `image(...)` docs.
pub mod image;
/// `image_button(...)` docs.
pub mod image_button;
/// `input(...)` docs.
pub mod input;
/// `progress(...)` docs.
pub mod progress;
/// `slider(...)` docs.
pub mod slider;
/// `text(...)` and `text_wrap(...)` docs.
pub mod text {
    //!
    //! ## Variants
    //!
    //! - `text(...)`
    //! - `text_wrap(...)` Same as `text(...)`, but the text content wraps
    //!
    //! ## Params
    //!
    //! * `lit` Literal text. If this value is set, this value is displayed
    //!   instead of the annotated type.
    //!
    //! You can also write this annotation as:
    //!
    //! * `#[imgui(text("literal..."))]`
    //!
    //! which is a shorthand form for `text(lit = "literal...")`.
    //!
    //! ## Example
    //!
    //! ```
    //! use imgui_ext::ImGuiExt;
    //!
    //! #[derive(ImGuiExt)]
    //! struct Example {
    //!     #[imgui(text_wrap("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc metus sem, facilisis hendrerit elementum et, egestas."),
    //!             separator(),
    //!             text("Input num:"),
    //!             slider(min = "-1.0", max = 1.0),
    //!             button(label = "Submit"))]
    //!     number: f32,
    //! }
    //! ```
    //!
    //! ### Result
    //!
    //! ![](https://i.imgur.com/0uvMFIm.png)
}
/// Support for some (basic) layout annotations.
pub mod layout {
    //!
    //! This module is mostly a work in progress. Any suggestions or
    //! contributions are very welcome.
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
    //! #[derive(imgui_ext::ImGuiExt)]
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
    //! * `catch`
    //!
    //! [issue]: #
    //!
    //! ## Example
    //!
    //! ```
    //! #[derive(imgui_ext::ImGuiExt)]
    //! struct Form {
    //!     #[imgui(input)]
    //!     user: imgui::ImString,
    //!     #[imgui(
    //!         input(flags = "passwd_flags"),
    //!         button(label = "Login", catch = "login_btn")
    //!     )]
    //!     passwd: imgui::ImString,
    //! }
    //!
    //! fn passwd_flags() -> imgui::ImGuiInputTextFlags {
    //!     imgui::ImGuiInputTextFlags::Password
    //! }
    //!
    //! #[derive(imgui_ext::ImGuiExt)]
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
    //! let events: Events<Example> = ui.imgui_ext(&mut example);
    //!
    //! if events.login_form().login_btn() {
    //!     validate_user(
    //!         &example.login_form.user,
    //!         &example.login_form.passwd,
    //!     )
    //! }
    //! ```
}
/// `button(...)` docs.
pub mod button {
    //!
    //! `button(...)` is not associated to any particular type, but its position
    //! within an annotation will determine where it is rendered in the
    //! final UI.
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
    //! * `bullet(text = "...")` Bullet text.
    //! * `bullet(...)` Nested.
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
    //!         bullet(slider(min = 0.0, max = 1.0))
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
    #[cfg(test)]
    mod tests {
        #![allow(dead_code)]
        use crate as imgui_ext;
        use crate::ImGuiExt;

        #[test]
        fn bullet() {
            #[derive(ImGuiExt)]
            struct Foo {
                #[imgui(bullet(checkbox))]
                a: bool,
                #[imgui(bullet(checkbox()))]
                b: bool,
                #[imgui(bullet())]
                c: (),
            }
        }
    }
}

/// Trait implemented by the derive macro.
pub trait ImGuiExt {
    type Events;
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events;
}

impl<T: ImGuiExt> ImGuiExt for Option<T> {
    type Events = T::Events;

    // TODO remove unsafe
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events {
        if let Some(ref mut ext) = ext {
            T::imgui_ext(ui, ext)
        } else {
            unsafe { std::mem::zeroed() }
        }
    }
}

impl<T: ImGuiExt> ImGuiExt for Box<T> {
    type Events = T::Events;
    #[inline]
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events {
        T::imgui_ext(ui, ext.as_mut())
    }
}

/// Alias for the `ImGuiExt::Events` associated type.
///
/// This type is included in the prelude.
///
/// ```ignore
/// use imgui_ext::prelude::*;
/// use imgui_ext::{ImGuiExt, Events};
///
/// #[derive(ImGuiExt)]
/// struct Example { /*...*/ }
///
/// fn handle_events(e: &Events<Example>) {
///     // ...
/// }
///
/// let mut example = Example { /*...*/ };
///
/// // init imgui (ui)...
/// let events = ui.imgui_ext(&mut example);
///
/// handle_events(&events);
/// ```
pub type Events<T> = <T as ImGuiExt>::Events;

/// Extension trait for imgui Ui.
///
/// ```ignore
/// use imgui::Ui;
/// ! use imgui_ext::prelude::*;
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
/// (If you [`use imgui_ext::prelude::*`][prelude], you might want to use the
/// [`UiExt`][UiExt] trait to do the same thing).
///
/// [UiExt]: ./trait.UiExt.html
/// [prelude]: ./prelude/index.html
///
/// ```ignore
/// use imgui_ext::ImGuiExt;
///
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
