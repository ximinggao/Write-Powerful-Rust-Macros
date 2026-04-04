use builder_macro::Builder;

#[derive(Builder)]
#[allow(unused)]
struct Gleipnir {
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
    fn should_generate_builder_for_struct_with_properties() {
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
    fn should_work_with_correct_order() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            anything_else: bool,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("moutains".to_string())
            .breath_of_a_fish(1)
            .anything_else(true)
            .build();

        assert_eq!(gleipnir.roots_of, "moutains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.anything_else, true);
    }
}
