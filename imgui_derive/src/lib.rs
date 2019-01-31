extern crate proc_macro;

use std::collections::HashMap;
use std::string::ToString;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Lit, Meta, MetaList,
    MetaNameValue, NestedMeta,
};

// TODO richer errors
const INVALID_FORMAT: &str = "Invalid annotation format.";
const MULTIPLE_ANNOT: &str = "Found multiple `#[imgui(...)]` annotations in a single field.";
const STRUCT_SUPPORT: &str = "`#[derive(ImGuiExt)]` only supports structs.";
const UNRECOG_MODE: &str = "Unrecognized mode.";
const UNEXPECTED_PARAM: &str = "Unexpected parameter.";

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

/// `#[imgui(label(label = "...", display = "...", x, y, z))]`
///                                                   ^---,
///                                                idents & literals
#[derive(Default)]
struct Label {
    label: Option<Lit>,
    display: Option<Lit>,
    params: Vec<DisplayParam>,
}

/*
//Never used
impl Display {
    const PARAMS: &'static [&'static str] = &["label", "display"];
}
*/

/// `#[imgui(checkbox(label = "..."))]`
#[derive(Default)]
struct Checkbox {
    label: Option<Lit>,
}

impl Checkbox {
    fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
        let mut widget = Self {
            label: None
        };
        for param in list.nested.iter() {
            match param {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => match ident.to_string().as_str() {
                    "label" => widget.label = Some(lit.clone()),
                    _ => return Err(Error::new(ident.span(), UNEXPECTED_PARAM)),
                }
                // TODO use proper span
                _ => return Err(Error::new(list.span(), INVALID_FORMAT)),
            }
        }
        Ok(widget)
    }
}

/// `#[imgui(input(label = "...", step = 1.0, step_fast = 1.0, precision = 3))]`
#[derive(Default)]
struct Input {
    label: Option<Lit>,
    flags: Option<Lit>,
    step: Option<Lit>,
    step_fast: Option<Lit>,
    precision: Option<Lit>,
}

impl Input {
    fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
        let mut widget = Self {
            label: None,
            flags: None,
            step: None,
            step_fast: None,
            precision: None
        };
        for param in list.nested.iter() {
            match param {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => match ident.to_string().as_str() {
                    "label" => widget.label = Some(lit.clone()),
                    "flags" => widget.flags = Some(lit.clone()),
                    "step" => widget.step = Some(lit.clone()),
                    "step_fast" => widget.step_fast = Some(lit.clone()),
                    "precission" => widget.precision = Some(lit.clone()),
                    _ => return Err(Error::new(ident.span(), UNEXPECTED_PARAM)),
                }
                // TODO use proper span
                _ => return Err(Error::new(list.span(), INVALID_FORMAT)),
            }
        }
        Ok(widget)
    }
}

/// `#[imgui(slider(label = "...", min = 0.0, max = 4.0, format = "..."))]`
struct Slider {
    label: Option<Lit>,
    format: Option<Lit>,
    min: Lit,
    max: Lit,
    power: Option<Lit>,
}

impl Slider {
    const PARAMS: &'static [&'static str] = &["label", "format", "min", "max", "power"];
}

#[derive(Default)]
struct Drag {
    label: Option<Lit>,
    min: Option<Lit>,
    max: Option<Lit>,
    speed: Option<Lit>,
    power: Option<Lit>,
    format: Option<Lit>,
}

impl Drag {
    fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
        let mut widget = Self {
            label: None,
            min: None,
            max: None,
            speed: None,
            power: None,
            format: None
        };
        for param in list.nested.iter() {
            match param {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => match ident.to_string().as_str() {
                    "label" => widget.label = Some(lit.clone()),
                    "min" => widget.min = Some(lit.clone()),
                    "max" => widget.max = Some(lit.clone()),
                    "speed" => widget.speed = Some(lit.clone()),
                    "power" => widget.power = Some(lit.clone()),
                    "format" => widget.format = Some(lit.clone()),
                    _ => return Err(Error::new(ident.span(), UNEXPECTED_PARAM)),
                }
                // TODO use proper span
                _ => return Err(Error::new(list.span(), INVALID_FORMAT)),
            }
        }
        Ok(widget)
    }
}

enum Tag {
    Label(Label),
    Checkbox(Checkbox),
    Input(Input),
    Slider(Slider),
    Drag(Drag),
    Nested,

    /// `#[imgui(separator)]`
    Separator,
    /// `#[imgui(new_line)]`
    NewLine,
}

impl Tag {
    fn display(&mut self) -> &mut Label {
        if let &mut Tag::Label(ref mut disp) = self {
            disp
        } else {
            panic!("Unexpected state")
        }
    }
}

fn impl_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match input.data {
        Data::Struct(ref body) => struct_body(body.fields.clone()),
        _ => Err(Error::new(input.span(), STRUCT_SUPPORT)),
    }?;

    Ok(quote! {
        impl #impl_generics imgui_ext::ImGuiExt for #name #ty_generics #where_clause {
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
#[rustfmt::skip]
fn struct_body(fields: Fields) -> Result<TokenStream, Error> {
    let field_body = fields
        .iter()
        .flat_map(|field| {

            // TODO add support for unnamed attributes
            let ident = field.ident.clone().expect("Named field");

            // collect all the imgui attributes
            // we need to check that there is only one.
            let attrs: Vec<Attribute> = field.attrs.iter()
                .filter(|attr| {
                    let ident = Ident::new("imgui", attr.span());
                    attr.path.is_ident(ident)
                })
                .cloned()
                .collect();

            let mut attrs = attrs.into_iter();
            let first = attrs.next();
            let second = attrs.next();

            match (first, second) {
                // No annotations were found.
                // Emmit no sourcecode.
                (None,          None     ) => vec![Ok(TokenStream::new())],

                // There is more than one imgui annotation.
                // Raise a descriptive error pointing to the extra annotation.
                (Some(_),       Some(err)) => vec![Err(Error::new(err.span(), MULTIPLE_ANNOT))],

                // There is a single annotation, as it should.
                // Parse the annotation and emmit the source code for this field
                (Some(attr),    None     ) => {
                    let tags = attr
                        .parse_meta()           // -> Meta
                        .and_then(parse_meta);  // -> Result<Vec<Tag>>

                    match tags {
                        Err(e) => vec![Err(e)],
                        Ok(tags) => tags
                            .into_iter()
                            .map(|tag| emmit_tag_tokens(&ident, (&attr, &tag)))
                            .collect()
                    }
                },

                _ => unreachable!(),
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(quote! { #( #field_body );*})
}

/// meta is the whole (parsed) tag: `#[imgui]` or `#[imgui(...)]`
fn parse_meta(meta: Meta) -> Result<Vec<Tag>, Error> {
    match meta {
        // #[imgui = ...] Nope
        Meta::NameValue(named) => Err(Error::new(named.span(), INVALID_FORMAT)),
        // #[imgui], treated as an empty label
        Meta::Word(_) => Ok(vec![Tag::Label(Label::default())]),
        // #[imgui(meta_list)] (general)
        Meta::List(meta_list) => parse_meta_list(meta_list),
    }
}

/// Parse the inside of `#[imgui(...)]`
///                              ^^^
/// `#[imgui(foo(...), bar(...))]`
/// `#[imgui(foo(...),)]`
/// `#[imgui(foo(...))]`
///
/// (Init) -> (Label)
///   - label = "..."
///   - display = "..."
///
fn parse_meta_list(meta_list: MetaList) -> Result<Vec<Tag>, Error> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum State {
        Init,
        Label,
        Tags,
    }

    let mut state = State::Init;
    let mut tags = vec![];

    for nested in meta_list.nested.iter() {
        match (state, nested) {
            (_, NestedMeta::Literal(_)) => return Err(Error::new(meta_list.span(), INVALID_FORMAT)),
            // Parse as a label(...)
            (State::Init, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, .. })))
                if ident.to_string() == "label" || ident.to_string() == "display" =>
            {
                tags.push(Tag::Label(parse_label(&meta_list)?));
                // any errors will have been reported by the previous call to `parse_label`.
                // At this point I can break out of the loop.
                break;
            },

            // widgets that can take no parameters
            (s, NestedMeta::Meta(Meta::Word(ident))) if s == State::Init || s == State::Tags => {
                match ident.to_string().as_str() {
                    "separator" => tags.push(Tag::Separator),
                    "new_line" => tags.push(Tag::NewLine),
                    "nested" => tags.push(Tag::Nested),

                    "label" => tags.push(Tag::Label(Default::default())),
                    "checkbox" => tags.push(Tag::Checkbox(Default::default())),
                    "input" => tags.push(Tag::Input(Default::default())),
                    "drag" => tags.push(Tag::Drag(Default::default())),
                    _ => return Err(Error::new(meta_list.span(), UNRECOG_MODE)),
                }
                state = State::Tags;
            }
            (s, NestedMeta::Meta(Meta::List(meta_list))) if s == State::Init || s == State::Tags => {

                let tag = match meta_list.ident.to_string().as_str() {
                    "separator" => Tag::Separator,
                    "new_line" => Tag::NewLine,
                    "nested" => Tag::Nested,

                    "label" => Tag::Label(parse_label(&meta_list)?),
                    "checkbox" => Tag::Checkbox(Checkbox::from_meta_list(meta_list)?),
                    "input" => Tag::Input(Input::from_meta_list(meta_list)?),
                    "drag" => Tag::Drag(Drag::from_meta_list(meta_list)?),
                    _ => return Err(Error::new(meta_list.span(), UNRECOG_MODE)),
                };

                tags.push(tag);
                state = State::Tags;
            }
            _ => panic!(),
        }
    }
    Ok(tags)
    /*
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Parser {
        Init,
        Label,
        Display,
        Widget,
    }

    let mut state = Parser::Init;
    let mut tag = Tag::Display(Label {
        label: None,
        display: None,
        params: vec![],
    });

    for pair in meta_list.nested.iter() {
        match (state, pair) {
            // label = "..." without trailing comma
            (
                Parser::Init,
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(value),
                    ..
                })),
            ) if ident.to_string() == "label" => {
                tag.display().label =
                    Some(Lit::Str(syn::LitStr::new(&value.value(), value.span())));
                state = Parser::Label;
            }
            (
                prev,
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    ident,
                    lit: Lit::Str(value),
                    ..
                })),
            ) if ident.to_string() == "display" && prev == Parser::Init
                || prev == Parser::Label =>
            {
                //tag.display().display = Some(value.value());
                tag.display().display =
                    Some(Lit::Str(syn::LitStr::new(&value.value(), value.span())));
                state = Parser::Display;
            }

            // Display tokens:
            //  - ident                 ;   identifier from inner field
            //  - literal               ;   raw literal
            //  - TODO ident=literal    ;   identifier in formatted str
            (Parser::Display, NestedMeta::Literal(lit)) => tag
                .display()
                .params
                .push(DisplayParam::Literal(lit.clone())),
            (Parser::Display, NestedMeta::Meta(Meta::Word(ident))) => tag
                .display()
                .params
                .push(DisplayParam::Ident(ident.clone())),

            // widgets that can take no parameters:
            //  - #[imgui(separator)]
            //  - #[imgui(input)]
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident)))
                if ident.to_string() == "separator" =>
            {
                tag = Tag::Separator;
                state = Parser::Widget;
            }
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident)))
                if ident.to_string() == "new_line" =>
            {
                tag = Tag::NewLine;
                state = Parser::Widget;
            }
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "input" => {
                tag = Tag::Input(Default::default());
                state = Parser::Widget;
            }
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident)))
                if ident.to_string() == "checkbox" =>
            {
                tag = Tag::Checkbox(Default::default());
                state = Parser::Widget;
            }
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident))) if ident.to_string() == "drag" => {
                tag = Tag::Drag(Default::default());
                state = Parser::Widget;
            }
            (Parser::Init, NestedMeta::Meta(Meta::Word(ident)))
                if ident.to_string() == "nested" =>
            {
                tag = Tag::Nested;
                state = Parser::Widget;
            }

            // Parse widget function
            //  - #[imgui(input(...))]
            //  - #[imgui(slide(...))]
            (Parser::Init, NestedMeta::Meta(Meta::List(meta_list))) => {
                let params = parse_params(meta_list)?;
                let params_ident = params.iter().map(|(param, (ident, _))| (param, ident));
                match meta_list.ident.to_string().as_str() {
                    "nested" => {
                        validate_fields(params_ident, &[])?;
                        tag = Tag::Nested
                    }
                    "separator" => {
                        validate_fields(params_ident, &[])?;
                        tag = Tag::Separator
                    }
                    "new_line" => {
                        validate_fields(params_ident, &[])?;
                        tag = Tag::NewLine
                    }
                    "checkbox" => {
                        validate_fields(params_ident, Checkbox::PARAMS)?;
                        let mut check = Checkbox::default();
                        check.label = params.get("label").map(|(_, lit)| lit.clone());
                        tag = Tag::Checkbox(check);
                    }
                    "drag" => {
                        validate_fields(params_ident, Drag::PARAMS)?;
                        let mut drag = Drag::default();
                        drag.min = params.get("min").map(|(_, lit)| lit).map(Clone::clone);
                        drag.max = params.get("max").map(|(_, lit)| lit).map(Clone::clone);
                        drag.speed = params.get("speed").map(|(_, lit)| lit.clone());
                        drag.power = params.get("power").map(|(_, lit)| lit.clone());
                        drag.label = params.get("label").map(|(_, lit)| lit.clone());
                        drag.format = params.get("format").map(|(_, lit)| lit.clone());
                        tag = Tag::Drag(drag);
                    }
                    "slider" => {
                        validate_fields(params_ident, Slider::PARAMS)?;
                        let min = params
                            .get("min")
                            .map(|(_, lit)| lit)
                            .ok_or(Error::new(meta_list.span(), "Attribute `min` is missing."))?;
                        let max = params
                            .get("max")
                            .map(|(_, lit)| lit)
                            .ok_or(Error::new(meta_list.span(), "Attribute `max` is missing."))?;
                        let mut slider = Slider {
                            min: min.clone(),
                            max: max.clone(),
                            label: None,
                            format: None,
                            power: None,
                        };

                        slider.power = params.get("power").map(|(_, lit)| lit.clone());
                        slider.label = params.get("label").map(|(_, lit)| lit.clone());
                        tag = Tag::Slider(slider);
                    }
                    "input" => {
                        validate_fields(params_ident, Input::PARAMS)?;
                        let mut input = Input::default();
                        input.step = params.get("step").map(|(_, lit)| lit).map(Clone::clone);
                        input.step_fast = params
                            .get("step_fast")
                            .map(|(_, lit)| lit)
                            .map(Clone::clone);
                        input.flags = params.get("flags").map(|(_, lit)| lit.clone());
                        input.label = params.get("label").map(|(_, lit)| lit.clone());
                        input.precision = params.get("precision").map(|(_, lit)| lit.clone());
                        tag = Tag::Input(input);
                    }
                    _ => {
                        return Err(Error::new(meta_list.ident.span(), INVALID_FORMAT));
                    }
                }
                state = Parser::Widget;
            }

            _ => return Err(Error::new(meta_list.span(), INVALID_FORMAT)),
        }
    }


    Ok(tag)
    */
}

/// Parse the contents of: `foo(k=v, ...)`
/// It must contain only key=value pairs, otherwise returns an Err.
fn parse_params(params: &MetaList) -> Result<HashMap<String, (Ident, Lit)>, Error> {
    params
        .nested
        .iter()
        .map(|nested| {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) = nested {
                Ok((ident.to_string(), (ident.clone(), lit.clone())))
            } else {
                Err(Error::new(params.span(), INVALID_FORMAT))
            }
        })
        .collect::<Result<HashMap<_, _>, Error>>()
}

/// Parse the contents of a label tag: `label(label = "...", display = "...", foo, bar)`
/// Asumes that `params.ident` is equal to "label"
fn parse_label(params: &MetaList) -> Result<Label, Error> {
    #[derive(Clone, Copy)]
    enum State {
        Init,
        Display,
    }

    let mut state = State::Init;
    let mut display = Label::default();

    for attr in params.nested.iter() {
        match (state, attr) {
            (State::Display, NestedMeta::Literal(lit)) => {
                display.params.push(DisplayParam::Literal(lit.clone()));
            }

            (State::Display, NestedMeta::Meta(Meta::Word(ident))) => {
                display.params.push(DisplayParam::Ident(ident.clone()));
            }

            (State::Init, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })))
                if ident.to_string() == "label" =>
            {
                display.label = Some(lit.clone());
            }

            (State::Init, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })))
                if ident.to_string() == "display" =>
            {
                display.display = Some(lit.clone());
                state = State::Display;
            }

            _ => return Err(Error::new(params.span(), INVALID_FORMAT)),
        }
    }

    Ok(display)
}

/// Output source code for a given field, a given attribute, and one of the parsed `Tag`s
///
/// For example, the this annotation: `#[imgui(label(...), input(...))]`
/// produces two tags: `Tag::Label` and `Tag::Input`.
///
/// This function needs to be called twice (once per Tag)
fn emmit_tag_tokens(ident: &Ident, (attr, tag): (&Attribute, &Tag)) -> Result<TokenStream, Error> {
    let tokens = match tag {
        Tag::Separator => quote!({ ui.separator() }),
        Tag::NewLine => quote!({ ui.new_line() }),
        Tag::Input(Input {
            label,
            step,
            step_fast,
            precision,
            flags,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::input::InputParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    precision: None,
                    step: None,
                    step_fast: None,
                    flags: None,
                };
            };

            match step {
                Some(Lit::Float(step)) => params.extend(quote! { params.step = Some(#step); }),
                Some(Lit::Int(step)) => params.extend(quote! { params.step = Some(#step); }),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match step_fast {
                Some(Lit::Float(step)) => params.extend(quote! { params.step_fast = Some(#step); }),
                Some(Lit::Int(step)) => params.extend(quote! { params.step_fast = Some(#step); }),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match precision {
                Some(Lit::Int(value)) => params.extend(quote! { params.precision = Some(#value); }),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), ident.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            params.extend(quote!(params));
            quote!({
                use imgui_ext::Input;
                Input::build(ui, &mut ext.#ident, { #params });
            })
        }
        Tag::Drag(Drag {
            label,
            min,
            max,
            speed,
            power,
            format,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::drag::DragParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    min: None,
                    max: None,
                    speed: None,
                    power: None,
                    format: None,
                };
            };

            match (min, max) {
                (Some(Lit::Float(min)), Some(Lit::Float(max))) => {
                    params.extend(quote!(params.min = Some(#min);));
                    params.extend(quote!(params.max = Some(#max);));
                }
                (Some(Lit::Int(min)), Some(Lit::Int(max))) => {
                    params.extend(quote!(params.min = Some(#min);));
                    params.extend(quote!(params.max = Some(#max);));
                }
                (Some(Lit::Float(min)), None) => params.extend(quote!(params.min = Some(#min);)),
                (Some(Lit::Int(min)), None) => params.extend(quote!(params.min = Some(#min);)),
                (None, Some(Lit::Float(max))) => params.extend(quote!(params.max = Some(#max);)),
                (None, Some(Lit::Int(max))) => params.extend(quote!(params.max = Some(#max);)),
                (None, None) => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match speed {
                Some(Lit::Float(value)) => params.extend(quote! { params.speed = Some(#value); }),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            match power {
                Some(Lit::Float(value)) => params.extend(quote! { params.power = Some(#value); }),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            match format {
                Some(Lit::Str(value)) => {
                    params.extend(quote!(params.format = Some(im_str!(#value));))
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            params.extend(quote!(params));
            quote!({
                use imgui_ext::Drag;
                Drag::build(ui, &mut ext.#ident, { #params });
            })
        }
        Tag::Slider(Slider {
            label,
            min,
            max,
            format,
            power,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let min_max = match (min, max) {
                (Lit::Int(min), Lit::Int(max)) => quote! { min: #min, max: #max },
                (Lit::Float(min), Lit::Float(max)) => quote! { min: #min, max: #max },
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let mut params = quote! {
                use imgui_ext::slider::SliderParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    format: None,
                    #min_max,
                    power: None,
                };
            };
            match format {
                Some(Lit::Str(value)) => params.extend(quote!(params.format = Some(#value);)),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            match power {
                Some(Lit::Float(value)) => params.extend(quote!(params.power = Some(#value);)),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            params.extend(quote!(params));
            quote!({
                use imgui_ext::Slider;
                Slider::build(ui, &mut ext.#ident, { #params });
            })
        }
        Tag::Checkbox(Checkbox { label }) => {
            let label = match label {
                Some(Lit::Str(lab)) => lab.value(),
                None => ident.to_string(),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            quote!({
                use imgui_ext::Checkbox;
                use imgui_ext::checkbox::CheckboxParams as Params;
                use imgui::im_str;
                Checkbox::build(ui, &mut ext.#ident, Params { label: im_str!(#label) });
            })
        }
        Tag::Nested => {
            quote! {{
                use imgui_ext::ImGuiExt;
                ImGuiExt::imgui_ext(ui, &mut ext.#ident);
            }}
        }
        Tag::Label(Label {
            label,
            display,
            params,
        }) => {
            let label = match label {
                Some(Lit::Str(lab)) => lab.value(),
                None => ident.to_string(),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);

            let display = match display {
                Some(Lit::Str(disp)) => Some(disp.value()),
                None => None,
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };

            let display = if let Some(display) = display {
                let literal = Literal::string(display.as_str());
                let params: Vec<_> = params
                    .into_iter()
                    .map(|field| match field {
                        DisplayParam::Literal(lit) => quote!( ext.#ident.#lit ),
                        DisplayParam::Ident(ident) => quote!( ext.#ident.#ident ),
                    })
                    .collect();
                quote!(#literal , #( #params ),*)
            } else {
                // display the variable using the Display trait
                quote!("{}", ext.#ident)
            };

            quote!({
                use imgui::im_str;
                ui.label_text(im_str!(#label), im_str!(#display));
            })
        }
    };

    Ok(tokens)
}
