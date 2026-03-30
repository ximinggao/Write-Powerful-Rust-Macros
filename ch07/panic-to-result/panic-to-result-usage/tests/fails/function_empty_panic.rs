use panic_to_result_macro::panic_to_result;

#[panic_to_result]
fn create_employee_empty_panic(name: &str, age: u8) -> Employee {
    if age > 35 {
        panic!();
    }
    Employee {
        name: name.into(),
        age,
    }
}

fn main() {}
