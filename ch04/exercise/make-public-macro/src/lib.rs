use proc_macro::TokenStream;
use quote::quote;
use std::iter::Map;
use syn::Data::{Enum, Struct};
use syn::Fields::Named;
use syn::Fields::Unnamed;
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::{
    DataEnum, DataStruct, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Variant,
    parse_macro_input,
};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let attributes = &ast.attrs;

    let basic_output = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let f = named_fields_public(named);
            generate_named_output(name, f)
        }
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let f = unnamed_fields_public(unnamed);
            generate_unnamed_output(name, f)
        }
        Enum(DataEnum { ref variants, .. }) => generate_enum_output(name, variants),
        _ => unimplemented!("only works for structs and enums"),
    };

    quote! {
        #(#attributes)*
        #basic_output
    }
    .into()
}

fn generate_enum_output(
    enum_name: Ident,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let as_iter = variants.into_iter();

    quote! {
        pub enum #enum_name {
            #(#as_iter,)*
        }
    }
}

fn generate_unnamed_output<'a>(
    struct_name: Ident,
    fields: Map<Iter<'a, Field>, fn(&'a Field) -> proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #struct_name (
            #(#fields,)*
        );
    }
}

fn unnamed_fields_public<'a>(
    fields: &'a Punctuated<Field, Comma>,
) -> Map<Iter<'a, Field>, fn(&Field) -> proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {pub #ty}
    })
}

fn generate_named_output<'a>(
    struct_name: Ident,
    fields: Map<Iter<'a, Field>, fn(&'a Field) -> proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #struct_name {
            #(#fields,)*
        }
    }
}

fn named_fields_public<'a>(
    fields: &'a Punctuated<Field, Comma>,
) -> Map<Iter<'a, Field>, fn(&Field) -> proc_macro2::TokenStream> {
    fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {pub #name: #ty}
    })
}
