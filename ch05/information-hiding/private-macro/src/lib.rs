use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data::Struct, DataStruct, DeriveInput, FieldsNamed, Ident, parse_macro_input};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item_as_stream: proc_macro2::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generate_methods(&ast);

    quote! {
        #item_as_stream

        impl #name {
            #(#methods)*
        }
    }
    .into()
}

fn generate_methods(ast: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let named_fields = match ast.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    named_fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let type_name = &f.ty;
            let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

            quote! {
                fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            }
        })
        .collect()
}
