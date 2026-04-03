use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

use crate::fields::{
    builder_field_definitions, builder_inits_values, builder_methods, original_struct_setters,
};

mod fields;

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

            pub(crate) fn build(self) -> #name {
                #name {
                    #(#original_struct_set_fields,)*
                }
            }
        }

        impl #name {
            pub(crate) fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
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
}
