extern crate proc_macro;

use std::string::ToString;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Meta, MetaNameValue,
    NestedMeta, parse_macro_input, Token,
};
use syn::parse::{Error, Parse};
use syn::punctuated::Pair;
use syn::spanned::Spanned;

mod errors;

enum ImGuiAttr {
    // - `#[imgui]`
    // - `#[imgui(label = "...")]`
    Simple {
        label: Option<String>,
    },

    // `#[imgui(input)]`
    // `#[imgui(input( ... )]`
    InputF32 {
        label: Option<String>,
        precission: Option<i32>,
        step: Option<f32>,
        step_fast: Option<f32>,
    },
    InputI32 {
        label: Option<String>,
        step: Option<i32>,
        step_fast: Option<i32>,
    },

    // `#[imgui(slider( ... ))]`
    SliderF32 {
        label: Option<String>,
        display: Option<String>,
        min: f32,
        max: f32,
        power: Option<f32>,
    },
    SliderI32 {
        label: Option<String>,
        display: Option<String>,
        min: i32,
        max: i32,
    },

    // `#[imgui(drag( ... ))]`
    DragF32 {
        label: Option<String>,
        display: Option<String>,
        min: Option<f32>,
        max: Option<f32>,
        speed: Option<f32>,
        power: Option<f32>,
    },
    DragI32 {
        label: Option<String>,
        display: Option<String>,
        min: Option<i32>,
        max: Option<i32>,
        speed: Option<f32>,
    },

    // `#[imgui(text( ... ))]`
    Test {
        label: Option<String>,
    }
}

impl ImGuiAttr {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        unimplemented!()
    }

    fn into_token_stream(self, ident: &Ident) -> Result<TokenStream, Error> {
        match self {
            ImGuiAttr::Simple { label } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));

                Ok(quote! {{
                    imgui_ext_traits::Simple::build(ui, &mut ext.#ident, imgui_ext_traits::params::SimpleParams {
                        label: imgui::im_str!( #label ),
                    })
                }})
            }
            ImGuiAttr::InputF32 {
                label,
                precission,
                step,
                step_fast,
            } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                let precission = precission.map(Literal::i32_suffixed);
                let step = step.map(Literal::f32_suffixed);
                let step_fast = step_fast.map(Literal::f32_suffixed);
                let mut fields = TokenStream::new();

                fields.extend(quote! { label: imgui::im_str!( #label ), });

                if let Some(val) = precission {
                    fields.extend(quote! { precission: Some( #val ), })
                } else {
                    fields.extend(quote! { precission: None, })
                }

                if let Some(val) = step {
                    fields.extend(quote! { step: Some( #val ), })
                } else {
                    fields.extend(quote! { step: None, })
                }

                if let Some(val) = step_fast {
                    fields.extend(quote! { step_fast: Some( #val ), })
                } else {
                    fields.extend(quote! { step_fast: None, })
                }

                Ok(quote! {
                    imgui_ext_traits::Input::build(ui, &mut ext.#ident, imgui_ext_traits::params::InputParams {
                        #fields
                    })
                })
            }
            ImGuiAttr::InputI32 {
                label,
                step,
                step_fast,
            } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                let step = step.map(Literal::i32_suffixed);
                let step_fast = step_fast.map(Literal::i32_suffixed);
                let mut fields = TokenStream::new();

                // precission is included because (for now) both i32 and f32 inputs share the
                // same Params struct.
                fields.extend(quote! { label: imgui::im_str!( #label ), precission: None, });

                if let Some(val) = step {
                    fields.extend(quote! { step: Some( #val ), })
                } else {
                    fields.extend(quote! { step: None, })
                }

                if let Some(val) = step_fast {
                    fields.extend(quote! { step_fast: Some( #val ), })
                } else {
                    fields.extend(quote! { step_fast: None, })
                }

                Ok(quote! {
                    imgui_ext_traits::Input::build(ui, &mut ext.#ident, imgui_ext_traits::params::InputParams {
                        #fields
                    })
                })
            }
            ImGuiAttr::SliderF32 {
                label,
                display,
                min,
                max,
                power,
            } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                let minlit = Literal::f32_suffixed(min);
                let maxlit = Literal::f32_suffixed(max);

                let mut fields = quote! {
                    min: #minlit,
                    max: #maxlit,
                    label: imgui::im_str!( #label ),
                };

                if let Some(disp) = display.map(|s| Literal::string(s.as_str())) {
                    fields.extend(quote! { display: Some(imgui::im_str!(#disp)), });
                } else {
                    fields.extend(quote! { display: None, });
                }

                if let Some(value) = power.map(|s| Literal::f32_suffixed(s)) {
                    fields.extend(quote! { power: Some(#value), });
                } else {
                    fields.extend(quote! { power: None, });
                }

                Ok(quote! {
                    imgui_ext_traits::Slider::build(ui, &mut ext.#ident, imgui_ext_traits::params::SliderParams {
                        #fields
                    })
                })
            }
            ImGuiAttr::SliderI32 {
                label,
                display,
                min,
                max,
            } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                let minlit = Literal::i32_suffixed(min);
                let maxlit = Literal::i32_suffixed(max);

                let mut fields = quote! {
                    min: #minlit,
                    max: #maxlit,
                    label: imgui::im_str!( #label ),
                };

                if let Some(disp) = display.map(|s| Literal::string(s.as_str())) {
                    fields.extend(quote! { display: Some(imgui::im_str!(#disp)), });
                } else {
                    fields.extend(quote! { display: None, });
                }

                // we specify power even though it has no use with integers
                // because both f32 and i32 share the same Params struct
                fields.extend(quote! { power: None, });

                Ok(quote! {
                    imgui_ext_traits::Slider::build(ui, &mut ext.#ident, imgui_ext_traits::params::SliderParams {
                        #fields
                    })
                })
            }
            ImGuiAttr::DragF32 {
                label,
                display,
                min,
                max,
                power,
                speed,
            } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                let mut fields = quote! { label: imgui::im_str!(#label), };

                if let Some(val) = display {
                    fields.extend(quote! { display: Some(imgui::im_str!(#val)), });
                } else {
                    fields.extend(quote! { display: None, })
                }

                if let Some(val) = min {
                    fields.extend(quote! { min: Some(#val), });
                } else {
                    fields.extend(quote! { min: None, })
                }

                if let Some(val) = max {
                    fields.extend(quote! { max: Some(#val), });
                } else {
                    fields.extend(quote! { max: None, })
                }

                if let Some(val) = power {
                    fields.extend(quote! { power: Some(#val), });
                } else {
                    fields.extend(quote! { power: None, })
                }

                if let Some(val) = speed {
                    fields.extend(quote! { speed: Some(#val), });
                } else {
                    fields.extend(quote! { speed: None, })
                }

                Ok(quote! {
                    imgui_ext_traits::Drag::build(ui, &mut ext.#ident, imgui_ext_traits::params::DragParams {
                        #fields
                    })
                })
            },
            ImGuiAttr::Test { label } => {
                let label = Literal::string(&label.unwrap_or(ident.to_string()));
                Ok(quote! {
                    imgui_ext_traits::Text::build(ui, &mut ext.#ident, imgui_ext_traits::params::TextParams {
                        label: imgui::im_str!( #label ),
                    })
                })
            },
            _ => unimplemented!(),
        }
    }
}

#[proc_macro_derive(ImGuiExt, attributes(imgui))]
pub fn imgui_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match input.data {
        Data::Struct(ref body) => imgui_body_fields(body.fields.clone()),
        _ => Err(Error::new(input.span(), "Only structs")),
    }?;

    Ok(quote! {
        impl #impl_generics imgui_ext_traits::ImGuiExt for #name #ty_generics #where_clause {
            fn imgui_ext(ui: &imgui::Ui, ext: &mut Self) {
                #body
            }
        }
    })
}

fn parse_input(meta_list: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    // int
    let mut i32_step: Option<i32> = None;
    let mut i32_step_fast: Option<i32> = None;

    // float
    let mut step: Option<f32> = None;
    let mut step_fast: Option<f32> = None;
    let mut precission: Option<i32> = None;

    let mut label = None;

    for item in meta_list.nested.iter() {
        match item {
            NestedMeta::Literal(l) => {
                // TODO FIXME error messages
                return Err(Error::new(
                    meta_list.span(),
                    "Unrecognized attribute literal",
                ));
            }
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Int(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "step" => {
                        if step.or(step_fast).is_some() || precission.is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if i32_step.is_some() {
                            return Err(errors::already_defined(ident.span(), "step"));
                        } else {
                            i32_step = Some(lit.value() as i32)
                        }
                    }
                    "step_fast" => {
                        if step.or(step_fast).is_some() || precission.is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if i32_step_fast.is_some() {
                            return Err(errors::already_defined(ident.span(), "step_fast"));
                        } else {
                            i32_step_fast = Some(lit.value() as i32)
                        }
                    }
                    "precission" => {
                        if i32_step.or(i32_step_fast).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if precission.is_some() {
                            return Err(errors::already_defined(ident.span(), "precission"));
                        } else {
                            precission = Some(lit.value() as i32)
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Float(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "step" => {
                        if i32_step.or(i32_step_fast).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if step.is_some() {
                            return Err(errors::already_defined(ident.span(), "step"));
                        } else {
                            step = Some(lit.value() as f32)
                        }
                    }
                    "step_fast" => {
                        if i32_step.or(i32_step_fast).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if step_fast.is_some() {
                            return Err(errors::already_defined(ident.span(), "step_fast"));
                        } else {
                            step_fast = Some(lit.value() as f32)
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "label" => {
                        if label.is_some() {
                            return Err(errors::already_defined(ident.span(), "label"));
                        } else {
                            label = Some(lit.value())
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                _ => return Err(Error::new(meta_list.span(), "Unrecognized attribute")),
            },
        }
    }

    if i32_step.or(i32_step_fast).is_some() {
        Ok(ImGuiAttr::InputI32 {
            step: i32_step,
            step_fast: i32_step_fast,
            label,
        })
    } else {
        Ok(ImGuiAttr::InputF32 {
            step,
            step_fast,
            label,
            precission,
        })
    }
}

fn parse_slider(meta_list: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    let mut label = None;
    let mut display = None;

    // integer slider
    let mut i32_min: Option<i32> = None;
    let mut i32_max: Option<i32> = None;

    // float slider
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;
    let mut power: Option<f32> = None;

    for item in meta_list.nested.iter() {
        match item {
            NestedMeta::Literal(l) => {
                return Err(Error::new(
                    meta_list.span(),
                    "Unrecognized attribute literal",
                ));
            }
            NestedMeta::Meta(meta) => match meta {
                // Parse float slider
                // Make sure mixing i32 and f32 raises build errors
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Float(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "min" => {
                        if i32_min.or(i32_max).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if min.is_some() {
                            return Err(errors::already_defined(ident.span(), "min"));
                        } else {
                            min = Some(lit.value() as f32)
                        }
                    }
                    "max" => {
                        if i32_min.or(i32_max).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if max.is_some() {
                            return Err(errors::already_defined(ident.span(), "max"));
                        } else {
                            max = Some(lit.value() as f32)
                        }
                    }
                    "power" => {
                        if i32_min.or(i32_max).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if power.is_some() {
                            return Err(errors::already_defined(ident.span(), "power"));
                        } else {
                            power = Some(lit.value() as f32)
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Int(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "min" => {
                        if min.or(min).or(power).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if i32_min.is_some() {
                            return Err(errors::already_defined(ident.span(), "min"));
                        } else {
                            i32_min = Some(lit.value() as i32)
                        }
                    }
                    "max" => {
                        if min.or(min).or(power).is_some() {
                            return Err(errors::invalid_format(ident.span()));
                        }
                        if i32_max.is_some() {
                            return Err(errors::already_defined(ident.span(), "max"));
                        } else {
                            i32_max = Some(lit.value() as i32)
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "label" => {
                        if label.is_some() {
                            return Err(errors::already_defined(ident.span(), "label"));
                        } else {
                            label = Some(lit.value())
                        }
                    }
                    "display" => {
                        if display.is_some() {
                            return Err(errors::already_defined(ident.span(), "display"));
                        } else {
                            display = Some(lit.value())
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                _ => return Err(Error::new(meta_list.span(), "Unrecognized attribute 2")),
            },
        }
    }

    if min.or(max).is_some() {
        Ok(ImGuiAttr::SliderF32 {
            min: min.ok_or(errors::missin_attrib(meta_list.span(), "min"))?,
            max: max.ok_or(errors::missin_attrib(meta_list.span(), "max"))?,
            label,
            display,
            power,
        })
    } else {
        Ok(ImGuiAttr::SliderI32 {
            min: i32_min.ok_or(errors::missin_attrib(meta_list.span(), "min"))?,
            max: i32_max.ok_or(errors::missin_attrib(meta_list.span(), "max"))?,
            label,
            display,
        })
    }
}

fn parse_text_input(meta_list: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    let mut label = None;
    for item in meta_list.nested.iter() {
        match item {
            NestedMeta::Literal(l) => {
                return Err(Error::new(
                    meta_list.span(),
                    "Unrecognized attribute literal",
                ));
            }
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "label" => {
                        if label.is_some() {
                            return Err(errors::already_defined(ident.span(), "label"));
                        } else {
                            label = Some(lit.value())
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                _ => return Err(Error::new(meta_list.span(), "Unrecognized attribute 2")),
            },
        }
    }

    Ok(ImGuiAttr::Test { label })
}

fn parse_drag(meta_list: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    // int
    // TODO

    // float
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;
    let mut power: Option<f32> = None;

    let mut speed: Option<f32> = None;
    let mut label = None;
    let mut display = None;

    for item in meta_list.nested.iter() {
        match item {
            NestedMeta::Literal(l) => {
                return Err(Error::new(
                    meta_list.span(),
                    "Unrecognized attribute literal",
                ));
            }
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Float(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "min" => {
                        if min.is_some() {
                            return Err(errors::already_defined(ident.span(), "min"));
                        } else {
                            min = Some(lit.value() as f32)
                        }
                    }
                    "max" => {
                        if max.is_some() {
                            return Err(errors::already_defined(ident.span(), "max"));
                        } else {
                            max = Some(lit.value() as f32)
                        }
                    }
                    "speed" => {
                        if speed.is_some() {
                            return Err(errors::already_defined(ident.span(), "speed"));
                        } else {
                            speed = Some(lit.value() as f32)
                        }
                    }
                    "power" => {
                        if power.is_some() {
                            return Err(errors::already_defined(ident.span(), "power"));
                        } else {
                            power = Some(lit.value() as f32)
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Int(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "min" => unimplemented!("DragInt is not implemented yet!"),
                    "max" => unimplemented!("DragInt is not implemented yet!"),
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(lit),
                    ..
                }) => match ident.to_string().as_str() {
                    "label" => {
                        if label.is_some() {
                            return Err(errors::already_defined(ident.span(), "label"));
                        } else {
                            label = Some(lit.value())
                        }
                    }
                    "display" => {
                        if display.is_some() {
                            return Err(errors::already_defined(ident.span(), "display"));
                        } else {
                            display = Some(lit.value())
                        }
                    }
                    id @ _ => return Err(errors::unrecog_ident(ident.span(), id.to_string())),
                },
                _ => return Err(Error::new(meta_list.span(), "Unrecognized attribute 2")),
            },
        }
    }

    Ok(ImGuiAttr::DragF32 {
        min,
        max,
        label,
        display,
        speed,
        power,
    })
}

// Parse the tokens between the parenthesis of a MetaList, that is, what
// is inside the parenthesis of this annotation:
//
//  - #[imgui( ... )]
//            ^^^^^
fn parse_meta_list(name: &Ident, meta: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    // Allow only one level of nested depth
    let nested = &meta.nested;
    if nested.len() != 1 {
        return Err(errors::invalid_format(nested.span()));
    }

    match nested.first() {
        // TODO
        // Do we want to support both:
        // - `#[imgui( foo )]` and
        // - `#[imgui( foo, )]` (with trailing comma)
        // or just the first one?
        Some(Pair::End(attr)) | Some(Pair::Punctuated(attr, _)) => {
            match attr {
                // This is not allowed (having a literal inside of the annotation)
                //  - `#[imgui("...")]`
                //  - `#[imgui(42)]`
                NestedMeta::Literal(lit) => Err(errors::invalid_format(meta.span())),

                NestedMeta::Meta(meta) => {
                    match meta {
                        // We should have
                        //  - `#[imgui(label = "...")]`
                        Meta::NameValue(MetaNameValue {
                            ident,
                            lit: Lit::Str(label),
                            ..
                        }) => {
                            let ident = ident.to_string();
                            if ident == "label" {
                                Ok(ImGuiAttr::Simple {
                                    label: Some(label.value()),
                                })
                            } else {
                                Err(errors::unrecog_ident(ident.span(), ident))
                            }
                        }

                        // Check things like:
                        //  - `#[imgui(input( ... ))]`
                        //  - `#[imgui(progress( ... ))]`
                        //  - `#[imgui(slider( ... ))]`
                        Meta::List(meta_list) => match meta_list.ident.to_string().as_str() {
                            "input" => parse_input(meta_list),
                            "slider" => parse_slider(meta_list),
                            "drag" => parse_drag(meta_list),
                            "text" => parse_text_input(meta_list),
                            _ => Err(errors::invalid_format(meta_list.span())),
                        },

                        // Special cases like:
                        //  - `#[input(text)]`
                        //  - `#[input(drag)]`
                        //
                        // Everything else should raise a compilation error.
                        Meta::Word(ident) => match ident.to_string().as_str() {
                            "input" => Ok(ImGuiAttr::InputF32 {
                                label: None,
                                precission: None,
                                step: None,
                                step_fast: None,
                            }),
                            "drag" => Ok(ImGuiAttr::DragF32 {
                                label: None,
                                display: None,
                                min: None,
                                max: None,
                                speed: None,
                                power: None,
                            }),
                            "text" => Ok(ImGuiAttr::Test {
                                label: None,
                            }),
                            _ => Err(errors::invalid_format(name.span())),
                        },

                        _ => Err(errors::invalid_format(name.span())),
                    }
                }
            }
        }
        _ => Err(errors::invalid_format(meta.span())),
    }
}

// #[imgui( ... )]
//   ^^^^^^^^^^^^
fn parse_meta(name: &Ident, meta: &Meta) -> Result<ImGuiAttr, Error> {
    use syn::MetaList;

    match meta {
        // At this point we know we have this:
        // #[imgui]
        &Meta::Word(_) => Ok(ImGuiAttr::Simple { label: None }),

        // #[imgui( meta_list )]
        //
        // We might have (but not be limited to):
        //  - #[imgui(display = "...")]
        //  - #[imgui(input( ... ))]
        //  - #[imgui(progress( ... ))]
        //  - #[imgui(slider( ... ))]
        &Meta::List(ref meta_list) => parse_meta_list(name, meta_list),

        // This type of attribute is not allowed
        //  - #[imgui = "..."]
        &Meta::NameValue(ref meta) => Err(errors::invalid_format(meta.span())),
    }
}

fn imgui_body_fields(fields: Fields) -> Result<TokenStream, Error> {
    let field_assign = fields
        .iter()
        .map(|field| {
            // collect all #[imgui] attributes
            let mut attributes = field
                .attrs
                .iter()
                .filter(is_imgui_attr)
                .map(Attribute::parse_meta)
                .collect::<Result<Vec<_>, Error>>()?;

            // Only one `#[imgui]` attribute per field is allowed.
            // If we encounter more than one, raise a compilation error
            if attributes.is_empty() {
                return Ok(TokenStream::new());
            } else if attributes.len() > 1 {
                return Err(Error::new(
                    field.span(),
                    "Only one `#[imgui]` tag per attribute is allowed",
                ));
            }

            // At this point, we are parsing the following attribute:
            //
            // #[imgui( ... )]
            //   ^^^^^^^^^^^^
            // Therefore it is safe to unwrap
            let attr_meta = attributes.get(0).unwrap();
            let ident = field.ident.as_ref().unwrap();

            parse_meta(&ident, attr_meta)?.into_token_stream(&ident)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(quote! {
        #( #field_assign );*
    })
}

fn is_imgui_attr(attr: &&Attribute) -> bool {
    attr.path.is_ident(Ident::new("imgui", Span::call_site()))
}
