pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{} {}!", greeting, name)
}

macro_rules! greeting {
    ($name:literal) => {
        base_greeting_fn($name, "Hello")
    };
    ($name:literal,$greeting:literal) => {
        base_greeting_fn($name, $greeting)
    };
    (test $name:literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returning the default greeting");
        base_greeting_fn($name, "Hello")
    }};
}

macro_rules! generate_get_value_method {
    ($struct_type:ident) => {
        generate_get_value_method!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}
