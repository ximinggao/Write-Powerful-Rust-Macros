use iac_macro::iac;

fn main() {
    // won't accept '-', not valid for identifiers
    iac! {
        bucket uniquename => lambda (
            name = my_name, mem = 1024, time = 15
        )
    }
}
