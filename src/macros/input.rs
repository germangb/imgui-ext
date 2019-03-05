macro_rules! imgui_input_scalar {
    ( ($scalar:ty ,), $len:expr, $variant: expr ) => {
        impl Input<$scalar> for $scalar {
            fn build(_: &Ui, elem: &mut Self, params: InputParams<$scalar>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let step = params.step.as_ref();
                let step_fast = params.step_fast.as_ref();
                let format = ptr::null();
                let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                let data_type = $variant;
                unsafe {
                    sys::igInputScalar(label, data_type, elem as *const Self as _, mem::transmute(step), mem::transmute(step_fast), format, flags)
                }
            }
        }
    };

    ( ( $head:ty, $($scalar:ty ,)* ), $len:expr, $variant:expr ) => {
        impl Input<$head> for ( $head, $($scalar),* ) {
            fn build(_: &Ui, elem: &mut Self, params: InputParams<$head>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let step = params.step.as_ref();
                let step_fast = params.step_fast.as_ref();
                let format = ptr::null();
                let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                let data_type = $variant;
                unsafe {
                    sys::igInputScalarN(label, data_type, elem as *const Self as _, $len, mem::transmute(step), mem::transmute(step_fast), format, flags)
                }
            }
        }

        impl Input<$head> for [$head; $len] {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$head>) -> bool {
                unsafe {
                    Input::build(ui, ::std::mem::transmute::<_, &mut ( $head , $( $scalar ),* )>(elem), params)
                }
            }
        }

        // tail recurse
        imgui_input_scalar! { ( $( $scalar , )* ), ($len - 1), $variant }
    };
}


