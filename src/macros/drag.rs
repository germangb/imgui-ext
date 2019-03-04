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
                let data_type = $variant;
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

        // tail recurse
        imgui_drag_scalar! { ( $( $scalar , )* ), ($len - 1), $variant }
    };
}
