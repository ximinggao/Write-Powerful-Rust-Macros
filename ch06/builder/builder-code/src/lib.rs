use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    punctuated::Punctuated, token::Comma,
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for struct with named fields"),
    };

    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_inits_values(fields);
    let builder_methods = builder_methods(fields);
    let original_struct_set_fields = original_struct_setters(fields);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(&self) -> #name {
                #name {
                    #(#original_struct_set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
}

fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! { #field_name: Option<#field_type> }
    })
}

fn builder_inits_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name: self.#field_name.as_ref()
                .expect(&format!("{} is not set", #field_name_as_string))
                .to_string()
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
    use super::*;

    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
            struct Gleipnir {}
        };
        let output = create_builder(input);
        assert!(output.to_string().contains("GleipnirBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder {}
        };
        let output = create_builder(input);
        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let actual = create_builder(input);
        let derived: DeriveInput = syn::parse2(actual).unwrap();
        assert_eq!(derived.ident.to_string(), "StructWithNoFieldsBuilder");
    }
}
