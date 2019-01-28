use syn::Error;
use proc_macro2::Span;

pub fn already_defined(span: Span, attr: &str) -> Error {
    Error::new(span, format!("Attribute `{}` is already defined.", attr))
}

pub fn missin_attrib(span: Span, attr: &str) -> Error {
    Error::new(span, format!("Attribute `{}` is missing.", attr))
}

pub fn invalid_format(span: Span) -> Error {
    Error::new(span, "Invalid attribute format.")
}

pub fn unrecog_ident(span: Span, ident: String) -> Error {
    Error::new(span, format!("Unrecognized identifier: `{}`", ident))
}
