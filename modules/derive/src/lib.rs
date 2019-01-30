extern crate proc_macro;

use std::string::ToString;
use std::collections::HashMap;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    MetaList,
    Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Meta, MetaNameValue,
    NestedMeta, parse_macro_input, Token,
};
use syn::parse::{Error, Parse};
use syn::punctuated::Pair;
use syn::spanned::Spanned;

/// Grammar state machine
#[derive(Copy, Clone, Eq, PartialEq)]
enum Parser {
    Init,
    Label,
    Display,
    Widget,
}

/// Allowed tags:
///
/// To display the contents of a field:
///   - `#[imgui]`
///   - `#[imgui(label = "...")]`
///   - `#[imgui(label = "...", display = "Display format: {}", field)]`
///
/// To add interaction:
///   - `#[imgui(checkbox(...))]`
///   - `#[imgui(input(...))]`
///   - `#[imgui(drag(...))]`
///   - `#[imgui(slider(...))]`
#[proc_macro_derive(ImGuiExt, attributes(imgui))]
pub fn imgui_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

enum DisplayParam {
    Literal(Lit),
    Ident(Ident),
}

impl DisplayParam {
    fn into_tokens(self) -> TokenStream {
        match self {
            DisplayParam::Literal(lit) => quote!(#lit),
            DisplayParam::Ident(ident) => quote!(#ident),
        }
    }
}

/// `#[imgui(label = "...", display = "...", x, y, z)]`
///                                          ^---,
///                                      idents & literals
struct Display {
    label: Option<String>,
    display: Option<String>,
    params: Vec<DisplayParam>,
}

/// `#[imgui(checkbox(label = "..."))]`
#[derive(Default)]
struct Checkbox {
    label: Option<String>,
}

/// `#[imgui(input(label = "...", step = 1.0, step_fast = 1.0, precision = 3))]`
#[derive(Default)]
struct Input {
    label: Option<String>,
    flags: Option<String>,
    step: Option<f32>,
    step_fast: Option<f32>,
    precission: Option<u32>,
}

/// `#[imgui(slider(label = "...", min = 0.0, max = 4.0, format = "..."))]`
struct Slider {
    label: Option<String>,
    format: Option<String>,
    min: f32,
    max : f32,
    power: Option<f32>,
}

#[derive(Default)]
struct Drag {
    label: Option<String>,
    min: Option<f32>,
    max: Option<f32>,
    speed: Option<f32>,
    power: Option<f32>,
    display: Option<String>,
}

enum Tag {
    Display(Display),
    /// `#[imgui(separator())]`
    Separator,
    Checkbox(Checkbox),
    Input(Input),
    Slider(Slider),
    Drag(Drag),
    Nested,
}

impl Tag {
    fn display(&mut self) -> &mut Display {
        if let &mut Tag::Display(ref mut disp) = self {
            disp
        } else {
            panic!()
        }
    }
}

fn impl_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match input.data {
        Data::Struct(ref body) => struct_body(body.fields.clone()),
        _ => Err(Error::new(input.span(), "`ImGuiExt` is only supported for structs at the moment.")),
    }?;

    Ok(quote! {
        impl #impl_generics imgui_ext_traits::ImGuiExt for #name #ty_generics #where_clause {
            fn imgui_ext(ui: &imgui::Ui, ext: &mut Self) {
                #body
            }
        }
    })
}

/// Adds support to allow multiple imgui tags in a single field:
/// ```
/// struct Demo {
///     #[imgui(drag(...))]
///     x: f32,
///
///     // multiple annotations
///     #[imgui(separator)]
///     #[imgui(slider(...))]
///     #[imgui(input(...))]
///     y: f32,
/// }
fn struct_body(fields: Fields) -> Result<TokenStream, Error> {
    let field_body = fields.iter()
        .map(|field| {
            // TODO support for unnamed attributes
            let ident = field.ident.clone().expect("Named field");

            // collect all imgui attributes from this field
            // (anything that contains the `imgui` path)
            let imgui: Result<Vec<_>, Error> = field.attrs.iter()
                // TODO check attribute style (Outer or Inner)
                .filter(|attr| attr.path.is_ident(Ident::new("imgui", attr.span())))

                // parse into `Tag` enums
                .map(|f| f.parse_meta().and_then(parse_meta))
                .collect();

            match imgui {
                Ok(imgui) => parse_field_body(ident, imgui),
                Err(error) => Err(error),
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(quote!{ #( #field_body );*})
}

/// meta is the whole tag: `#[imgui]` or `#[imgui(...)]`
fn parse_meta(meta: Meta) -> Result<Tag, Error> {
    match meta {
        // simple #[imgui]
        Meta::Word(id) => Ok(Tag::Display(Display {
            label: None,
            display: None,
            params: vec![],
        })),

        Meta::NameValue(named) => Err(Error::new(named.span(), "Invalid annotation format.")),

        // general case: #[imgui(meta_list)]
        Meta::List(meta_list) => parse_meta_list(meta_list),
    }
}

/// Parse the inside of `#[imgui(...)]`
///                              ^^^
fn parse_meta_list(meta_list: MetaList) -> Result<Tag, Error> {
    let mut state = Parser::Init;
    let mut tag = Tag::Display(Display {
        label: None,
        display: None,
        params: vec![],
    });

    for pair in meta_list.nested.iter() {
        match (state, pair) {
            // label = "..." without trailing comma
            (Parser::Init, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(value), .. }))) if ident.to_string() == "label" => {
                tag.display().label = Some(value.value());
                state = Parser::Label;
            },
            (prev, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(value), .. }))) if ident.to_string() == "display" && prev == Parser::Init || prev == Parser::Label => {
                tag.display().display = Some(value.value());
                state = Parser::Display;
            },

            // Display tokens:
            //  - ident                 ;   identifier from inner field
            //  - literal               ;   raw literal
            //  - TODO ident=literal    ;   identifier in formatted str
            (Parser::Display, NestedMeta::Literal(lit)) => tag.display().params.push(DisplayParam::Literal(lit.clone())),
            (Parser::Display, NestedMeta::Meta(Meta::Word(ident))) => tag.display().params.push(DisplayParam::Ident(ident.clone())),

            // widgets that can take no parameters:
            //  - #[imgui(separator)]
            //  - #[imgui(input)]
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "separator" => {
                tag = Tag::Separator;
                state = Parser::Widget;
            },
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "input" => {
                tag = Tag::Input(Default::default());
                state = Parser::Widget;
            },
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "checkbox" => {
                tag = Tag::Checkbox(Default::default());
                state = Parser::Widget;
            },
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "drag" => {
                tag = Tag::Drag(Default::default());
                state = Parser::Widget;
            },
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "nested" => {
                tag = Tag::Nested;
                state = Parser::Widget;
            },

            // Parse widget function
            //  - #[imgui(input(...))]
            //  - #[imgui(slide(...))]
            (Parser::Init, NestedMeta::Meta(Meta::List(meta_list))) => {
                let params = parse_params(meta_list)?;
                match meta_list.ident.to_string().as_str() {
                    "nested" => tag = Tag::Nested,
                    "separator" => tag = Tag::Separator,
                    "drag" => {
                        let mut drag = Drag::default();
                        drag.min = params.get("min").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        drag.max = params.get("max").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        drag.speed = params.get("speed").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        drag.power = params.get("power").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        drag.label = params.get("label").and_then(|lit| if let Lit::Str(str) = lit { Some(str.value()) } else { None });
                        tag = Tag::Drag(drag);
                    },
                    "slider" => {
                        let min = params.get("min").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None }).ok_or(Error::new(meta_list.span(), "Attribute `min` is missing."))?;
                        let max = params.get("max").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None }).ok_or(Error::new(meta_list.span(), "Attribute `max` is missing."))?;
                        let mut slider = Slider {
                            min,
                            max,
                            label: None,
                            format: None,
                            power: None,
                        };

                        slider.power = params.get("power").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        slider.label = params.get("label").and_then(|lit| if let Lit::Str(str) = lit { Some(str.value()) } else { None });
                        tag = Tag::Slider(slider);
                    },
                    "input" => {
                        let mut input = Input::default();
                        input.flags = params.get("flags").and_then(|lit| if let Lit::Str(str) = lit { Some(str.value()) } else { None });
                        input.label = params.get("label").and_then(|lit| if let Lit::Str(str) = lit { Some(str.value()) } else { None });
                        input.step = params.get("step").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        input.step_fast = params.get("step_fast").and_then(|lit| if let Lit::Float(f) = lit { Some(f.value() as f32) } else { None });
                        tag = Tag::Input(input);
                    }
                    _ => return Err(Error::new(meta_list.ident.span(), "Unrecognized widget type.")),
                }
                state = Parser::Widget;
            }

            _ => return Err(Error::new(meta_list.span(), "Invalid attribute format.")),
        }
    }

    Ok(tag)
}

fn parse_params(params: &MetaList) -> Result<HashMap<String, Lit>, Error> {
    params.nested
        .iter()
        .map(|nested| {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, ..})) = nested {
                Ok((ident.to_string(), lit.clone()))
            } else {
                Err(Error::new(params.span(), "Invalid attribute format."))
            }
        })
        .collect::<Result<HashMap<_, _>, Error>>()
}

/// Output source code for a given field identifier, and its metadata.
///
/// imgui only contains the legal variants of Meta
///   - Meta::World => `#[imgui]`
///   - Meta::List => `#[imgui(...)]`
fn parse_field_body(ident: Ident, imgui: Vec<Tag>) -> Result<TokenStream, Error> {
    let mut token_stream = TokenStream::new();

    for tag in imgui {
        match tag {
            Tag::Separator => token_stream.extend(quote!({ui.separator()})),
            #[rustfmt::ignore]
            Tag::Input(Input { label, step, step_fast, precission, flags }) => {
                let label = Literal::string(label.unwrap_or(ident.to_string()).as_str());
                let mut params = quote!{
                    use imgui_ext_traits::params::InputParams as Params;
                    use imgui::im_str;
                    let mut params = Params {
                        label: im_str!( #label ),
                        precission: None,
                        step: None,
                        step_fast: None,
                        flags: None,
                    };
                };
                if let Some(value) = step.map(Literal::f32_suffixed) { params.extend(quote!(params.step = Some(#value);)); }
                if let Some(value) = step_fast.map(Literal::f32_suffixed) { params.extend(quote!(params.step_fast = Some(#value);)); }
                if let Some(value) = precission.map(Literal::u32_suffixed) { params.extend(quote!(params.precission = Some(#value);)); }
                if let Some(value) = flags {
                    // TODO get correct span
                    let fn_ident = Ident::new(value.as_str(), ident.span());
                    params.extend(quote! {
                        params.flags = Some( #fn_ident() );
                    });
                }
                params.extend(quote!(params));
                token_stream.extend(quote!({
                    use imgui_ext_traits::Input;
                    Input::build(ui, &mut ext.#ident, { #params });
                }));
            }
            #[rustfmt::ignore]
            Tag::Drag(Drag { label, min, max, speed, power, display }) => {
                let label = Literal::string(label.unwrap_or(ident.to_string()).as_str());
                let mut params = quote!{
                    use imgui_ext_traits::params::DragParams as Params;
                    use imgui::im_str;
                    let mut params = Params {
                        label: im_str!( #label ),
                        min: None,
                        max: None,
                        speed: None,
                        power: None,
                        display: None,
                    };
                };
                if let Some(value) = min.map(Literal::f32_suffixed) { params.extend(quote!(params.min = Some(#value);)); }
                if let Some(value) = max.map(Literal::f32_suffixed) { params.extend(quote!(params.max = Some(#value);)); }
                if let Some(value) = max.map(Literal::f32_suffixed) { params.extend(quote!(params.speed = Some(#value);)); }
                if let Some(value) = max.map(Literal::f32_suffixed) { params.extend(quote!(params.power = Some(#value);)); }
                if let Some(value) = display.map(|s| Literal::string(s.as_str())) { params.extend(quote!(params.display = Some(#value);)); }
                params.extend(quote!(params));
                token_stream.extend(quote!({
                    use imgui_ext_traits::Drag;
                    Drag::build(ui, &mut ext.#ident, { #params });
                }));
            }
            #[rustfmt::ignore]
            Tag::Slider(Slider { label, min, max, format, power }) => {
                let label = Literal::string(label.unwrap_or(ident.to_string()).as_str());
                let min = Literal::f32_suffixed(min);
                let max = Literal::f32_suffixed(max);
                let mut params = quote!{
                    use imgui_ext_traits::params::SliderParams as Params;
                    use imgui::im_str;
                    let mut params = Params {
                        label: im_str!( #label ),
                        display: None,
                        min: #min,
                        max: #max,
                        power: None,
                    };
                };
                if let Some(value) = power.map(Literal::f32_suffixed) { params.extend(quote!(params.power = Some(#value);)); }
                if let Some(value) = format.map(|s| Literal::string(s.as_str())) { params.extend(quote!(params.display = Some(#value);)); }
                params.extend(quote!(params));
                token_stream.extend(quote!({
                    use imgui_ext_traits::Slider;
                    Slider::build(ui, &mut ext.#ident, { #params });
                }));
            }
            Tag::Checkbox(Checkbox { label }) => {
                let label = Literal::string(label.unwrap_or(ident.to_string()).as_str());
                token_stream.extend(quote!({
                    use imgui_ext_traits::Checkbox;
                    use imgui_ext_traits::params::CheckboxParams as Params;
                    use imgui::im_str;
                    Checkbox::build(ui, &mut ext.#ident, Params { label: im_str!(#label) });
                }));
            }
            Tag::Nested => {
                token_stream.extend(quote! {{
                    use imgui_ext_traits::ImGuiExt;
                    ImGuiExt::imgui_ext(ui, &mut ext.#ident);
                }});
            }
            Tag::Display(Display { label, display, params }) => {
                let label = Literal::string(label.unwrap_or(ident.to_string()).as_str());

                let display = if let Some(display) = display {
                    let literal = Literal::string(display.as_str());
                    let params: Vec<_> = params.into_iter().map(|i| {
                        let field = i.into_tokens();
                        quote!( ext.#ident.#field )
                    }).collect();
                    quote!(#literal , #( #params ),*)
                } else {
                    // display the variable using the Display trait
                    quote!("{}", ext.#ident)
                };

                token_stream.extend(quote!({
                    use imgui::im_str;
                    ui.label_text(im_str!(#label), im_str!(#display));
                }));
            }
            _ => unimplemented!(),
        }
    }

    Ok(token_stream)
}
