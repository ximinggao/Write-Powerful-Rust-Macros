use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

use crate::fields::{
    builder_definition, builder_impl_for_struct, builder_methods, marker_trait_and_structs,
};

mod fields;
mod util;

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for struct with named fields"),
    };

    let builder = builder_definition(&name, fields);
    let builder_method_for_struct = builder_impl_for_struct(&name, fields);
    let marker_and_structs = marker_trait_and_structs(&name, fields);
    let builder_methods = builder_methods(&name, fields);

    quote! {
        #builder
        #builder_method_for_struct
        #marker_and_structs
        #builder_methods
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
            struct Gleipnir { name: String }
        };
        let output = create_builder(input);
        assert!(output.to_string().contains("GleipnirBuilder"));
    }
}
