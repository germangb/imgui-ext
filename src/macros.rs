/// A macro that expands to the type that contains the UI events.
///
/// ## Notes
///
/// * *Currently unimplemented. See [#][issue].*
/// * Because this macro should use [`concat_idents!`][concat], it's only available in Rust nightly.
///
/// [issue]: #
/// [concat]: https://doc.rust-lang.org/std/macro.concat_idents.html
///
/// ## Example
///
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct Example {
///     // ...
/// }
///
/// fn handle_events(events: Event!(Example)) {
///     // ...
/// }
///
/// let mut my_ui = Example { /*..*/ };
/// let ui: &Ui = ...;
///
/// handle_events(ui.imgui_ext(&mut my_ui));
/// ```
#[macro_export]
macro_rules! Events {
    ( $ui:ident ) => {
        unimplemented!("`Events!` macro is not implemented yet. See issue #0")
        //concat_idents!($ui, ImGuiExt)
    };
}
