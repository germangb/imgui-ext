use imgui::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, ImGuiInputTextFlags, ImStr, ImString,
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, Ui,
};

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct SliderParams<'ui, T> {
    pub min: T,
    pub max: T,
    pub label: &'ui ImStr,
    pub format: Option<&'ui ImStr>,

    // ignored for integers
    pub power: Option<f32>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct InputParams<'ui, T> {
    pub label: &'ui ImStr,

    // ignored for integer inputs
    pub precission: Option<i32>,

    // fields ignored in multidimensional inputs
    pub step: Option<T>,
    pub step_fast: Option<T>,

    pub flags: Option<ImGuiInputTextFlags>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct DragParams<'ui, T> {
    pub label: &'ui ImStr,
    // TODO use this
    pub format: Option<&'ui ImStr>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub speed: Option<f32>,

    // ignored for integers
    pub power: Option<f32>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct SimpleParams<'ui> {
    pub label: &'ui ImStr,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct TextParams<'ui> {
    pub label: &'ui ImStr,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct CheckboxParams<'ui> {
    pub label: &'ui ImStr,
}
