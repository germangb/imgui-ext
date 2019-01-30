use imgui_ext_derive as derive;
use imgui_ext_traits as traits;

pub mod prelude {
    pub use super::derive::ImGuiExt;
    pub use super::traits::{Checkbox, Drag, Input, Simple, Slider, Text, UiExt};
    pub use imgui_ext_derive;
    pub use imgui_ext_traits;
}
