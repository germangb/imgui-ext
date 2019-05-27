use example::imgui_ext::prelude::*;
use example::{imgui, imgui_ext, support};

#[derive(imgui_ext::Ui, Debug)]
struct Example {
    #[imgui(text("nalgebra_glm::Vec4"), input(map = "as_vec4"), new_line)]
    vec: glm::Vec4,

    #[imgui(text("nalgebra_glm::Mat4"), input(map = "as_mat4"), new_line)]
    mat4: glm::Mat4,

    #[imgui(text("nalgebra_glm::Mat4"), drag(map = "as_mat3"))]
    mat3: glm::Mat3,
}

#[rustfmt::skip] fn as_mat4(u: &mut glm::Mat4) -> &mut [[f32; 4]; 4] { unsafe { std::mem::transmute(u) } }
#[rustfmt::skip] fn as_mat3(u: &mut glm::Mat3) -> &mut [[f32; 3]; 3] { unsafe { std::mem::transmute(u) } }
#[rustfmt::skip] fn as_vec4(u: &mut glm::Vec4) -> &mut  [f32; 4]     { unsafe { std::mem::transmute(u) } }

impl Default for Example {
    fn default() -> Self {
        Self {
            mat4: glm::identity(),
            mat3: glm::identity(),
            vec: glm::Vec4::y() * 9.81,
        }
    }
}

fn main() {
    let mut example = Example::default();

    support::run(file!(), (640, 480), |_, ui| {
        ui.columns(2, imgui::im_str!("nalgebra"), true);
        ui.imgui_ext(&mut example);
        ui.next_column();
        ui.text_wrapped(imgui::im_str!("{:#?}", example));
    });
}
