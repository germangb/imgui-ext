macro_rules! imgui_drag_scalar {
    ( ($scalar:ty ,), $len:expr, $variant: expr ) => {
        impl Drag<$scalar> for $scalar {
            fn build(_: &Ui, elem: &mut Self, params: DragParams<$scalar>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let min = params.min.as_ref();
                let max = params.min.as_ref();
                let format = ptr::null();
                let speed = params.speed.unwrap_or(1.0);
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant;
                unsafe {
                    sys::igDragScalar(label, data_type, elem as *const Self as _, speed, mem::transmute(min), mem::transmute(max), format, power)
                }
            }
        }
    };

    ( ( $head:ty, $($scalar:ty ,)* ), $len:expr, $variant:expr ) => {
        impl Drag<$head> for ( $head, $($scalar),* ) {
            fn build(_: &Ui, elem: &mut Self, params: DragParams<$head>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let min = params.min.as_ref();
                let max = params.min.as_ref();
                let format = ptr::null();
                let speed = params.speed.unwrap_or(1.0);
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant;
                unsafe {
                    sys::igDragScalarN(label, data_type, elem as *const Self as _, $len, speed, mem::transmute(min), mem::transmute(max), format, power)
                }
            }
        }

        impl Drag<$head> for [$head; $len] {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<$head>) -> bool {
                unsafe {
                    Drag::build(ui, ::std::mem::transmute::<_, &mut ( $head , $( $scalar ),* )>(elem), params)
                }
            }
        }

        // tail recurse
        imgui_drag_scalar! { ( $( $scalar , )* ), ($len - 1), $variant }
    };
}

macro_rules! imgui_slider_scalar {
    ( ($scalar:ty ,), $len:expr, $variant: expr ) => {
        impl Slider<$scalar> for $scalar {
            fn build(_: &Ui, elem: &mut Self, params: SliderParams<$scalar>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let min = &params.min;
                let max = &params.max;
                let format = ptr::null();
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant;
                unsafe {
                    sys::igSliderScalar(label, data_type, elem as *const Self as _, mem::transmute(min), mem::transmute(max), format, power)
                }
            }
        }
    };

    ( ( $head:ty, $($scalar:ty ,)* ), $len:expr, $variant:expr ) => {
        impl Slider<$head> for ( $head, $($scalar),* ) {
            fn build(_: &Ui, elem: &mut Self, params: SliderParams<$head>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let min = &params.min;
                let max = &params.max;
                let format = ptr::null();
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant;
                unsafe {
                    sys::igSliderScalarN(label, data_type, elem as *const Self as _, $len, mem::transmute(min), mem::transmute(max), format, power)
                }
            }
        }

        impl Slider<$head> for [$head; $len] {
            #[inline]
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<$head>) -> bool {
                unsafe {
                    Slider::build(ui, ::std::mem::transmute::<_, &mut ( $head , $( $scalar ),* )>(elem), params)
                }
            }
        }

        // tail recurse
        imgui_slider_scalar! { ( $($scalar ,)* ), ($len - 1), $variant }
    };
}

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

macro_rules! imgui_input_matrix {
    ( ($head:ty), $size:expr, $kind:expr) => {};
    ( ($head:ty, $($tail:ty),+), $size:expr, $kind:expr ) => {
        impl Input<$head> for [[$head; $size]; $size] {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$head>) -> bool {
                let mut trigger = false;

                let mut index = 0;
                ui.push_id(elem[index].as_ptr());

                unsafe {
                    let step = params.step.as_ref();
                    let step_fast = params.step_fast.as_ref();
                    let format = std::ptr::null();
                    let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                    trigger |= sys::igInputScalarN(params.label.as_ptr(), $kind, elem[0].as_mut_ptr() as _, $size, std::mem::transmute(step), std::mem::transmute(step_fast), format, flags);
                }


                $(
                    index += 1;
                    ui.push_id(elem[index].as_ptr());
                    unsafe {
                        let _: $tail = std::mem::zeroed();

                        let step = params.step.as_ref();
                        let step_fast = params.step_fast.as_ref();
                        let format = std::ptr::null();
                        let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                        trigger |= sys::igInputScalarN(imgui::im_str!("##").as_ptr(), $kind, elem[index].as_mut_ptr() as _, $size, std::mem::transmute(step), std::mem::transmute(step_fast), format, flags);
                    }
                )+

                $(
                    unsafe {
                        let _: $tail = std::mem::zeroed();
                    }
                    ui.pop_id();
                )+
                ui.pop_id();

                trigger
            }
        }

        imgui_input_matrix! { ($($tail),*), ($size-1), $kind }
    }
}


