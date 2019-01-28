extern crate proc_macro;

use std::string::ToString;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Meta, MetaNameValue, NestedMeta, parse_macro_input, Token};
use syn::parse::{Error, Parse};
use syn::punctuated::Pair;
use syn::spanned::Spanned;

const INVALID_ATTR_FORMAT: &str = "Invalid attribute format";
const INVALID_IDENT: &str = "Invalid identifier token";
const UNSUPPORTED_META: &str = "Unsupported metadata";

enum ImGuiAttr {
    // - `#[imgui]`
    // - `#[imgui(label = "...")]`
    Simple {
        label: Option<String>,
    },

    // `#[imgui(input(precission = f32, label = "...")]`
    Input {
        label: Option<String>,
        precission: f32,
        step: Option<f32>,
        step_fast: Option<f32>,
    },

    // `#[imgui(slider(min = f32, height = f32))]`
    Slider {
        label: Option<String>,
        min: f32,
        max: f32,
    },
}

impl ImGuiAttr {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        unimplemented!()
    }

    fn into_token_stream(self, ident: &Ident) -> Result<TokenStream, Error> {
        match self {
            ImGuiAttr::Simple { label } => {
                let label = label.unwrap_or(ident.to_string());
                let literal = Literal::string(&label);

                Ok(quote! {{
                    use imgui::im_str;
                    ui.checkbox(im_str!( #literal ), &mut ext.#ident);
                }})
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
        _ => Err(Error::new(input.span(), "Only structs"))
    }?;

    Ok(quote! {
        impl #impl_generics imgui_ext_traits::ImGuiExt for #name #ty_generics #where_clause {
            fn imgui_ext(ui: &imgui::Ui, ext: &mut Self) {
                #body
            }
        }
    })
}

// Parse the tokens between the parenthesis of a MetaList, that is, what
// is inside the parenthesis of this annotation:
//
//  - #[imgui( ... )]
//            ^^^^^
fn parse_meta_list(name: &Ident, meta: &syn::MetaList) -> Result<ImGuiAttr, Error> {
    let nested = &meta.nested;
    if nested.len() != 1 {
        return Err(Error::new(meta.span(), INVALID_ATTR_FORMAT));
    }

    match nested.first() {
        // TODO
        // Do we want to support both:
        // - `#[imgui( foo )]` and
        // - `#[imgui( foo, )]` (with trailing comma)
        // or just the first one?
        Some(Pair::End(attr)) | Some(Pair::Punctuated(attr, _)) => {
            match attr {
                // This is not allowed (literal inside of the annotation)
                //  - `#[imgui("...")]`
                NestedMeta::Literal(lit) => {
                    Err(Error::new(meta.span(), INVALID_ATTR_FORMAT))
                },

                NestedMeta::Meta(meta) => {
                    match meta {
                        // We should have
                        //  - `#[imgui(label = "...")]`
                        Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(label), .. }) => {
                            if ident.to_string() == "label" {
                                Ok(ImGuiAttr::Simple {
                                    label: Some(label.value()),
                                })
                            } else {
                                Err(Error::new(ident.span(), INVALID_IDENT))
                            }
                        },

                        // Check things like:
                        //  - `#[imgui(input( ... ))]`
                        //  - `#[imgui(progress( ... ))]`
                        //  - `#[imgui(slider( ... ))]`
                        Meta::List(meta_list) => match meta_list.ident.to_string().as_str() {
                            "input" => unimplemented!("input"),
                            "progress" => unimplemented!("progress"),
                            "slider" => unimplemented!("slider"),
                            _ => Err(Error::new(meta_list.span(), UNSUPPORTED_META)),
                        },

                        _ => Err(Error::new(name.span(), INVALID_ATTR_FORMAT)),
                    }
                }
            }
        },
        _ => {
            // FIXME
            Err(Error::new(meta.span(), INVALID_ATTR_FORMAT))
        }
    }
}

// #[imgui( ... )]
//   ^^^^^^^^^^^^
fn parse_meta(name: &Ident, meta: &Meta) -> Result<ImGuiAttr, Error> {
    use syn::MetaList;

    match meta {
        // At this point we know we have this:
        // #[imgui]
        &Meta::Word(_) => {
            Ok(ImGuiAttr::Simple { label: None })
        },

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
        &Meta::NameValue(_) => {
            Err(Error::new(meta.span(), INVALID_ATTR_FORMAT))
        },
    }
}

fn imgui_body_fields(fields: Fields) -> Result<TokenStream, Error> {
    let field_assign = fields.iter().map(|field| {

        // collect all #[imgui] attributes
        let mut attributes = field.attrs.iter()
            .filter(is_imgui_attr)
            .map(Attribute::parse_meta)
            .collect::<Result<Vec<_>, Error>>()?;

        // Only one `#[imgui]` attribute per field is allowed.
        // If we encounter more than one, raise a compilation error
        if attributes.is_empty() {
            return Ok(TokenStream::new());
        } else if attributes.len() > 1 {
            return Err(Error::new(field.span(), "Only one `#[imgui]` tag per attribute is allowed"));
        }

        // At this point, we are parsing the following attribute:
        //
        // #[imgui( ... )]
        //   ^^^^^^^^^^^^
        // Therefore it is safe to unwrap
        let attr_meta = attributes.get(0).unwrap();
        let ident = field.ident.as_ref().unwrap();

        parse_meta(&ident, attr_meta)?.into_token_stream(&ident)
    }).collect::<Result<Vec<_>, Error>>()?;
    Ok(quote! {
        #( #field_assign );*
    })
}

fn is_imgui_attr(attr: &&Attribute) -> bool {
    attr.path.is_ident(Ident::new("imgui", Span::call_site()))
}
