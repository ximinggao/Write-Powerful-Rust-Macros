use builder_macro::Builder;

#[derive(Builder)]
struct Gleipnir {
    roots_of: String,
    breath_of_a_fish: u8,
    other_necessities: bool,
}

fn main() {
    Gleipnir::builder()
        .roots_of("mountains".to_string())
        .breath_of_a_fish(1)
        // .other_necessities(true)
        .build();
}
