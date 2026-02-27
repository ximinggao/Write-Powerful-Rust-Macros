use make_public_macro_using_parse::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    println!("Hello, world!");
}
