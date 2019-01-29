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

struct WidgetStruct {
    trait_ident: Ident,

    // struct holding parameters
    params_struct_ident: Ident,
    /// label field in Params struct
    params_label: Literal,
    /// rest of the Params fields
    params_rest: HashMap<String, Lit>,
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
        // This function maps into the field identifier, followed by all the imgui attributes
        //
        // We want to allow several #[imgui] tags per field a priori.
        // Once they are collected we can make sure they don't collide with each other (for example
        // mixing two widgets in a single field)
        // ```
        // struct MyStruct {
        //     #[imgui(...)]
        //     #[imgui(...)]
        //     foo: f32,
        //
        //     // ...
        // }
        // ```
        .map(|field| {
            // TODO support for unnamed attributes
            let ident = field.ident.clone().expect("Named field");

            // collect all imgui attributes from this field
            // (anything that contains the `imgui` path)
            let imgui: Result<Vec<_>, Error> = field.attrs.iter()
                // TODO check attribute style (Outer or Inner)
                .filter(|attr| attr.path.is_ident(Ident::new("imgui", attr.span())))

                // Parse attribute into metadata enums to follow rust conventions
                .map(|f| f.parse_meta())
                .collect();

            match imgui {
                Ok(imgui) => parse_field_body(ident, imgui),
                Err(error) => Err(error),
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(quote!{ #( #field_body );*})
}

/// Output source code for a given field identifier, and its metadata.
///
/// imgui only contains the legal variants of Meta
///   - Meta::World => `#[imgui]`
///   - Meta::List => `#[imgui(...)]`
fn parse_field_body(ident: Ident, imgui: Vec<Meta>) -> Result<TokenStream, Error> {
    let mut token_stream = TokenStream::new();
    for meta in imgui.iter() {
        match meta {
            // `#[imgui]` (checkboxes, labels)
            //
            // the widget will have the field identifier as label.
            Meta::Word(w) => {
                let label = Literal::string(ident.to_string().as_str());
                token_stream.extend(quote! {{
                    use imgui_ext_traits::Simple as Trait;
                    use imgui_ext_traits::params::SimpleParams as Params;
                    use imgui::im_str;
                    let params = Params { label: im_str!( #label ) };
                    Trait::build(ui, &mut ext.#ident, params);
                }});
            },
            // Parse a tag invoked like this:
            //   - `#[imgui( literal )]` <-- This raises build error
            //   - `#[imgui( meta )]`
            Meta::List(meta_list) => match meta_list.nested.first() {
                Some(Pair::End(single)) | Some(Pair::Punctuated(single, _)) => {
                    // only a single element is allowed
                    if meta_list.nested.len() != 1 {
                        return Err(Error::new(meta_list.span(), "Invalid annotation format."));
                    }

                    // At this point, we check if what we have is:
                    //   - `#[imgui(label = "...")]`
                    //   - `#[imgui(ident(...))]`
                    match single {
                        NestedMeta::Literal(_lit) => return Err(Error::new(meta_list.span(), "Invalid annotation format.")),

                        // `#[imgui(label = "...")]`
                        NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident: label, lit: Lit::Str(label_str), .. })) if label.to_string() == "label" => {
                            token_stream.extend(quote! {{
                                use imgui_ext_traits::Simple as Trait;
                                use imgui_ext_traits::params::SimpleParams as Params;
                                use imgui::im_str;
                                let params = Params { label: im_str!( #label_str ) };
                                Trait::build(ui, &mut ext.#ident, params);
                            }});
                        },

                        // Single identifier
                        //   - `#[imgui(input)]`
                        NestedMeta::Meta(Meta::Word(ident_widget)) => match ident_widget.to_string().as_str() {
                            "separator" => token_stream.extend(quote! {{
                                ui.separator();
                            }}),
                            "input" => {
                                let label = Literal::string(ident.to_string().as_str());
                                token_stream.extend(quote! {{
                                    use imgui_ext_traits::Input as Trait;
                                    use imgui_ext_traits::params::InputParams as Params;
                                    use imgui::im_str;
                                    let params = Params {
                                        label: im_str!( #label ),
                                        precission: None,
                                        step: None,
                                        step_fast: None,
                                    };
                                    Trait::build(ui, &mut ext.#ident, params);
                                }});
                            }
                            _ => unimplemented!("unimplemented 42"),
                        },

                        // check things like
                        //   - `#[imgui(slider(min = 0.0, max = 4.0))]`
                        NestedMeta::Meta(Meta::List(meta_list)) => {
                            unimplemented!("unimp foo");
                        },
                        _ => unimplemented!(),
                    }
                    //panic!("{}", meta_list.nested.len())
                },

                // Empty form. I'm not sure if this should raise an error...
                //   - `#[imgui()]`
                _ => return Err(Error::new(meta_list.span(), "Invalid annotation format.")),
            },
            // This was filtered out before hand
            //   - `#[imgui = ...]`
            //
            // TODO handle build error here?
            #[rustfmt::ignore]
            Meta::NameValue(sp) => return Err(Error::new(sp.span(), "Invalid annotation format.")),
        }

        /*
        let label = widget.params_label.clone();
        let params = widget.params_struct_ident.clone();
        let widget = widget.trait_ident.clone();
        token_stream.extend(quote! {{
            use imgui_ext_traits::params::#params as Params;
            use imgui_ext_traits::#widget as Widget;
            use imgui::im_str;
            let params = Params { label: im_str!( #label ) };
            Widget::build(ui, &mut ext.#ident, params);
        }});
        */
    }
    Ok(token_stream)
}

/// Parses the contents of a Meta::List. That is, what's inside of the `#[imgui( ... )]` tag.
/// The specific variants implemented by this function are:
///
///     - `#[imgui(ident)]` where `ident` is a single identifier
///       It will return an empty hashmap
///
///     - `#[imgui(ident( #(kₙ = vₙ ),* ))]` where `ident` is a single identifier, `(kₙ, vₙ)` are
///       `(identifier, literal)` pairs. In this case, it will return Some identifier, and the key
///       value pairs (It will raise a build error if the meta inside of the parenthesis is not NamedValue)
///
/// # Panics
/// It will panic if the metadata doesn't match any of the expected two formats.
fn parse_meta_list_content(meta: MetaList) -> Result<(Option<String>, HashMap<String, Lit>), Error> {
    match meta.nested.len() {
        // #[imgui()]
        0 => Ok((None, Default::default())),

        // #[imgui(foo = bar)]
        // #[imgui(foo(a = b, c = d))]
        1 => match meta.nested.first() {
            Some(Pair::End(single)) => {
                unimplemented!()
            },

            Some(Pair::Punctuated(_, sep)) => Err(unimplemented!()),
            None => unreachable!(),
        },

        _ => Err(Error::new(meta.span(), "<build error>")),

    }
}

fn parse_key_values(meta_list: MetaList) -> HashMap<Ident, Lit> {
    Default::default()
}
