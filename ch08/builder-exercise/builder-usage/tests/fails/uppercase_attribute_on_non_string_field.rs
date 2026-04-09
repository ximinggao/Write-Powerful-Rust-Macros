use builder_macro::Builder;

#[derive(Builder)]
#[allow(unused)]
struct Gleipnir {
    #[builder(rename = "tops_of")]
    #[uppercase]
    roots_of: String,
    #[uppercase]
    breath_of_a_fish: u8,
}

fn main() {}
