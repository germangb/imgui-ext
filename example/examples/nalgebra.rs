use nalgebra_glm as glm;

#[derive(imgui_ext::Gui, Debug)]
struct Example {
    // The "map" attribute adapts the nalgebra type (Mat4) into a type that is supported by the Ui
    // derive macro.
    #[imgui(drag(map = "as_mat_array"), new_line)]
    mat: glm::Mat4,
    #[imgui(input(map = "as_vec_array"))]
    vec: glm::Vec4,
}

// This conversion is safe, because both nalgebra type is layed out in memory
// the same as a regular [[f32; 4]; 4] array.
fn as_mat_array(u: &mut glm::Mat4) -> &mut [[f32; 4]; 4] {
    unsafe { &mut *(u.as_mut_ptr() as *mut [[f32; 4]; 4]) }
}

// Likewise, glm::Vec4 can be safely casted to a [f32; 4] for the same reason.
fn as_vec_array(u: &mut glm::Vec4) -> &mut [f32; 4] {
    unsafe { &mut *(u.as_mut_ptr() as *mut [f32; 4]) }
}

fn main() {
    let mut example = Example {
        mat: glm::identity(),
        vec: glm::Vec4::y(),
    };

    example::support::run(file!(), (640, 480), example);
}
