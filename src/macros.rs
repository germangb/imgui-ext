/// A macro that expands to the type that contains the UI events.
///
/// Input events include:
/// * Button presses
/// * Input updates
///
/// Because this macro uses [`concat_idents!`], it is a nightly-only feature.
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
/// [`concat_idents!`]: https://doc.rust-lang.org/std/macro.concat_idents.html
#[macro_export]
macro_rules! Events {
    ( $ui:ident ) => {
        unimplemented!("`Events!` macros is not implemented yet. See issue #0")
        //concat_idents!($ui, ImGuiExt)
    };
}
