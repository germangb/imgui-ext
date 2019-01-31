use imgui::Ui;

use super::ImGuiExt;

#[doc(hidden)]
pub fn imgui_ext_impl<U: ImGuiExt>(ui: &Ui, ext: &mut U) {
    U::imgui_ext(ui, ext);
}

/// A macro you use to draw the UI in imgui.
#[macro_export]
macro_rules! imgui_ext {
    ($ui:expr, $gui:expr) => {
        $crate::macros::imgui_ext_impl($ui, $gui);
    };
}
