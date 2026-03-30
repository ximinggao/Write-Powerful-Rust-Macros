use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Employee {
    name: String,
    age: u8,
}

#[panic_to_result]
fn create_employee(name: &str, age: u8) -> Employee {
    if age > 35 {
        panic!("Employee age cannot be greater than 35");
    }
    Employee {
        name: name.into(),
        age,
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn happy_path() {
        let employee = create_employee("Alice", 25).unwrap();
        assert_eq!(employee.name, "Alice");
        assert_eq!(employee.age, 25);
    }

    #[test]
    fn should_panic_on_invalid_age() {
        let employee = create_employee("Bob", 36);
        assert_eq!(
            employee.expect_err("this should be an err"),
            "Employee age cannot be greater than 35"
        );
    }
}
