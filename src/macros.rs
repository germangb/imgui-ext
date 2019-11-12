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
                let data_type = $variant as i32;

                unsafe {
                    sys::igDragScalar(label,
                                      data_type,
                                      elem as *const Self as _,
                                      speed,
                                      mem::transmute(min),
                                      mem::transmute(max),
                                      format,
                                      power)
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
                let data_type = $variant as _;

                unsafe {
                    sys::igDragScalarN(label,
                                       data_type,
                                       elem as *const Self as _,
                                       $len,
                                       speed,
                                       mem::transmute(min),
                                       mem::transmute(max),
                                       format,
                                       power)
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

        imgui_drag_scalar! { ( $( $scalar , )* ), ($len - 1), $variant }
    };
}

macro_rules! imgui_slider_scalar {
    ( ($scalar:ty), $len:expr, $variant: expr ) => {
        impl Slider<$scalar> for $scalar {
            fn build(_: &Ui, elem: &mut Self, params: SliderParams<$scalar>) -> bool {
                use std::{mem, ptr};

                let label = params.label.as_ptr();
                let min = &params.min;
                let max = &params.max;
                let format = ptr::null();
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant as _;

                unsafe {
                    sys::igSliderScalar(label,
                                        data_type,
                                        elem as *const Self as _,
                                        mem::transmute(min),
                                        mem::transmute(max),
                                        format,
                                        power)
                }
            }
        }
    };

    ( ( $head:ty $(, $scalar:ty)+ ), $len:expr, $variant:expr ) => {
        impl Slider<$head> for ( $head, $($scalar),+ ) {
            fn build(_: &Ui, elem: &mut Self, params: SliderParams<$head>) -> bool {
                use std::{mem, ptr};

                let label = params.label.as_ptr();
                let min = &params.min;
                let max = &params.max;
                let format = ptr::null();
                let power = params.power.unwrap_or(1.0);
                let data_type = $variant as i32;

                unsafe {
                    sys::igSliderScalarN(label,
                                         data_type,
                                         elem as *const Self as _,
                                         $len,
                                         mem::transmute(min),
                                         mem::transmute(max),
                                         format,
                                         power)
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

        imgui_slider_scalar! { ( $($scalar),+ ), ($len - 1), $variant }
    };
}

macro_rules! imgui_input_scalar {
    ( ($scalar:ty ), $len:expr, $variant: expr ) => {
        impl Input<$scalar> for $scalar {
            fn build(_: &Ui, elem: &mut Self, params: InputParams<$scalar>) -> bool {
                use std::{mem, ptr};
                let label = params.label.as_ptr();
                let step = params.step.as_ref();
                let step_fast = params.step_fast.as_ref();
                let format = ptr::null();
                let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                let data_type = $variant as i32;

                unsafe {
                    sys::igInputScalar(label,
                                       data_type,
                                       elem as *const Self as _,
                                       mem::transmute(step),
                                       mem::transmute(step_fast),
                                       format,
                                       flags.bits())
                }
            }
        }
    };

    ( ( $head:ty $(, $scalar:ty)+ ), $len:expr, $variant:expr ) => {
        impl Input<$head> for ( $head, $($scalar),* ) {
            fn build(_: &Ui, elem: &mut Self, params: InputParams<$head>) -> bool {
                use std::{mem, ptr};

                let label = params.label.as_ptr();
                let step = params.step.as_ref();
                let step_fast = params.step_fast.as_ref();
                let format = ptr::null();
                let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());
                let data_type = $variant as i32;

                unsafe {
                    sys::igInputScalarN(label,
                                        data_type,
                                        elem as *const Self as _,
                                        $len,
                                        mem::transmute(step),
                                        mem::transmute(step_fast),
                                        format,
                                        flags.bits())
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

        imgui_input_scalar! { ( $( $scalar ),+ ), ($len - 1), $variant }
    };
}

macro_rules! imgui_input_matrix {
    ( (), $size:expr, $size_2:expr, $kind:expr) => {};
    (
        ($head:ty $(, $tail:ty)*),
        $size:expr, $size_2:expr,
        $kind:expr
    ) => {
        #[cfg(feature = "matrix")]
        impl Input<$head> for [[$head; $size]; $size_2] {
            fn build(ui: &Ui, elem: &mut Self, params: InputParams<$head>) -> bool {
                let mut trigger = false;
                #[allow(unused_mut, unused_variables)]
                let mut index = 0;

                unsafe {
                    let stack_token = ui.push_id(elem[index].as_ptr());

                    let step = params.step.as_ref();
                    let step_fast = params.step_fast.as_ref();
                    let format = std::ptr::null();
                    let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());

                    trigger |= sys::igInputScalarN(params.label.as_ptr(),
                                                   $kind as i32,
                                                   elem[0].as_mut_ptr() as _,
                                                   $size,
                                                   std::mem::transmute(step),
                                                   std::mem::transmute(step_fast),
                                                   format,
                                                   flags.bits());

                    stack_token.pop(ui);
                }

                $(
                    index += 1;
                    unsafe {
                        // to match the repeating macro pattern (*)
                        let _ : $tail = std::mem::zeroed();

                        let stack_token = ui.push_id(elem[index].as_ptr());

                        let step = params.step.as_ref();
                        let step_fast = params.step_fast.as_ref();
                        let format = std::ptr::null();
                        let flags = params.flags.unwrap_or(imgui::ImGuiInputTextFlags::empty());

                        trigger |= sys::igInputScalarN(imgui::im_str!("##").as_ptr(),
                                                       $kind as i32,
                                                       elem[index].as_mut_ptr() as _,
                                                       $size,
                                                       std::mem::transmute(step),
                                                       std::mem::transmute(step_fast),
                                                       format,
                                                       flags.bits());

                        stack_token.pop(ui);
                    }
                )*

                trigger
            }
        }

        #[cfg(feature = "matrix")]
        imgui_input_matrix! { ($($tail),*), ($size-1), $size_2, $kind }
    }
}

macro_rules! imgui_drag_matrix {
    ( (), $size:expr, $size_2:expr, $kind:expr) => {};
    (
        ($head:ty $(, $tail:ty)*),
        $size:expr, $size_2:expr,
        $kind:expr
    ) => {
        #[cfg(feature = "matrix")]
        impl Drag<$head> for [[$head; $size]; $size_2] {
            fn build(ui: &Ui, elem: &mut Self, params: DragParams<$head>) -> bool {
                let mut trigger = false;

                #[allow(unused_mut, unused_variables)]
                let mut index = 0;

                unsafe {
                    let stack_token = ui.push_id(elem[index].as_ptr());

                    let label = params.label.as_ptr();
                    let min = params.min.as_ref();
                    let max = params.min.as_ref();
                    let format = std::ptr::null();
                    let speed = params.speed.unwrap_or(1.0);
                    let power = params.power.unwrap_or(1.0);

                    trigger |= sys::igDragScalarN(label,
                                                  $kind as i32,
                                                  elem[index].as_mut_ptr() as _,
                                                  $size,
                                                  speed,
                                                  std::mem::transmute(min),
                                                  std::mem::transmute(max),
                                                  format,
                                                  power);

                    stack_token.pop(ui);
                }

                $(
                    index += 1;
                    unsafe {
                        // to match the repeating macro pattern (*)
                        let _ : $tail = std::mem::zeroed();
                        
                        let stack_token = ui.push_id(elem[index].as_ptr());

                        let label = params.label.as_ptr();
                        let min = params.min.as_ref();
                        let max = params.min.as_ref();
                        let format = std::ptr::null();
                        let speed = params.speed.unwrap_or(1.0);
                        let power = params.power.unwrap_or(1.0);

                        trigger |= sys::igDragScalarN(label,
                                                      $kind as i32,
                                                      elem[index].as_mut_ptr() as _,
                                                      $size,
                                                      speed,
                                                      std::mem::transmute(min),
                                                      std::mem::transmute(max),
                                                      format,
                                                      power);

                        stack_token.pop(ui);
                    }
                )*

                trigger
            }
        }

        #[cfg(feature = "matrix")]
        imgui_drag_matrix! { ($($tail),*), ($size-1), $size_2, $kind }
    }
}

macro_rules! imgui_slider_matrix {
    ( (), $size:expr, $size_2:expr, $kind:expr) => {};
    (
        ($head:ty $(, $tail:ty)*),
        $size:expr, $size_2:expr,
        $kind:expr
    ) => {
        #[cfg(feature = "matrix")]
        impl Slider<$head> for [[$head; $size]; $size_2] {
            fn build(ui: &Ui, elem: &mut Self, params: SliderParams<$head>) -> bool {
                let mut trigger = false;

                #[allow(unused_mut)]
                let mut index = 0;

                unsafe {
                    let stack_token = ui.push_id(elem[index].as_ptr());

                    let label = params.label.as_ptr();
                    let min = &params.min;
                    let max = &params.max;
                    let format = std::ptr::null();
                    let power = params.power.unwrap_or(1.0);

                    trigger |= sys::igSliderScalarN(label,
                                                    $kind as i32,
                                                    elem[index].as_mut_ptr() as _,
                                                    $size,
                                                    std::mem::transmute(min),
                                                    std::mem::transmute(max),
                                                    format,
                                                    power);

                    stack_token.pop(ui);
                }

                $(
                    index += 1;
                    unsafe {
                        // to match the repeating macro pattern (*)
                        let _ : $tail = std::mem::zeroed();

                        let stack_token = ui.push_id(elem[index].as_ptr());

                        let label = params.label.as_ptr();
                        let min = &params.min;
                        let max = &params.max;
                        let format = std::ptr::null();
                        let power = params.power.unwrap_or(1.0);

                        trigger |= sys::igSliderScalarN(label,
                                                        $kind as i32,
                                                        elem[index].as_mut_ptr() as _,
                                                        $size,
                                                        std::mem::transmute(min),
                                                        std::mem::transmute(max),
                                                        format,
                                                        power);

                        stack_token.pop(ui);
                    }
                )*

                trigger
            }
        }

        #[cfg(feature = "matrix")]
        imgui_slider_matrix! { ($($tail),*), ($size-1), $size_2, $kind }
    }
}
