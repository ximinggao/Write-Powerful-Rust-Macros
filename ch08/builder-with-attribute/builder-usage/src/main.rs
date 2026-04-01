use builder_macro::Builder;

#[derive(Builder)]
#[allow(unused)]
struct Gleipnir {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_builder_for_struct_with_one_renamed_property() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
            #[rename = "fish_birth"]
            birth_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .tops_of("mountains".to_string())
            .fish_birth(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.birth_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_property() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            other_necessities: Vec<String>,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .other_necessities(vec!["a".to_string(), "b".to_string()])
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(
            gleipnir.other_necessities,
            vec!["a".to_string(), "b".to_string()]
        );
    }

    #[test]
    #[should_panic(expected = "field not set: _roots_of")]
    fn should_panic_if_field_not_set() {
        #[derive(Builder)]
        struct Gleipnir {
            _roots_of: String,
        }

        let _ = Gleipnir::builder().build();
    }
}
