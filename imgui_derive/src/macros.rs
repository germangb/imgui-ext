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

