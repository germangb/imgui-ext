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

fn main() {
    let mut example = Example {
        mat4: glm::identity(),
        mat3: glm::identity(),
        vec: glm::Vec4::y() * 9.81,
    };

    support::run(file!(), (640, 480), |_, ui| {
        ui.columns(2, imgui::im_str!("nalgebra"), true);
        ui.imgui_ext(&mut example);
        ui.next_column();
        ui.text_wrapped(imgui::im_str!("{:#?}", example));
    });
}

// these conversions are safe, because nalgebra types and standard arrays are
// layed out the same in memory.

#[rustfmt::skip] fn as_mat4(u: &mut glm::Mat4) -> &mut [[f32; 4]; 4] { unsafe { &mut *(u as *mut glm::Mat4 as *mut _) } }
#[rustfmt::skip] fn as_mat3(u: &mut glm::Mat3) -> &mut [[f32; 3]; 3] { unsafe { &mut *(u as *mut glm::Mat3 as *mut _) } }
#[rustfmt::skip] fn as_vec4(u: &mut glm::Vec4) -> &mut  [f32; 4]     { unsafe { &mut *(u as *mut glm::Vec4 as *mut _) } }
