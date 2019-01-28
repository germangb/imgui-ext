pub use imgui_ext_derive as derive;
pub use imgui_ext_traits as traits;

pub mod prelude {
    pub use super::derive::ImGuiExt;
    pub use super::traits::UiExt;
}
