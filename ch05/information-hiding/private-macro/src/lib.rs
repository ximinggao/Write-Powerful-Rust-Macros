use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, Ident, Type,
    parse_macro_input,
};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let fields = get_field_info(&ast);
    let private_fields = generate_private_fields(&fields);
    let methods = generate_methods(&fields);

    quote! {
        pub struct #name {
            #(#private_fields,)*
        }

        impl #name {
            #(#methods)*

            pub fn hello(&self) {
                println!("Hello, World");
            }
        }
    }
    .into()
}

fn get_field_info(ast: &DeriveInput) -> Vec<(&Ident, &Type)> {
    match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    }
    .iter()
    .map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let type_name = &f.ty;
        (field_name, type_name)
    })
    .collect()
}

fn generate_private_fields(fields: &Vec<(&Ident, &Type)>) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|f| {
            let (field_name, type_name) = f;
            quote! (
                #field_name: #type_name
            )
        })
        .collect()
}

fn generate_methods(fields: &Vec<(&Ident, &Type)>) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|f| {
            let (field_name, type_name) = f;
            let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

            quote! {
                fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            }
        })
        .collect()
}
