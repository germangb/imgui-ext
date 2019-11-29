mod support;

use imgui::{im_str, ImStr};

#[derive(imgui_ext::Gui, Debug, Default)]
pub struct Example {
    #[imgui(combobox(label = "choose one", items = "combo_items"))]
    selected: usize,
}

fn combo_items<F>(mut callback: F)
where F: FnMut(&[&ImStr]) {
    callback(&[
      im_str!("Foo"),
      im_str!("Bar"),
      im_str!("Baz")
    ])
}

fn main() {
    support::demo().run_debug::<Example, _>(|_, _| {});
}
