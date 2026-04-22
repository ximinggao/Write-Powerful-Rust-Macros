use std::{collections::HashMap, fs};

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse_macro_input;

#[cfg(feature = "struct")]
use syn::DeriveInput;

use crate::{input::ConfigInput, output::generate_config_struct};

mod input;
mod output;
#[cfg(feature = "struct")]
mod struct_output;

#[cfg(feature = "functional")]
#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(item);

    match find_yaml_values(input) {
        Ok(values) => generate_config_struct(values).into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn find_yaml_values(input: ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
    let file_name = input
        .path
        .unwrap_or_else(|| "./configuration/config.yaml".to_string());
    let file = fs::File::open(&file_name).map_err(|err| {
        syn::Error::new(
            Span::call_site(),
            format!("could not read config with path {}: {}", &file_name, err),
        )
    })?;
    serde_yaml::from_reader(file).map_err(|e| syn::Error::new(Span::call_site(), e.to_string()))
}

#[cfg(feature = "struct")]
#[proc_macro_attribute]
pub fn config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(attr);
    let ast: DeriveInput = parse_macro_input!(item);

    match find_yaml_values(input) {
        Ok(values) => struct_output::generate_annotation_struct(ast, values).into(),
        Err(e) => e.into_compile_error().into(),
    }
}
