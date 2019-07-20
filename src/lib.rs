//! Derive macro that allows you to quickly build immediate mode UIs (based on
//! the [imgui] crate).
//!
//! [imgui]: https://crates.io/crates/imgui
//!
//! # Basic usage
//!
//! ```
//! #[derive(imgui_ext::Gui)]
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
//! # Input events
//!
//! Rendering a UI with `imgui` & `imgui-ext` returns a type with all the
//! triggered input events which can be accessed either by field name or by
//! method:
//!
//! ```
//! use imgui_ext::UiExt;
//!
//! #[derive(imgui_ext::Gui)]
//! struct Example {
//!     #[imgui(checkbox(label = "Checkbox"))]
//!     check: bool,
//! }
//!
//! # struct A;
//! # struct B;
//! # impl A { fn draw_gui<T>(&self, _: &mut T) -> B { B } }
//! # impl B { fn check(&self) -> bool { true } }
//! # let ui = A;
//! let mut example = Example { check: false };
//!
//! let events = ui.draw_gui(&mut example);
//!
//! if events.check() {
//!     println!("checkbox state changed.");
//! }
//! ```
//!
//! The checkbox event is mapped to the method `check` on the returned type. The
//! name of the method matches the name of the field on the Example type.
//!
//! You can override this default naming by defining the "catch" attribute on
//! the annotation (all widgets support this attribute, not just checkbox):
//!
//! ```no_run
//! use imgui_ext::UiExt;
//!
//! #[derive(imgui_ext::Gui)]
//! struct Example {
//!     #[imgui(checkbox(label = "Checkbox", catch = "checkbox_event"))]
//!     check: bool,
//! }
//!
//! # struct A;
//! # struct B;
//! # impl A { fn draw_gui<T>(&self, _: &mut T) -> B { B } }
//! # impl B { fn checkbox_event(&self) -> bool { true } }
//! # let ui = A;
//! let mut example = Example { check: false };
//!
//! let events = ui.draw_gui(&mut example);
//!
//! if events.checkbox_event() {
//!     println!("checkbox state changed.");
//! }
//! ```
//!
//! [repo]: https://github.com/germangb/imgui-ext
#![deny(warnings)]

use imgui::Ui;

pub use imgui_ext_derive::Gui;

include!("macros.rs");

/// `vars(...)` docs.
pub mod vars {
    //!
    //! Pushes style and color parameters to the stack, so they can be applied
    //! to the widgets contained in the annotation.
    //!
    //! This is a rather complex annotation. It's not meant to be used
    //! extensively though..
    //!
    //! # Params
    //!
    //! - `content(...)` widgets inside this annotation will have the style and
    //!   color params applied.
    //!
    //! # Optional params
    //!
    //! - `style = "..."` path to a function returning style parameters.
    //! - `color = "..."` path to a function returning color parameters.
    //!
    //! # Example
    //!
    //! ```
    //! use imgui::{StyleColor, StyleVar};
    //!
    //! #[derive(imgui_ext::Gui)]
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
    //! fn example_color() -> &'static [(StyleColor, [f32; 4])] {
    //!     &[(StyleColor::Button, [1.0, 0.0, 1.0, 1.0])]
    //! }
    //! ```
    //!
    //! ## Result
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
    //! # Optional params
    //!
    //! - `label = ".."` mode label.
    //! - `flags = ".."` path to a function returning [`ImGuiTreeNodeFlags`],
    //!   which is used to customize how a tree node looks.
    //! - `node(..)` list of widget annotations.
    //! - `cond` One of the [`ImGuiCond`] variants.
    //!
    //! [`ImGuiCond`]: https://docs.rs/imgui/*/imgui/struct.ImGuiCond.html
    //! [`ImGuiTreeNodeFlags`]: https://docs.rs/imgui/*/imgui/struct.ImGuiTreeNodeFlags.html
    //!
    //! # Example
    //!
    //! ```
    //! use imgui::{ImGuiTreeNodeFlags, ImString};
    //!
    //! #[derive(imgui_ext::Gui)]
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
    //! #[derive(imgui_ext::Gui)]
    //! pub struct Sliders {
    //!     #[imgui(text("Slider widgets:"), slider(min = 0.0, max = 3.0))]
    //!     s1: f32,
    //!     #[imgui(slider(min = "-4", max = 4))]
    //!     s2: [i32; 3],
    //!     #[imgui(slider(min = "-1.0", max = 1.0))]
    //!     s3: [f64; 2],
    //! }
    //!
    //! #[derive(imgui_ext::Gui)]
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
    //! # Result
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
/// `text(...)` & `text_wrap(...)` docs.
pub mod text {
    //!
    //! # Variants
    //!
    //! - `text(...)` non-wrapping text.
    //! - `text_wrap(...)` wrapping text.
    //!
    //! # Params
    //!
    //! * `lit` a string literal.
    //!
    //! You can also write this annotation as:
    //!
    //! * `#[imgui(text("literal..."))]`
    //!
    //! which is a shorthand for `text(lit = "literal...")`.
    //!
    //! # Example
    //!
    //! ```
    //! #[derive(imgui_ext::Gui)]
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
    //! # Result
    //!
    //! ![](https://i.imgur.com/0uvMFIm.png)
}
pub mod misc {
    //! []()
    //!
    //! * `#[imgui(separator)]` inserts a separator
    //! * `#[imgui(new_line)]` inserts an empty line
}
/// `display(...)` docs.
pub mod display {
    //!
    //! `display(...)` is used to display and format a field.
    //!
    //! # Optional fields
    //!
    //! * `label` override widget label.
    //! * `display` formatted text.
    //!
    //! # Example
    //!
    //! ```
    //! #[derive(imgui_ext::Gui)]
    //! struct Labels {
    //!     #[imgui(display)]
    //!     foo: f32,
    //!
    //!     // Use inner fields to format the text.
    //!     #[imgui(display(label = "Tuple", display = "({}, {}, {})", 0, 1, 2))]
    //!     bar: (f32, bool, usize),
    //!
    //!     // When display is the only annotation on a type, you may write it in this shorter form:
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
    //! Types that `#[derive(Ui)]` can be nested.
    //!
    //! # Optional fields
    //!
    //! * `catch`
    //!
    //! # Example
    //!
    //! ```
    //! #[derive(imgui_ext::Gui)]
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
    //! #[derive(imgui_ext::Gui)]
    //! struct Example {
    //!     #[imgui(nested, separator)]
    //!     login_form: Form,
    //!     #[imgui(checkbox(label = "Remember login?"))]
    //!     remember: bool,
    //! }
    //! ```
    //!
    //! ## Result
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/l6omyf4.png
    //!
    //! # Nested input events
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
    //! `button(...)` places a button on the UI.
    //!
    //! # Fields
    //!
    //! - `label` button name.
    //!
    //! # Optional fields
    //!
    //! - `size` path to a function that returns the button size.
    //! - `catch`
    //!
    //! # Example
    //!
    //! ```
    //! use imgui_ext::UiExt;
    //!
    //! #[derive(imgui_ext::Gui)]
    //! struct Button {
    //!     #[imgui(
    //!         button(size = "button_size", label = "Click me!", catch = "click"),
    //!         separator,
    //!         display(label = "Clicks")
    //!     )]
    //!     count: i32,
    //! }
    //!
    //! const fn button_size() -> [f32; 2] {
    //!     [100.0, 20.0]
    //! }
    //!
    //! # struct A;
    //! # struct B;
    //! # impl A { fn draw_gui<T>(&self, _: &mut T) -> B { B } }
    //! # impl B { fn click(&self) -> bool { true } }
    //! # let ui = A;
    //! let mut buttons = Button { count: 0 };
    //!
    //! let events = ui.draw_gui(&mut buttons);
    //!
    //! if events.click() {
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
    //! * `bullet(...)` bullet widget.
    //!
    //! # Example
    //!
    //! ```
    //! #[derive(imgui_ext::Gui)]
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
    //! ## Result
    //!
    //! ![][result]
    //!
    //! [result]: https://i.imgur.com/CLPl993.png
}

/// Trait implemented by the derive macro.
pub trait Gui {
    type Events;
    fn draw_gui(ui: &Ui, ext: &mut Self) -> Self::Events;
}

impl<T: Gui> Gui for Option<T> {
    type Events = T::Events;

    // TODO remove unsafe
    fn draw_gui(ui: &Ui, ext: &mut Self) -> Self::Events {
        if let Some(ref mut ext) = ext {
            T::draw_gui(ui, ext)
        } else {
            unsafe { std::mem::zeroed() }
        }
    }
}

impl<T: Gui> Gui for Box<T> {
    type Events = T::Events;
    #[inline]
    fn draw_gui(ui: &Ui, ext: &mut Self) -> Self::Events {
        T::draw_gui(ui, ext.as_mut())
    }
}

/// Extension trait for imgui's [`Ui`](https://docs.rs/imgui/*/imgui/struct.Ui.html).
///
/// ```
/// use imgui_ext::UiExt;
///
/// #[derive(imgui_ext::Gui)]
/// struct Example {
///     // ...
/// }
///
/// # struct A;
/// # struct B;
/// # impl A { fn draw_gui<T>(&self, _: &mut T) -> B { B } }
/// # impl B { fn click(&self) -> bool { true } }
/// # fn init_imgui() -> A { A }
///
/// // Initialize the imgui crate...
/// let ui = init_imgui();
///
/// // initialize Example...
/// let mut example = Example { /* ... */ };
///
/// ui.draw_gui(&mut example);
/// ```
pub trait UiExt {
    fn draw_gui<U: Gui>(&self, ext: &mut U) -> U::Events;
}

impl UiExt for Ui<'_> {
    #[inline]
    fn draw_gui<U: Gui>(&self, ext: &mut U) -> U::Events {
        U::draw_gui(self, ext)
    }
}
