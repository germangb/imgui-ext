use imgui::Ui;

/// An extension trait for [`Ui`] that adds support for [`ImGuiExt`]
///
/// [`Ui`]: #
/// [`ImGuiExt`]: #
pub trait UiExt {
    fn ui_ext<U: ImGuiExt>(&self, ui: &mut U);
}

impl<'ui> UiExt for Ui<'ui> {
    fn ui_ext<U: ImGuiExt>(&self, ui: &mut U) {
        U::imgui_ext(self, ui);
    }
}

/// A trait to implement custom imgui UIs on user structs.
///
/// This trait is meant to be used through a `#[derive(ImGuiExt)]`
///
/// # Example
/// ```ignore
/// #[derive(ImGuiExt)]
/// struct MyStruct {
///     #[imgui(label = "Foo variable")]
///     foo: f32,
/// }
/// ```
pub trait ImGuiExt {
    fn imgui_ext(ui: &Ui, ext: &mut Self);
}
