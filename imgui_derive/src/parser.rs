use std::collections::HashSet;
use std::fmt;
use std::string::ToString;

use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Lit, Meta, MetaList,
    MetaNameValue, NestedMeta, Type,
};

use crate::error::ErrorKind;

use super::error::Error;

macro_rules! tag {
    (
        $(#[$meta:meta])*
        pub struct $tag:ident {
            fields { $( $field:ident : Lit ,)* },
            optional { $( $opt_field:ident : Option<Lit> ,)* }
        }
    ) => {
        $(#[$meta])*
        pub struct $tag {
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
                                    return Err(Error::already_defined(ident.span()))
                                }
                                $opt_field = Some(lit.clone());
                            },)*
                            $( stringify!($field) => {
                                if $field.is_some() {
                                    return Err(Error::already_defined(ident.span()))
                                }
                                $field = Some(lit.clone());
                            },)*
                            _ => return Err(Error::unexpected_param(ident.span())),
                        }
                        // TODO use proper span
                        _ => return Err(Error::invalid_format(list.span())),
                    }
                }
                Ok(Self {
                    //$( $field : $field.ok_or(Error::new(list.span(), format!("Parameter `{}` missing.", stringify!($field) )))?,)*
                    $( $field : $field.ok_or(Error::missing_param(list.span(), stringify!($field)))?,)*
                    $( $opt_field,)*
                })
            }
        }

    }
}

pub enum DisplayParam {
    Literal(Lit),
    Ident(Ident),
}

/// `#[imgui(label(label = "...", display = "...", x, y, z))]`
///                                                   ^---,
///                                                idents & literals
#[derive(Default)]
pub struct Display {
    label: Option<Lit>,
    display: Option<Lit>,
    params: Vec<DisplayParam>,
}

impl Display {
    /// Parse the contents of a label tag: `label(label = "...", display = "...", foo, bar)`
    /// Asumes that `params.ident` is equal to "label"
    pub fn from_meta_list(params: &MetaList) -> Result<Self, Error> {
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

                (State::Init,
                 NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })))
                    if ident.to_string() == "label" =>
                {
                    display.label = Some(lit.clone());
                }

                (State::Init,
                 NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })))
                    if ident.to_string() == "display" =>
                {
                    display.display = Some(lit.clone());
                    state = State::Display;
                }

                _ => return Err(Error::invalid_format(params.span())),
            }
        }

        Ok(display)
    }
}

tag! {
    /// `#[imgui(checkbox(label = "..."))]`
    #[derive(Default)]
    pub struct Checkbox {
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
    pub struct Input {
        fields {
            // none
        },
        optional {
            label: Option<Lit>,
            flags: Option<Lit>,
            step: Option<Lit>,
            step_fast: Option<Lit>,
            catch: Option<Lit>,
            size: Option<Lit>,
        }
    }
}

tag! {
    /// `#[imgui(slider(label = "...", min = 0.0, max = 4.0, format = "..."))]`
    pub struct Slider {
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
    pub struct Drag {
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
    pub struct Button {
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
    pub struct Bullet {
        fields {
        },
        optional {
            text: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    pub struct Nested {
        fields {
        },
        optional {
            catch: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    pub struct Progress {
        fields {
        },
        optional {
            overlay: Option<Lit>,
            size: Option<Lit>,
        }
    }
}

tag! {
    pub struct Image {
        fields {
            size: Lit,
        },
        optional {
            border: Option<Lit>,
            tint: Option<Lit>,
            uv0: Option<Lit>,
            uv1: Option<Lit>,
        }
    }
}

tag! {
    pub struct ImageButton {
        fields {
            size: Lit,
        },
        optional {
            background: Option<Lit>,
            frame_padding: Option<Lit>,
            tint: Option<Lit>,
            uv0: Option<Lit>,
            uv1: Option<Lit>,
        }
    }
}

tag! {
    #[derive(Default)]
    pub struct ColorButton {
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
    pub struct ColorPicker {
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
    pub struct ColorEdit {
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
    pub struct Text {
        fields {
            lit: Lit,
        },
        optional {
        }
    }
}

impl Text {
    /// allows parsing:
    /// - text("...") -> literal form
    /// - text(lit = "...") -> regular form
    fn from_meta_list2(list: &MetaList) -> Result<Self, Error> {
        let mut iter = list.nested.iter();
        let first = iter.next();
        let second = iter.next();

        match (first, second) {
            // text("...")
            (Some(NestedMeta::Literal(Lit::Str(s))), None) => {
                Ok(Self { lit: Lit::Str(s.clone()) })
            }
            _ => Self::from_meta_list(list),
        }
    }
}

/// Render ui with the given style and color vars.
/// - vars(style = "...", color = "...", content(...))
#[derive(Default)]
pub struct Vars {
    /// An identifier to a local function returning the style variables to be pushed into the styles stack.
    style: Option<Lit>,
    /// An identifier to a local function returning the color variables to be pushed into the color stack.
    color: Option<Lit>,
    /// List of ui widgets that the pushed style and colors vars will be applied to.
    content: Option<Vec<Tag>>,
}

impl Vars {
    fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
        let mut style: Option<Lit> = None;
        let mut color: Option<Lit> = None;
        let mut content: Option<Vec<Tag>> = None;

        for meta in list.nested.iter() {
            match meta {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => {
                    match &ident.to_string()[..] {
                        "color" => {
                            if color.is_some() {
                                return Err(Error::already_defined(ident.span()));
                            } else {
                                color = Some(lit.clone());
                            }
                        }

                        "style" => {
                            if style.is_some() {
                                return Err(Error::already_defined(ident.span()));
                            } else {
                                style = Some(lit.clone());
                            }
                        }

                        _ => return Err(Error::unexpected_param(ident.span())),
                    }
                }

                NestedMeta::Meta(Meta::List(list)) if list.ident.to_string() == "content" => {
                    if content.is_some() {
                        return Err(Error::already_defined(list.span()));
                    } else {
                        content = Some(parse_meta_list(&list)?);
                    }
                }

                // Nope
                _ => return Err(Error::invalid_format(list.span())),
            }
        }

        Ok(Self { content, style, color })
    }
}

/// Allowed formats:
/// - `#[imgui(tree(label = "...", node(...))]`
/// - `#[imgui(tree(label = "...")]`
#[derive(Default)]
pub struct Tree {
    label: Option<Lit>,
    cond: Option<Lit>,
    flags: Option<Lit>,
    node: Option<Vec<Tag>>,
}

/// TODO define a macro to parse this kind of annotation
impl Tree {
    fn from_meta_list(list: &MetaList) -> Result<Self, Error> {
        let mut label: Option<Lit> = None;
        let mut cond: Option<Lit> = None;
        let mut flags: Option<Lit> = None;
        let mut node: Option<Vec<Tag>> = None;

        for meta in list.nested.iter() {
            match meta {
                // label = "..."
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) => {
                    match &ident.to_string()[..] {
                        "label" => {
                            if label.is_some() {
                                return Err(Error::already_defined(ident.span()));
                            } else {
                                label = Some(lit.clone());
                            }
                        }

                        "flags" => {
                            if flags.is_some() {
                                return Err(Error::already_defined(ident.span()));
                            } else {
                                flags = Some(lit.clone());
                            }
                        }

                        "cond" => {
                            if cond.is_some() {
                                return Err(Error::already_defined(ident.span()));
                            } else {
                                cond = Some(lit.clone());
                            }
                        }

                        _ => return Err(Error::unexpected_param(ident.span())),
                    }
                }

                // node(...)
                // we need to validate that the nested list contains a single item.
                NestedMeta::Meta(Meta::List(list)) if list.ident.to_string() == "node" => {
                    if node.is_some() {
                        return Err(Error::already_defined(list.span()));
                    } else {
                        node = Some(parse_meta_list(&list)?);
                    }
                }

                // Nope
                _ => panic!(),
                //_ => return Err(Error::new(list.span(), "fuck")),
            }
        }

        Ok(Self { label, node, cond, flags })
    }
}

pub enum Tag {
    None,
    Display(Display),
    Checkbox(Checkbox),
    Input(Input),
    Slider(Slider),
    Drag(Drag),
    Nested(Nested),
    Progress(Progress),
    Image(Image),
    ImageButton(ImageButton),
    Button(Button),

    ColorButton(ColorButton),
    ColorPicker(ColorPicker),
    ColorEdit(ColorEdit),

    /// `#[imgui(separator)]`
    Separator,
    /// `#[imgui(new_line)]`
    NewLine,
    /// - Litaral`: #[text(literal = "...")]`
    /// - Annotated field (AsRef<str>): `#[text(literal)]`
    Text(Text),
    TextWrap(Text),

    BulletParent,
    Bullet(Bullet),

    Tree(Tree),
    Vars(Vars),
}

/// meta is the whole (parsed) tag: `#[imgui]` or `#[imgui(...)]`
pub fn parse_meta(meta: Meta) -> Result<Vec<Tag>, Error> {
    match meta {
        // #[imgui = ...] Nope
        Meta::NameValue(named) => Err(Error::invalid_format(named.span())),
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
            (_, NestedMeta::Literal(_)) => return Err(Error::invalid_format(meta_list.span())),
            // Parse as a label(...)
            (State::Init, NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, .. })))
                if ident.to_string() == "label" || ident.to_string() == "display" =>
            {
                tags.push(Tag::Display(Display::from_meta_list(&meta_list)?));
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
                    "display" => tags.push(Tag::Display(Default::default())),
                    "checkbox" => tags.push(Tag::Checkbox(Default::default())),
                    "input" => tags.push(Tag::Input(Default::default())),
                    "drag" => tags.push(Tag::Drag(Default::default())),
                    "bullet" => tags.push(Tag::Bullet(Default::default())),
                    "progress" => tags.push(Tag::Progress(Default::default())),
                    //"text" => tags.push(Tag::Text(Default::default())),
                    //"text_wrap" => tags.push(Tag::TextWrap(Default::default())),
                    "tree" => tags.push(Tag::Tree(Default::default())),
                    "vars" => tags.push(Tag::Vars(Default::default())),

                    // errors
                    "color" => return Err(Error::invalid_format(meta_list.span())),
                    "text" => return Err(Error::invalid_format(meta_list.span())),
                    "text_wrap" => return Err(Error::invalid_format(meta_list.span())),
                    "slider" => {
                        Tag::Slider(Slider::from_meta_list(&meta_list)?);
                    }
                    "button" => {
                        Tag::Button(Button::from_meta_list(&meta_list)?);
                    }
                    "image" => {
                        Tag::Image(Image::from_meta_list(&meta_list)?);
                    }

                    _ => return Err(Error::unexpected_mode(meta_list.span())),
                }
                state = State::Tags;
            }
            (s, NestedMeta::Meta(Meta::List(meta_list)))
                if s == State::Init || s == State::Tags =>
            {
                let tag = match meta_list.ident.to_string().as_str() {
                    "separator" => Tag::Separator,
                    "new_line" => Tag::NewLine,

                    "display" => Tag::Display(Display::from_meta_list(&meta_list)?),
                    "nested" => Tag::Nested(Nested::from_meta_list(meta_list)?),
                    "checkbox" => Tag::Checkbox(Checkbox::from_meta_list(meta_list)?),
                    "input" => Tag::Input(Input::from_meta_list(meta_list)?),
                    "drag" => Tag::Drag(Drag::from_meta_list(meta_list)?),
                    "slider" => Tag::Slider(Slider::from_meta_list(meta_list)?),
                    "button" => Tag::Button(Button::from_meta_list(meta_list)?),
                    "progress" => Tag::Progress(Progress::from_meta_list(meta_list)?),
                    "image" => Tag::Image(Image::from_meta_list(meta_list)?),
                    "image_button" => Tag::ImageButton(ImageButton::from_meta_list(meta_list)?),
                    "text" => Tag::Text(Text::from_meta_list2(meta_list)?),
                    "text_wrap" => Tag::TextWrap(Text::from_meta_list2(meta_list)?),
                    "tree" => Tag::Tree(Tree::from_meta_list(meta_list)?),
                    "vars" => Tag::Vars(Vars::from_meta_list(meta_list)?),

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
                                        _ => return Err(Error::unexpected_mode(ident.span())),
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
                                                return Err(Error::unexpected_mode(color_meta_list.ident.span()))
                                            }
                                        }
                                }

                                _ => return Err(Error::invalid_format(meta_list.span())),
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
                                    Err(Error::bullet(meta_list.span()))
                                }
                                _ => Err(Error::invalid_format(meta_list.span())),
                            }?
                        }
                    },
                    _ => return Err(Error::unexpected_mode(meta_list.span())),
                };

                tags.push(tag);
                state = State::Tags;
            }
            _ => panic!(),
        }
    }
    Ok(tags)
}

/// Output source code for a given field, a given attribute, and one of the parsed `Tag`s
///
/// For example, the this annotation: `#[imgui(label(...), input(...))]`
/// produces two tags: `Tag::Display` and `Tag::Input`.
///
/// This function needs to be called twice (once per Tag)
pub fn emmit_tag_tokens(ident: &Ident,
                        _ty: &Type,
                        attr: &Attribute,
                        tag: &Tag,
                        fields: &mut TokenStream,
                        methods: &mut TokenStream,
                        input_fields: &mut HashSet<String>)
                        -> Result<TokenStream, Error> {
    let tokens = match tag {
        Tag::None => quote!(),
        Tag::Separator => quote!({ ui.separator() }),
        Tag::NewLine => quote!({ ui.new_line() }),
        Tag::Vars(Vars { color, style, content }) => {
            let mut tokens = TokenStream::new();
            if let Some(tags) = content.as_ref() {
                for tag in tags.iter() {
                    tokens.extend(emmit_tag_tokens(ident,
                                                   _ty,
                                                   attr,
                                                   tag,
                                                   fields,
                                                   methods,
                                                   input_fields)?);
                }
            }

            let tokens = match color {
                Some(Lit::Str(color)) => {
                    let ident = Ident::new(&color.value(), color.span());
                    quote! {
                        ui.with_color_vars(&#ident(), || { #tokens });
                    }
                }
                None => tokens,
                _ => return Err(Error::invalid_format(attr.span())),
            };

            let tokens = match style {
                Some(Lit::Str(style)) => {
                    let ident = Ident::new(&style.value(), style.span());
                    quote! {
                        ui.with_style_vars(&#ident(), || { #tokens });
                    }
                }
                None => tokens,
                _ => return Err(Error::invalid_format(attr.span())),
            };

            quote!( #tokens )
        }
        Tag::Tree(Tree { label, node, cond, flags }) => {
            let label = match label {
                Some(Lit::Str(s)) => s.value(),
                None => ident.to_string(),
                _ => return Err(Error::invalid_format(attr.span())),
            };
            let label = Literal::string(&label);

            // node contents
            let mut node_tokens = TokenStream::new();
            if let Some(tags) = node.as_ref() {
                for tag in tags.iter() {
                    node_tokens.extend(emmit_tag_tokens(ident,
                                                        _ty,
                                                        attr,
                                                        tag,
                                                        fields,
                                                        methods,
                                                        input_fields)?);
                }
            }

            let mut tree_tokens = TokenStream::default();

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    tree_tokens.extend(quote! {tree = tree.flags(#fn_ident());});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match cond {
                Some(Lit::Str(cond)) => {
                    let ident = Ident::new(&cond.value(), flags.span());
                    tree_tokens.extend(quote! {tree = tree.opened(true, imgui::ImGuiCond::#ident);});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            quote! {{
                let mut tree = imgui::TreeNode::new(ui, imgui::im_str!(#label));
                { #tree_tokens }
                tree.build(|| { #node_tokens })
            }}
        }
        Tag::ImageButton(ImageButton { size, background, frame_padding, uv0, uv1, tint }) => {
            let size = match size {
                Lit::Str(size) => Ident::new(&size.value(), size.span()),
                _ => return Err(Error::invalid_format(attr.span())),
            };

            let mut params = quote! {
                use imgui_ext::image_button::ImageButtonParams as Params;
                use imgui::{ImVec2, im_str};
                let mut params = Params {
                    size: ImVec2::from(#size()),
                    background: None,
                    frame_padding: None,
                    tint: None,
                    uv0: None,
                    uv1: None,
                };
            };
            match frame_padding {
                Some(Lit::Str(value_str)) => {
                    let value = value_str.value()
                                         .parse()
                                         .map(Literal::i32_unsuffixed)
                                         .expect("frame_padding expected to be numeric (i32).");
                    params.extend(quote!(params.frame_padding = Some(#value);));
                }
                Some(Lit::Int(value)) => {
                    params.extend(quote!(params.frame_padding = Some(#value);));
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match uv0 {
                Some(Lit::Str(uv0)) => {
                    let fn_ident = Ident::new(&uv0.value(), uv0.span());
                    params.extend(
                        quote! {{ params.uv0 = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match uv1 {
                Some(Lit::Str(uv1)) => {
                    let fn_ident = Ident::new(&uv1.value(), uv1.span());
                    params.extend(
                        quote! {{ params.uv1 = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match tint {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.tint = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match background {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.background = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            quote! {{
                use imgui_ext::image::Image;
                Image::build(ui, ext.#ident, { #params ; params });
            }}
        }
        Tag::Image(Image { size, border, tint, uv0, uv1 }) => {
            let size = match size {
                Lit::Str(size) => Ident::new(&size.value(), size.span()),
                _ => return Err(Error::invalid_format(attr.span())),
            };

            let mut params = quote! {
                use imgui_ext::image::ImageParams as Params;
                use imgui::{ImVec2, im_str};
                let mut params = Params {
                    size: ImVec2::from(#size()),
                    border: None,
                    tint: None,
                    uv0: None,
                    uv1: None,
                };
            };
            match uv0 {
                Some(Lit::Str(uv0)) => {
                    let fn_ident = Ident::new(&uv0.value(), uv0.span());
                    params.extend(
                        quote! {{ params.uv0 = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match uv1 {
                Some(Lit::Str(uv1)) => {
                    let fn_ident = Ident::new(&uv1.value(), uv1.span());
                    params.extend(
                        quote! {{ params.uv1 = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match tint {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.tint = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match border {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.border = Some( imgui::ImVec4::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match size {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.size = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            quote! {{
                use imgui_ext::progress::Progress;
                Progress::build(ui, &ext.#ident, { #params; params });
            }}
        }
        Tag::Text(Text { lit }) => {
            match lit {
                //Some(Lit::Str(lit)) => quote! { ui.text_wrapped(imgui::im_str!(#lit)); },
                Lit::Str(lit) => quote! { ui.text(#lit); },

                // Invalid format. Raise compiler error.
                _ => return Err(Error::invalid_format(attr.span())),
            }
        }
        Tag::TextWrap(Text { lit }) => {
            match lit {
                Lit::Str(lit) => quote! { ui.text_wrapped(imgui::im_str!(#lit)); },

                // Invalid format. Raise compiler error.
                _ => return Err(Error::invalid_format(attr.span())),
            }
        }
        Tag::ColorEdit(ColorEdit { label, flags, preview, mode, format, catch }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match preview {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.preview = Some( imgui::ColorPreview::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match mode {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.mode = Some( imgui::ColorEditMode::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match format {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.format = Some( imgui::ColorFormat::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorEdit;
                let _ev = ColorEdit::build(ui, &mut ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::ColorPicker(ColorPicker { label, flags, preview, mode, format, catch }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match preview {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.preview = Some( imgui::ColorPreview::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match mode {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.mode = Some( imgui::ColorPickerMode::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match format {
                Some(Lit::Str(c)) => {
                    let var = Ident::new(&c.value(), ident.span());
                    params.extend(quote! {{
                                      params.format = Some( imgui::ColorFormat::#var );
                                  }});
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorPicker;
                let _ev = ColorPicker::build(ui, &mut ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::ColorButton(ColorButton { label, flags, preview, size, catch }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match size {
                Some(Lit::Str(size)) => {
                    let ident = Ident::new(&size.value(), size.span());
                    params.extend(quote! { params.size = Some( imgui::ImVec2::from(#ident()) ); });
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }

            let catch_ident =
                catch_ident(attr, ident, catch.as_ref(), input_fields, fields, methods)?;

            quote! {{
                use imgui_ext::color::ColorButton;
                let _ev = ColorButton::build(ui, ext.#ident, { #params ; params });
                events.#catch_ident |= _ev;
            }}
        }
        Tag::Input(Input { label, step, step_fast, flags, catch, size }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                // TODO proper error span
                _ => return Err(Error::invalid_format(attr.span())),
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
                    size: None,
                };
            };

            match size {
                Some(Lit::Str(size)) => {
                    let fn_ident = Ident::new(&size.value(), size.span());
                    params.extend(
                        quote! {{ params.size = Some( imgui::ImVec2::from(#fn_ident()) ); }},
                    );
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match step {
                Some(Lit::Float(step)) => params.extend(quote! { params.step = Some(#step); }),
                Some(Lit::Int(step)) => params.extend(quote! { params.step = Some(#step); }),
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match step_fast {
                Some(Lit::Float(step)) => params.extend(quote! { params.step_fast = Some(#step); }),
                Some(Lit::Int(step)) => params.extend(quote! { params.step_fast = Some(#step); }),
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match flags {
                Some(Lit::Str(flags)) => {
                    let fn_ident = Ident::new(&flags.value(), flags.span());
                    params.extend(quote! { params.flags = Some( #fn_ident() ); });
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
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
        Tag::Drag(Drag { label, min, max, speed, power, format, catch }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                _ => return Err(Error::invalid_format(attr.span())),
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

            match min {
                Some(Lit::Float(min)) => params.extend(quote!(params.min = Some(#min);)),
                Some(Lit::Int(min)) => params.extend(quote!(params.min = Some(#min);)),
                Some(Lit::Str(min)) => {
                    let min_i64 = min.value().parse().map(Literal::i64_unsuffixed);
                    let min_f64 = min.value().parse().map(Literal::f64_unsuffixed);
                    match (min_i64, min_f64) {
                        (Err(_), Ok(min)) => params.extend(quote!(params.min = Some(#min);)),
                        (Ok(min), _) => params.extend(quote!(params.min = Some(#min);)),
                        _ => return Err(Error::parsing_error(min.span())),
                    }
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match max {
                Some(Lit::Float(max)) => params.extend(quote!(params.max = Some(#max);)),
                Some(Lit::Int(max)) => params.extend(quote!(params.max = Some(#max);)),
                Some(Lit::Str(max)) => {
                    let max_i64 = max.value().parse().map(Literal::i64_unsuffixed);
                    let max_f64 = max.value().parse().map(Literal::f64_unsuffixed);
                    match (max_i64, max_f64) {
                        (Err(_), Ok(max)) => params.extend(quote!(params.max = Some(#max);)),
                        (Ok(max), _) => params.extend(quote!(params.max = Some(#max);)),
                        _ => return Err(Error::parsing_error(max.span())),
                    }
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }

            match speed {
                Some(Lit::Float(value)) => params.extend(quote! { params.speed = Some(#value); }),
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match power {
                Some(Lit::Float(value)) => params.extend(quote! { params.power = Some(#value); }),
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match format {
                Some(Lit::Str(value)) => {
                    params.extend(quote!(params.format = Some(im_str!(#value));))
                }
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
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
                    _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
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
        Tag::Slider(Slider { label, min, max, format, power, catch }) => {
            let label = match label {
                Some(Lit::Str(stri)) => stri.value(),
                None => ident.to_string(),
                _ => return Err(Error::invalid_format(attr.span())),
            };
            let label = Literal::string(&label);
            let min_max = match (min, max) {
                (Lit::Int(min), Lit::Int(max)) => quote! { min: #min, max: #max },
                (Lit::Float(min), Lit::Float(max)) => quote! { min: #min, max: #max },
                (Lit::Str(min), Lit::Int(max)) => {
                    let min = min.value()
                                 .parse()
                                 .map(Literal::i64_unsuffixed)
                                 .map_err(|_| Error::parsing_error(min.span()))?;

                    quote! { min: #min, max: #max }
                }
                (Lit::Str(min), Lit::Float(max)) => {
                    let min = min.value()
                                 .parse()
                                 .map(Literal::f64_unsuffixed)
                                 .map_err(|_| Error::parsing_error(min.span()))?;

                    quote! { min: #min, max: #max }
                }
                (Lit::Int(min), Lit::Str(max)) => {
                    let max = max.value()
                                 .parse()
                                 .map(Literal::i64_unsuffixed)
                                 .map_err(|_| Error::parsing_error(max.span()))?;

                    quote! { min: #min, max: #max }
                }
                (Lit::Float(min), Lit::Str(max)) => {
                    let max = max.value()
                                 .parse()
                                 .map(Literal::f64_unsuffixed)
                                 .map_err(|_| Error::parsing_error(max.span()))?;

                    quote! { min: #min, max: #max }
                }
                (Lit::Str(min), Lit::Str(max)) => {
                    let min_f64 = min.value().parse().map(Literal::f64_unsuffixed);
                    let max_f64 = max.value().parse().map(Literal::f64_unsuffixed);
                    let min_i32 = min.value().parse().map(Literal::i64_unsuffixed);
                    let max_i32 = max.value().parse().map(Literal::i64_unsuffixed);

                    match (min_f64, max_f64, min_i32, max_i32) {
                        (_, _, Ok(min), Ok(max)) => quote! { min: #min, max: #max },
                        (Ok(min), Ok(max), _, _) => quote! { min: #min, max: #max },

                        // Nope
                        _ => return Err(Error::parsing_error(max.span())),
                    }
                }
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
            }
            match power {
                Some(Lit::Float(value)) => params.extend(quote!(params.power = Some(#value);)),
                None => {}
                _ => return Err(Error::invalid_format(attr.span())),
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
                _ => return Err(Error::invalid_format(attr.span())),
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
            let catch_ident = catch_ident_nested(attr,
                                                 _ty,
                                                 ident,
                                                 catch.as_ref(),
                                                 input_fields,
                                                 fields,
                                                 methods)?;

            quote! {{
                use imgui_ext::ImGuiExt;
                let _ev = ImGuiExt::imgui_ext(ui, &mut ext.#ident);
                events.#catch_ident = _ev;
            }}
        }
        Tag::Display(Display { label, display, params }) => {
            let label = match label {
                Some(Lit::Str(lab)) => lab.value(),
                None => ident.to_string(),
                _ => return Err(Error::invalid_format(attr.span())),
            };
            let label = Literal::string(&label);

            let display = match display {
                Some(Lit::Str(disp)) => Some(disp.value()),
                None => None,
                _ => return Err(Error::invalid_format(attr.span())),
            };

            let display = if let Some(display) = display {
                let literal = Literal::string(display.as_str());
                let params: Vec<_> =
                    params.into_iter()
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

fn catch_ident(attr: &Attribute,
               field: &Ident,
               catch: Option<&Lit>,
               field_set: &mut HashSet<String>,
               fields: &mut TokenStream,
               methods: &mut TokenStream)
               -> Result<Ident, Error> {
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

        _ => return Err(Error::invalid_format(attr.span())),
    }
}

// TODO code repetition bad nono FIXME naw
fn catch_ident_nested(attr: &Attribute,
                      _ty: &Type,
                      field: &Ident,
                      catch: Option<&Lit>,
                      field_set: &mut HashSet<String>,
                      fields: &mut TokenStream,
                      methods: &mut TokenStream)
                      -> Result<Ident, Error> {
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

        _ => return Err(Error::invalid_format(attr.span())),
    }
}
