use builder_macro::Builder;

struct DoesNotImplementDefault;

#[derive(Builder)]
#[builder_defaults]
#[allow(unused)]
struct ExampleStruct {
    not: DoesNotImplementDefault,
}

fn main() {}
