/// A macro you use to draw the UI in imgui.
#[macro_export]
macro_rules! imgui_ext {
    ($ui:expr, $gui:expr) => {
        $crate::imgui_ext_impl($ui, $gui);
    };
}
