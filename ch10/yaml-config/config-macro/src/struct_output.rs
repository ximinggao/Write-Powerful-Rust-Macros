use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

pub fn generate_annotation_struct(
    input: syn::DeriveInput,
    yaml_values: HashMap<String, String>,
) -> TokenStream {
    let attributes = &input.attrs;
    let name = &input.ident;
    let fields = generate_fields(&yaml_values);
    let inits = generate_inits(&yaml_values);

    quote! {
        #(#attributes)*
        pub struct #name {
            #(#fields,)*
        }

        impl #name {
            pub fn new() -> Self {
                #name {
                    #(#inits,)*
                }
            }
        }
    }
}

fn generate_fields(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values
        .iter()
        .map(|v| {
            let key = Ident::new(v.0, Span::call_site());
            quote! {
                pub #key: String
            }
        })
        .collect()
}

fn generate_inits(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values
        .iter()
        .map(|v| {
            let key = Ident::new(v.0, Span::call_site());
            let value = v.1;
            quote! {
                #key: #value.to_string()
            }
        })
        .collect()
}
