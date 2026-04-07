use make_public_macro::public;

#[public(exclude(third))]
struct Example {
    first: String,
    pub second: u32,
    third: bool,
}

impl Example {
    pub fn new() -> Self {
        Self {
            first: "first".to_string(),
            second: 5,
            third: true,
        }
    }
}


fn main() {
    let e = Example::new();
    println!("{}", e.first);
    println!("{}", e.second);
    println!("{}", e.third);
}
