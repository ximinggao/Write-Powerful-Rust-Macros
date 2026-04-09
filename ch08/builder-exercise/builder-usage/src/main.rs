use builder_macro::Builder;

#[derive(Builder)]
#[builder_defaults]
#[allow(unused)]
struct Gleipnir {
    #[builder(rename = "tops_of")]
    #[uppercase]
    roots_of: String,
    breath_of_a_fish: u8,
}
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
            #[builder(rename = "tops_of")]
            #[uppercase]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string().to_uppercase());
    }

    #[test]
    fn should_use_defaults_when_attribute_is_present() {
        #[derive(Builder)]
        #[builder_defaults]
        struct ExampleStructTwoFields {
            string_value: String,
            int_value: i32,
        }

        let exmaple: ExampleStructTwoFields = ExampleStructTwoFields::builder().build();

        assert_eq!(exmaple.string_value, String::default());
        assert_eq!(exmaple.int_value, i32::default());
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
