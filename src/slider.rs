use imgui::{ImStr, Ui};

#[derive(Copy, Clone)]
pub struct SliderParams<'ui, T> {
    pub min: T,
    pub max: T,
    pub label: &'ui imgui::ImStr,
    pub format: Option<&'ui imgui::ImStr>,
    pub power: Option<f32>,
}
pub trait Slider<T> {
    fn build(ui: &imgui::Ui, elem: &mut Self, params: SliderParams<T>) -> bool;
}
