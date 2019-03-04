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

        // tail recurse
        imgui_slider_scalar! { ( $( $scalar , )* ), ($len - 1), $variant }
    };
}
