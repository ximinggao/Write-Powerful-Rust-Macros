use quote::{format_ident, quote, quote_spanned};
use syn::{
    Attribute, Field, Ident, LitStr, Token, Type, punctuated::Punctuated, spanned::Spanned,
    token::Comma,
};

pub fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! { #field_name: Option<#field_type> }
    })
}

pub fn builder_inits_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|f| {
            let (field_name, field_type) = get_name_and_type(f);
            let attr = extract_attribute_from_field(f, "builder").map(|a| {
                let mut content = None;

                a.parse_nested_meta(|m| {
                    if m.path.is_ident("rename") {
                        let _: Token![=] = m.input.parse().unwrap();
                        let name: LitStr = m.input.parse().unwrap();
                        content = Some(Ident::new(&name.value(), name.span()));
                    }
                    Ok(())
                })
                .unwrap();
                content.unwrap()
            });

            if let Some(attr) = attr {
                quote! {
                    pub fn #attr(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #field_name(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            }
        })
        .collect()
}

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
    use_defaults: bool,
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    fields
        .iter()
        .map(|f| {
            let (field_name, field_type) = get_name_and_type(f);
            let field_name_as_string = field_name.as_ref().unwrap().to_string();

            let uppercase_attr = extract_attribute_from_field(f, "uppercase");
            let to_add = if uppercase_attr.is_some() && matches_type(field_type, "String") {
                quote! {
                    .map(|v| v.to_uppercase())
                }
            } else if uppercase_attr.is_some() {
                return Err(syn::Error::new(
                    field_name.span(),
                    "uppercase attribute only works on String fields",
                ));
            } else {
                quote!()
            };

            let handle_type = if use_defaults {
                default_fallback()
            } else {
                panic_fallback(field_name_as_string)
            };

            Ok(quote! {
                #field_name: self.#field_name #to_add.#handle_type
            })
        })
        .collect()
}

fn matches_type(field_type: &Type, type_name: &str) -> bool {
    if let Type::Path(p) = field_type {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }
    false
}

pub fn optional_default_asserts(
    fields: &Punctuated<Field, Comma>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|f| {
            let name = snake_to_capitalized(f.ident.as_ref().unwrap().to_string());
            let ty = &f.ty;
            let assertion_ident = format_ident!("__{}DefaultAssertion", name);
            quote_spanned! {
                ty.span() => struct #assertion_ident where #ty: core::default::Default;
            }
        })
        .collect()
}

fn snake_to_capitalized(name: String) -> String {
    let parts = name.split('_');
    parts
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn panic_fallback(field_name_as_string: String) -> proc_macro2::TokenStream {
    quote! {
        expect(concat!("field not set: ", #field_name_as_string))
    }
}

fn default_fallback() -> proc_macro2::TokenStream {
    quote! {
        unwrap_or_default()
    }
}

fn get_name_and_type(f: &Field) -> (&Option<syn::Ident>, &Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a Attribute> {
    f.attrs.iter().find(|attr| attr.path().is_ident(name))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{FieldMutability, Ident, Path, PathSegment, Type, TypePath, Visibility};

    use super::*;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };

        let mut pun = Punctuated::new();
        pun.push(p);

        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });

        let f = Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example_field", Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);
        assert_eq!(actual_name.as_ref().unwrap().to_string(), "example_field");
    }
}
