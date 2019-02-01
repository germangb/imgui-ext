use imgui::{ImStr, Ui};

#[derive(Copy, Clone)]
pub struct DragParams<'ui, T> {
    pub label: &'ui imgui::ImStr,
    pub format: Option<&'ui imgui::ImStr>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub speed: Option<f32>,
    pub power: Option<f32>,
}

pub trait Drag<T> {
    fn build(ui: &imgui::Ui, elem: &mut Self, params: DragParams<T>);
}
