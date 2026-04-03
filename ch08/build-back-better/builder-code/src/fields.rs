use quote::quote;
use syn::{Field, punctuated::Punctuated, token::Comma};

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

pub fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            pub fn #field_name(mut self, input: #field_type) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
        }
    })
}

fn get_name_and_type(f: &Field) -> (&Option<syn::Ident>, &syn::Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
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
