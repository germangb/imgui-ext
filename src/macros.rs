use imgui::Ui;

use super::ImGuiExt;

#[doc(hidden)]
pub fn imgui_ext_impl<U: ImGuiExt>(ui: &Ui, ext: &mut U) -> U::Events {
    U::imgui_ext(ui, ext)
}

/// A macro to draw the UI and check *click* events.
///
/// calling this macro will return a type with all the triggered events (such as when a button is clicked).
///
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct Example {
///     #[imgui(checkbox(catch = "changed"))]
///     check: bool,
/// }
///
/// let mut example = Example { check: false };
///
/// let events = imgui_ext!(ui, &mut example);
/// if events.changed {
///     println!("New value: {}", example.check);
/// }
/// ```
#[macro_export]
macro_rules! imgui_ext {
    ($ui:expr, $gui:expr) => {{
        $crate::macros::imgui_ext_impl($ui, $gui)
    }};
}

/// A macro that expands to the type returned by [`imgui_ext!`].
///
/// This macro uses [`concat_idents!`] which is a nightly-only feature.
///
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct Example {
///     // ...
/// }
///
/// let mut my_ui = Example { /*..*/ };
/// let ui: &Ui = ...;
///
/// handle_events(imgui_ext!(ui, &mut my_ui));
///
/// fn handle_events(events: Event!(Example)) {
///     // ...
/// }
/// ```
///
/// [`imgui_ext!`]: ./macro.imgui_ext.html
/// [`concat_idents!`]: https://doc.rust-lang.org/std/macro.concat_idents.html
#[macro_export]
macro_rules! Events {
    ( $ui:ident ) => {
        concat_idents!($ui, ImGuiExt)
    };
}
