//! ## Optional params
//!
//! * `lit` Literal text. If this value is set, this value is displayed instead of the annotated type.
//!
use imgui::Ui;

pub trait Text {
    fn build(ui: &Ui, elem: &Self);
}

impl<S: AsRef<str>> Text for S {
    fn build(ui: &Ui, elem: &Self) {
        ui.text(elem)
    }
}
