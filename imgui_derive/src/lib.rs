#![recursion_limit = "128"]
extern crate proc_macro;

use std::collections::HashSet;
use std::string::ToString;

use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Lit, Meta, MetaList,
    MetaNameValue, NestedMeta, Type,
};

// error messages
const INVALID_FORMAT: &str = "Invalid annotation format.";
const MULTIPLE_ANNOT: &str = "Multiple `#[imgui(...)]` annotations on a single field.";
const STRUCT_SUPPORT: &str = "`ImGuiExt` derive is only supported on structs.";
const UNRECOG_MODE: &str = "Unexpected mode.";
const UNEXPECTED_PARAM: &str = "Unexpected parameter.";
const BULLET_MULTIPLE: &str = "bullet can't nest multiple things.";
const FIELD_ALREADY_DEFINED: &str = "Field already defined.";

macro_rules! tag {
    (
        $(#[$meta:meta])*
        struct $tag:ident {
            fields { $( $field:ident : Lit ,)* },
            optional { $( $opt_field:ident : Option<Lit> ,)* }
        }
    ) => {
        $(#[$meta])*
        struct $tag {
            $( $field : Lit ,)*
            $( $opt_field : Option<Lit> ,)*
        }
        impl $tag {
            fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
                $( let mut $field = None; )*
                $( let mut $opt_field = None; )*
                for param in list.nested.iter() {
                    match param {
                        NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => match ident.to_string().as_str() {
                            //"label" => widget.label = Some(lit.clone()),
                            $( stringify!($opt_field) => {
                                if $opt_field.is_some() {
                                    return Err(Error::new(ident.span(), FIELD_ALREADY_DEFINED));
                                }
                                $opt_field = Some(lit.clone());
                            },)*
                            $( stringify!($field) => {
                                if $field.is_some() {
                                    return Err(Error::new(ident.span(), FIELD_ALREADY_DEFINED));
                                }
                                $field = Some(lit.clone());
                            },)*
                            _ => return Err(Error::new(ident.span(), UNEXPECTED_PARAM)),
                        }
                        // TODO use proper span
                        _ => return Err(Error::new(list.span(), INVALID_FORMAT)),
                    }
                }
                Ok(Self {
                    $( $field : $field.ok_or(Error::new(list.span(), format!("Parameter `{}` missing.", stringify!($field) )))?,)*
                    $( $opt_field,)*
                })
            }
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

enum DisplayParam {
    Literal(Lit),
    Ident(Ident),
}

/// `#[imgui(label(label = "...", display = "...", x, y, z))]`
///                                                   ^---,
///                                                idents & literals
#[derive(Default)]
struct Display {
    label: Option<Lit>,
    display: Option<Lit>,
    params: Vec<DisplayParam>,
}

tag! {
    /// `#[imgui(checkbox(label = "..."))]`
    #[derive(Default)]
    struct Checkbox {
        fields {
            // none
        },
        optional {
            label: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    /// `#[imgui(input(label = "...", step = 1.0, step_fast = 1.0))]`
    #[derive(Default)]
    struct Input {
        fields {
            // none
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            step: Option<Lit>,
            step_fast: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    /// `#[imgui(slider(label = "...", min = 0.0, max = 4.0, format = "..."))]`
    struct Slider {
        fields {
            min: Lit,
            max: Lit,
        },
        optional {
            label: Option<Lit>,
            format: Option<Lit>,
            power: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct Drag {
        fields {
            // none
        },
        optional {
            label: Option<Lit>,
            min: Option<Lit>,
            max: Option<Lit>,
            speed: Option<Lit>,
            power: Option<Lit>,
            format: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    struct Button {
        fields {
            label: Lit,
        },
        optional {
            size: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    /// `#[imgui(bullet)]`
    /// `#[imgui(bullet(label = "Bullet list item"))]`
    #[derive(Default)]
    struct Bullet {
        fields {
        },
        optional {
            text: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct Nested {
        fields {
        },
        optional {
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct Text {
        fields {
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            size: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct Progress {
        fields {
        },
        optional {
            overlay: Option<Lit>,
            size: Option<Lit>,
        }
    }
}

tag! {
    struct Image {
        fields {
            size: Lit,
        },
        optional {
            border: Option<Lit>,
            tint: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct ColorButton {
        fields {
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            preview: Option<Lit>,
            size: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct ColorPicker {
        fields {
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            preview: Option<Lit>,
            mode: Option<Lit>,
            format: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    struct ColorEdit {
        fields {
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            preview: Option<Lit>,
            mode: Option<Lit>,
            format: Option<Lit>,
            catch: Option<Lit>,
        }
    }
}

enum Tag {
    None,
    Display(Display),
    Checkbox(Checkbox),
    Input(Input),
    Slider(Slider),
    Drag(Drag),
    Nested(Nested),
    Text(Text),
    Progress(Progress),
    Image(Image),
    Button(Button),

    ColorButton(ColorButton),
    ColorPicker(ColorPicker),
    ColorEdit(ColorEdit),

    /// `#[imgui(separator)]`
    Separator,
    /// `#[imgui(new_line)]`
    NewLine,

    BulletParent,
    Bullet(Bullet),
}

fn impl_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (body, catch_fields, catch_methods) = match input.data {
        Data::Struct(ref body) => struct_body(body.fields.clone()),
        _ => Err(Error::new(input.span(), STRUCT_SUPPORT)),
    }?;

    // crate a new type.
    // It should never generate a collision
    let event_type = Ident::new(&format!("{}ImGuiExtEvents", name.to_string()), input.span());

    Ok(quote! {
        pub struct #event_type {
            #catch_fields
        }
        impl #event_type {
            #catch_methods
        }
        impl #impl_generics imgui_ext::ImGuiExt for #name #ty_generics #where_clause {
            type Events = #event_type;
            fn imgui_ext(ui: &imgui::Ui, ext: &mut Self) -> Self::Events {
                // Because all fields are bool, it should be OK to zero the memory (right...?)
                let mut events: Self::Events = unsafe { std::mem::zeroed() };
                #body
                events
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
fn struct_body(fields: Fields) -> Result<(TokenStream, TokenStream, TokenStream), Error> {
    let mut input_methods: TokenStream = TokenStream::new();

    let mut input_fields: TokenStream = TokenStream::new();
    let mut input_fields_set = HashSet::new();

    let field_body = fields
        .iter()
        .flat_map(|field| {

            // TODO add support for unnamed attributes
            let ident = field.ident.clone().expect("Named field");
            let ty = &field.ty;

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
                            .map(|tag| {
                                emmit_tag_tokens(&ident,
                                                 &ty,
                                                 &attr,
                                                 &tag,
                                                 &mut input_fields,
                                                 &mut input_methods,
                                                 &mut input_fields_set)
                            })
                            .collect()
                    }
                },

                _ => unreachable!(),
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok((quote! { #( #field_body );*}, input_fields, input_methods))
}

/// meta is the whole (parsed) tag: `#[imgui]` or `#[imgui(...)]`
fn parse_meta(meta: Meta) -> Result<Vec<Tag>, Error> {
    match meta {
        // #[imgui = ...] Nope
        Meta::NameValue(named) => Err(Error::new(named.span(), INVALID_FORMAT)),
        // #[imgui], treated as an empty label
        Meta::Word(_) => Ok(vec![Tag::Display(Display::default())]),
        // #[imgui(meta_list)] (general)
        Meta::List(meta_list) => parse_meta_list(&meta_list),
    }
}

/// Parse the inside of `#[imgui(...)]`
///                              ^^^
/// Possible cases:
///   - `#[imgui(foo(...), bar(...))]`
///   - `#[imgui(foo(...),)]`
///   - `#[imgui(foo(...))]`
///   - `#[imgui(label = "...", display = "...", foo, bar)]`
fn parse_meta_list(meta_list: &MetaList) -> Result<Vec<Tag>, Error> {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum State {
        Init,
        //Label,
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
                tags.push(Tag::Display(parse_label(&meta_list)?));
                // any errors will have been reported by the previous call to `parse_label`.
                // At this point I can break out of the loop.
                break;
            }

            // widgets that can take no parameters
            (s, NestedMeta::Meta(Meta::Word(ident))) if s == State::Init || s == State::Tags => {
                match ident.to_string().as_str() {
                    "separator" => tags.push(Tag::Separator),
                    "new_line" => tags.push(Tag::NewLine),

                    "nested" => tags.push(Tag::Nested(Default::default())),
                    "text" => tags.push(Tag::Text(Default::default())),
                    "display" => tags.push(Tag::Display(Default::default())),
                    "checkbox" => tags.push(Tag::Checkbox(Default::default())),
                    "input" => tags.push(Tag::Input(Default::default())),
                    "drag" => tags.push(Tag::Drag(Default::default())),
                    "bullet" => tags.push(Tag::Bullet(Default::default())),
                    "progress" => tags.push(Tag::Progress(Default::default())),

                    // errors
                    "color" => return Err(Error::new(meta_list.span(), INVALID_FORMAT)),
                    "slider" => {
                        Tag::Slider(Slider::from_meta_list(&meta_list)?);
                    }
                    "button" => {
                        Tag::Button(Button::from_meta_list(&meta_list)?);
                    }
                    "image" => {
                        Tag::Image(Image::from_meta_list(&meta_list)?);
                    }

                    _ => return Err(Error::new(meta_list.span(), UNRECOG_MODE)),
                }
                state = State::Tags;
            }
            (s, NestedMeta::Meta(Meta::List(meta_list)))
                if s == State::Init || s == State::Tags =>
            {
                let tag = match meta_list.ident.to_string().as_str() {
                    "separator" => Tag::Separator,
                    "new_line" => Tag::NewLine,

                    "display" => Tag::Display(parse_label(&meta_list)?),
                    "nested" => Tag::Nested(Nested::from_meta_list(meta_list)?),
                    "checkbox" => Tag::Checkbox(Checkbox::from_meta_list(meta_list)?),
                    "input" => Tag::Input(Input::from_meta_list(meta_list)?),
                    "drag" => Tag::Drag(Drag::from_meta_list(meta_list)?),
                    "slider" => Tag::Slider(Slider::from_meta_list(meta_list)?),
                    "button" => Tag::Button(Button::from_meta_list(meta_list)?),
                    "text" => Tag::Text(Text::from_meta_list(meta_list)?),
                    "progress" => Tag::Progress(Progress::from_meta_list(meta_list)?),
                    "image" => Tag::Image(Image::from_meta_list(meta_list)?),

                    "color" => {
                        for nested in meta_list.nested.iter() {
                            match nested {
                                // One of:
                                //   - `color(edit)`
                                //   - `color(picker)`
                                //   - `color(button)`
                                NestedMeta::Meta(Meta::Word(ident)) => {
                                    match ident.to_string().as_str() {
                                        "edit" => tags.push(Tag::ColorEdit(Default::default())),
                                        "picker" => tags.push(Tag::ColorPicker(Default::default())),
                                        "button" => tags.push(Tag::ColorButton(Default::default())),

                                        // Compiler error
                                        _ => return Err(Error::new(ident.span(), UNRECOG_MODE)),
                                    }
                                }

                                // One of:
                                //   - `color(edit(...))`
                                //   - `color(picker(...))`
                                //   - `color(button(...))`
                                NestedMeta::Meta(Meta::List(color_meta_list)) => {
                                    match color_meta_list.ident.to_string().as_str() {
                                        "edit" => tags.push(Tag::ColorEdit(
                                            ColorEdit::from_meta_list(color_meta_list)?,
                                        )),
                                        "picker" => tags.push(Tag::ColorPicker(
                                            ColorPicker::from_meta_list(color_meta_list)?,
                                        )),
                                        "button" => tags.push(Tag::ColorButton(
                                            ColorButton::from_meta_list(color_meta_list)?,
                                        )),

                                        // Compiler error
                                        _ => {
                                            return Err(Error::new(
                                                color_meta_list.ident.span(),
                                                UNRECOG_MODE,
                                            ));
                                        }
                                    }
                                }

                                _ => return Err(Error::new(meta_list.span(), INVALID_FORMAT)),
                            }
                        }

                        Tag::None
                    }

                    // TODO refactor
                    // FIXME errors handling not clear enough
                    // bullet(toxt = "..") raises the wrong error
                    "bullet" => match Bullet::from_meta_list(meta_list) {
                        Ok(bullet) => Tag::Bullet(bullet),
                        Err(_err) => {
                            let mut inner = parse_meta_list(meta_list)?.into_iter();

                            match (inner.next(), inner.next()) {
                                (Some(first), None) => {
                                    tags.push(Tag::BulletParent);
                                    Ok(first)
                                }
                                (None, None) => Ok(Tag::BulletParent),
                                (Some(_first), Some(_second)) => {
                                    Err(Error::new(meta_list.span(), BULLET_MULTIPLE))
                                }
                                _ => Err(Error::new(meta_list.span(), INVALID_FORMAT)),
                            }?
                        }
                    },
                    _ => return Err(Error::new(meta_list.span(), UNRECOG_MODE)),
                };

                tags.push(tag);
                state = State::Tags;
            }
            _ => panic!(),
        }
    }
    Ok(tags)
}

/// Parse the contents of a label tag: `label(label = "...", display = "...", foo, bar)`
/// Asumes that `params.ident` is equal to "label"
fn parse_label(params: &MetaList) -> Result<Display, Error> {
    #[derive(Clone, Copy)]
    enum State {
        Init,
        Display,
    }

    let mut state = State::Init;
    let mut display = Display::default();

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

// TODO
#[allow(unused_macros)]
macro_rules! tag_match {
    (match $tag:ident {
        $(
            $ident:ident { $( $param:ident ),* }
        ),*
    }) => {
        match $tag {
            $(
                Tag::$ident( $ident { $( $param , )* } ) => { quote!() },
            )*
            _ => quote!(),
        }
    }
}

/// Output source code for a given field, a given attribute, and one of the parsed `Tag`s
///
/// For example, the this annotation: `#[imgui(label(...), input(...))]`
/// produces two tags: `Tag::Display` and `Tag::Input`.
///
/// This function needs to be called twice (once per Tag)
fn emmit_tag_tokens(
    ident: &Ident,
    _ty: &Type,
    attr: &Attribute,
    tag: &Tag,
    fields: &mut TokenStream,
    methods: &mut TokenStream,
    input_fields: &mut HashSet<String>,
) -> Result<TokenStream, Error> {
    /*
    let tokens = tag_match! {
        match tag {
            Progress { overlay, size }
        }
    };
    */
    let tokens = match tag {
        Tag::None => quote!(),
        Tag::Separator => quote!({ ui.separator() }),
        Tag::NewLine => quote!({ ui.new_line() }),
        Tag::Image(Image { size, border, tint }) => {
            let size = match size {
                Lit::Str(size) => Ident::new(&size.value(), size.span()),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };

            let mut params = quote! {
                use imgui_ext::image::ImageParams as Params;
                use imgui::{ImVec2, im_str};
                let mut params = Params {
                    size: ImVec2::from(#size()),
                    border: None,
                    tint: None,
                };
            };
            match tint {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.tint = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            match border {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.border = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            quote! {{
                use imgui_ext::image::Image;
                Image::build(ui, ext.#ident, { #params ; params });
            }}
        }
        Tag::Progress(Progress { overlay, size }) => {
            let mut params = quote! {
                use imgui_ext::progress::ProgressParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    overlay: None,
                    size: None,
                };
            };

            let ident_str = ident.to_string();
            match (overlay, ident_str.as_bytes()[0]) {
                (Some(Lit::Str(stri)), _) => {
                    params.extend(quote! {{ params.overlay = Some(im_str!(#stri)); }})
                }
                (None, b'_') => {}
                (None, _) => {
                    let overlay = Literal::string(&ident_str);
                    params.extend(quote! {{ params.overlay = Some(im_str!(#overlay)); }});
                }
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match size {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.size = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            quote! {{
                use imgui_ext::progress::Progress;
                Progress::build(ui, &ext.#ident, { #params; params });
            }}
        }
        Tag::ColorEdit(ColorEdit {
            label,
            flags,
            preview,
            mode,
            format,
            catch,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::color::ColorEditParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    flags: None,
                    preview: None,
                    mode: None,
                    format: None,
                };
            };

            match flags {
                Some(Lit::Str(flags)) => {
                    let ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match preview {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.preview = Some( imgui::ColorPreview::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match mode {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.mode = Some( imgui::ColorEditMode::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match format {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.format = Some( imgui::ColorFormat::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorEdit;
                let _ev = ColorEdit::build(ui, &mut ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::ColorPicker(ColorPicker {
            label,
            flags,
            preview,
            mode,
            format,
            catch,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::color::ColorPickerParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    flags: None,
                    preview: None,
                    mode: None,
                    format: None,
                };
            };

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match preview {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.preview = Some( imgui::ColorPreview::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match mode {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.mode = Some( imgui::ColorPickerMode::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match format {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        params.format = Some( imgui::ColorFormat::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorPicker;
                let _ev = ColorPicker::build(ui, &mut ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::ColorButton(ColorButton {
            label,
            flags,
            preview,
            size,
            catch,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::color::ColorButtonParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    flags: None,
                    size: None,
                    preview: None,
                };
            };

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match size {
                Some(Lit::Str(size)) => {
                    let ident = Ident::new(&size.value(), size.span());
                    params.extend(quote! { params.size = Some( imgui::ImVec2::from(#ident()) ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match preview {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                        use imgui::ColorPreview;
                        params.preview = Some( ColorPreview::#var );
                    }});
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorButton;
                let _ev = ColorButton::build(ui, ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::Text(Text {
            label,
            size,
            flags,
            catch,
        }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);
            let mut params = quote! {
                use imgui_ext::text::TextParams as Params;
                use imgui::im_str;
                let mut params = Params {
                    label: im_str!( #label ),
                    flags: None,
                    size: None,
                };
            };

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            match size {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.size = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            // TODO ??????1
            params.extend(quote!(params));

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote!({
                use imgui_ext::text::Text;
                let _ev = Text::build(ui, &mut ext.#ident, { #params });
                events.#catch_ident |= _ev;
            })
        }
        Tag::Input(Input {
            label,
            step,
            step_fast,
            flags,
            catch,
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

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            // TODO ????????
            params.extend(quote!(params));

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote!({
                use imgui_ext::input::Input;
                let _ev = Input::build(ui, &mut ext.#ident, { #params });
                events.#catch_ident |= _ev;
            })
        }
        Tag::Drag(Drag {
            label,
            min,
            max,
            speed,
            power,
            format,
            catch,
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

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            params.extend(quote!(params));
            quote!({
                use imgui_ext::drag::Drag;
                let _ev = Drag::build(ui, &mut ext.#ident, { #params });
                events.#catch_ident |= _ev;
            })
        }
        Tag::Button(Button { label, size, catch }) => {
            let label = match label {
                Lit::Str(stri) => Literal::string(&stri.value()),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };

            let catch = if let Some(Lit::Str(c)) = catch {
                let id = Ident::new(&c.value(), ident.span());
                let q = quote! { events.#id = _ev; };
                fields.extend(quote! { pub #id: bool , });
                methods.extend(quote! { pub fn #id(&self) -> bool { self.#id } });
                q
            } else {
                quote!()
            };

            if let Some(size) = size {
                let size_fn = match size {
                    Lit::Str(size) => Ident::new(&size.value(), size.span()),
                    _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
                };
                quote! {{
                    use imgui::ImVec2;
                    let _ev = ui.button( imgui::im_str!( #label ), { ImVec2::from(#size_fn()) } );
                    #catch
                }}
            } else {
                quote! {{
                    use imgui::ImVec2;
                    let _ev = ui.small_button( imgui::im_str!( #label ) );
                    #catch
                }}
            }
        }
        Tag::BulletParent => {
            quote! { ui.bullet(); }
        }
        Tag::Bullet(Bullet { text }) => {
            let text = match text {
                Some(Lit::Str(text)) => Some(text),
                None => None,
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };

            if let Some(text) = text {
                quote! {{
                    use imgui::im_str;
                    ui.bullet_text( im_str!( #text ));
                }}
            } else {
                quote! { ui.bullet(); }
            }
        }
        Tag::Slider(Slider {
            label,
            min,
            max,
            format,
            power,
            catch,
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
                Some(Lit::Str(value)) => {
                    params.extend(quote!(params.format = Some( im_str!(#value) );))
                }
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }
            match power {
                Some(Lit::Float(value)) => params.extend(quote!(params.power = Some(#value);)),
                None => {}
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            params.extend(quote!(params));
            quote!({
                use imgui_ext::slider::Slider;
                let _ev = Slider::build(ui, &mut ext.#ident, { #params });
                events.#catch_ident |= _ev;
            })
        }
        Tag::Checkbox(Checkbox { label, catch }) => {
            let label = match label {
                Some(Lit::Str(lab)) => lab.value(),
                None => ident.to_string(),
                _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
            };
            let label = Literal::string(&label);

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote!({
                use imgui_ext::checkbox::Checkbox;
                use imgui_ext::checkbox::CheckboxParams as Params;
                use imgui::im_str;
                let _ev = Checkbox::build(ui, &mut ext.#ident, Params { label: im_str!(#label) });
                events.#catch_ident |= _ev;
            })
        }
        Tag::Nested(Nested { catch }) => {
            let catch_ident = catch_ident_nested(
                attr,
                _ty,
                ident,
                catch.as_ref(),
                input_fields,
                fields,
                methods,
            )?;

            quote! {{
                use imgui_ext::ImGuiExt;
                let _ev = ImGuiExt::imgui_ext(ui, &mut ext.#ident);
                events.#catch_ident = _ev;
            }}
        }
        Tag::Display(Display {
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

fn catch_ident(
    attr: &Attribute,
    field: &Ident,
    catch: Option<&Lit>,
    field_set: &mut HashSet<String>,
    fields: &mut TokenStream,
    methods: &mut TokenStream,
) -> Result<Ident, Error> {
    match catch {
        Some(Lit::Str(lit)) => {
            let ident = Ident::new(&lit.value(), field.span());

            fields.extend(quote! { pub #ident: bool , });
            methods.extend(quote! { pub fn #ident(&self) -> bool { self.#ident } });

            Ok(ident)
        }

        // Use field identifier
        None => {
            if field_set.insert(field.to_string()) {
                fields.extend(quote! { pub #field: bool , });
                methods.extend(
                    quote! { #[inline(always)] pub fn #field(&self) -> bool { self.#field } },
                );
            }

            Ok(field.clone())
        }

        _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
    }
}

// TODO code repetition bad nono FIXME naw
fn catch_ident_nested(
    attr: &Attribute,
    _ty: &Type,
    field: &Ident,
    catch: Option<&Lit>,
    field_set: &mut HashSet<String>,
    fields: &mut TokenStream,
    methods: &mut TokenStream,
) -> Result<Ident, Error> {
    let tp = _ty.clone().into_token_stream();

    match catch {
        Some(Lit::Str(lit)) => {
            let ident = Ident::new(&lit.value(), field.span());

            fields.extend(quote! { pub #ident: imgui_ext::Events<#tp> , });
            methods.extend(
                quote! { pub fn #ident(&self) -> &imgui_ext::Events<#tp> { &self.#ident } },
            );

            Ok(ident)
        }

        // Use field identifier
        None => {
            if field_set.insert(field.to_string()) {
                fields.extend(quote! { pub #field: imgui_ext::Events<#tp> , });
                methods.extend(
                    quote! { pub fn #field(&self) -> &imgui_ext::Events<#tp> { &self.#field } },
                );
            }

            Ok(field.clone())
        }

        _ => return Err(Error::new(attr.span(), INVALID_FORMAT)),
    }
}
