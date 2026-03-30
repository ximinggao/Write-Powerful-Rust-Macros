use panic_to_result_macro::panic_to_result;

#[panic_to_result]
fn create_employee_returning_result(name: &str, age: u8) -> Result<Employee, String> {
    if age > 35 {
        panic!("Employee age cannot be greater than 35");
    }
    Ok(Employee {
        name: name.into(),
        age,
    })
}

fn main() {}
