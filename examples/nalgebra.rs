mod support;

type Mat4 = na::Matrix4<f32>;
type Vec4 = na::Vector4<f32>;

#[derive(imgui_ext::Gui, Debug)]
struct Example {
    // The "map" attribute adapts the nalgebra type (Mat4) into a type that is supported by the Ui
    // derive macro.
    #[imgui(drag(map = "as_mat_array"), new_line)]
    mat: Mat4,
    #[imgui(input(map = "as_vec_array"))]
    vec: Vec4,
}

// This is safe because both Mat4 and [[f32; 4]; 4] have the same memory layout.
fn as_mat_array(u: &mut Mat4) -> &mut [[f32; 4]; 4] {
    unsafe { &mut *(u.as_mut_ptr() as *mut _) }
}

// safe version using TryInto/TryFrom
fn as_vec_array(u: &mut Vec4) -> &mut [f32; 4] {
    use std::convert::TryInto;

    u.as_mut_slice().try_into().unwrap()
}

fn main() {
    let mut example = Example {
        mat: na::Matrix4::identity(),
        vec: na::Vector4::y(),
    };

    support::run(file!(), (640, 480), example);
}
