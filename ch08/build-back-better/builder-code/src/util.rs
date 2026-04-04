use quote::format_ident;
use syn::Ident;

pub fn create_builder_ident(name: &Ident) -> Ident {
    format_ident!("{}Builder", name)
}

pub fn create_field_struct_name(builder: &Ident, field: &Ident) -> Ident {
    let name = field.to_string();
    let camel_case_name = name
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("");
    format_ident!("{}Of{}", camel_case_name, builder)
}
