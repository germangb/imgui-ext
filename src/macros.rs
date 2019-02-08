/// A macro that expands to the type that contains [catched][catched] input events.
///
/// [catched]: ./index.html#input-events
///
/// Basically this is equivalent to writing: `<Example as ImGuiExt>::Events`.
///
/// ## Example
///
/// ```ignore
/// use imgui_ext::prelude::*;
///
/// #[derive(ImGuiExt)]
/// struct Example { /*...*/ }
///
/// let mut example = Example { /*...*/ };
///
/// // init imgui (ui)...
///
/// handle_events(ui.imgui_ext(&mut example));
///
/// fn handle_events(events: Events!(Example)) {
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! Events {
    ( $ui:ty ) => {
        <$ui as $crate::ImGuiExt>::Events
    };
}
