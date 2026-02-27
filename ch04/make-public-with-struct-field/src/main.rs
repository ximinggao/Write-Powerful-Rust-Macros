use make_public_macro_with_struct_field::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    println!("Hello, world!");
}
