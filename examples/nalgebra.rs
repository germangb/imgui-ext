mod support;

type Mat4 = na::Matrix4<f32>;
type Vec4 = na::Vector4<f32>;

#[derive(imgui_ext::Gui, Debug)]
struct Example {
    #[imgui(drag(map = "as_mat_array"), new_line)]
    mat: Mat4,
    #[imgui(input(map = "as_vec_array"))]
    vec: Vec4,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            mat: Mat4::identity(),
            vec: na::zero(),
        }
    }
}

// This is safe since both Mat4 and [[f32; 4]; 4] have the same memory layout.
fn as_mat_array(u: &mut Mat4) -> &mut [[f32; 4]; 4] {
    unsafe { &mut *(u.as_mut_ptr() as *mut _) }
}

// safe version using TryInto/TryFrom
fn as_vec_array(u: &mut Vec4) -> &mut [f32; 4] {
    use std::convert::TryInto;

    u.as_mut_slice().try_into().unwrap()
}

fn main() {
    support::demo().run_debug::<Example, _>(|_, _| {});
}
