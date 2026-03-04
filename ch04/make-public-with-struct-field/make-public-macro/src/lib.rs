use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed, FieldsUnnamed, Ident,
    Type, parse_macro_input,
};

struct StructField {
    name: Option<Ident>,
    ty: Type,
}

impl StructField {
    fn new(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().map(|f| f.clone()),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        if n.is_some() {
            quote!(pub #n: #t).to_tokens(tokens);
        } else {
            quote!(pub #t).to_tokens(tokens);
        }
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    // eprintln!("{:#?}", &ast);

    let name = ast.ident;
    let (fields, is_named) = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => (named, true),
        Struct(DataStruct {
            fields: syn::Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => (unnamed, false),
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(StructField::new);

    let public_version = if is_named {
        quote! {
            pub struct #name {
                #(#builder_fields,)*
            }
        }
    } else {
        quote! {
            pub struct #name (
                #(#builder_fields,)*
            );
        }
    };
    public_version.into()
}
