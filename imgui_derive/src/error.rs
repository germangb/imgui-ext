use proc_macro2::TokenStream;
use std::fmt;
use syn::export::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidFormat,
    Multiple,
    NonStruct,
    UnexpectedMode,
    UnexpectedParam,
    Bullet,
    AlreadyDefined,
    ParseError,
    MissingParam(&'static str),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidFormat => write!(fmt, "Invalid annotation format."),
            ErrorKind::Multiple => write!(fmt, "Multiple annotations per field."),
            ErrorKind::NonStruct => write!(
                fmt,
                "ImGuiExt macro is only supported for structs with named fields."
            ),
            ErrorKind::UnexpectedMode => write!(fmt, "Unexpected annotation."),
            ErrorKind::UnexpectedParam => write!(fmt, "Unexpected parameter."),
            ErrorKind::Bullet => write!(
                fmt,
                "Multiple nested annotations inside of a bullet list element."
            ),
            ErrorKind::AlreadyDefined => write!(fmt, "Field is defined already."),
            ErrorKind::ParseError => write!(fmt, "String parsing error."),
            ErrorKind::MissingParam(p) => write!(fmt, "Parameter `{}` missing.", p),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

impl From<Error> for syn::Error {
    fn from(err: Error) -> Self {
        syn::Error::new(err.span, err.kind)
    }
}

impl Error {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn to_compile_error(&self) -> TokenStream {
        syn::Error::new(self.span.clone(), self.kind).to_compile_error()
    }

    pub fn missing_param(span: Span, name: &'static str) -> Self {
        Self {
            kind: ErrorKind::MissingParam(name),
            span,
        }
    }

    pub fn invalid_format(span: Span) -> Self {
        Self {
            kind: ErrorKind::InvalidFormat,
            span,
        }
    }

    /// Multiple annotations per field.
    pub fn multiple(span: Span) -> Self {
        Self {
            kind: ErrorKind::Multiple,
            span,
        }
    }

    /// No support for anything other that structs with names fields
    pub fn non_struct(span: Span) -> Self {
        Self {
            kind: ErrorKind::NonStruct,
            span,
        }
    }

    /// Unexpected annotation mode.
    pub fn unexpected_mode(span: Span) -> Self {
        Self {
            kind: ErrorKind::UnexpectedMode,
            span,
        }
    }

    pub fn unexpected_param(span: Span) -> Self {
        Self {
            kind: ErrorKind::UnexpectedParam,
            span,
        }
    }

    pub fn bullet(span: Span) -> Self {
        Self {
            kind: ErrorKind::Bullet,
            span,
        }
    }

    /// Annotation param defined already
    pub fn already_defined(span: Span) -> Self {
        Self {
            kind: ErrorKind::AlreadyDefined,
            span,
        }
    }

    pub fn parsing_error(span: Span) -> Self {
        Self {
            kind: ErrorKind::ParseError,
            span,
        }
    }
}
