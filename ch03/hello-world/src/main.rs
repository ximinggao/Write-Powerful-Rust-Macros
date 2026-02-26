#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
struct Example;

fn main() {
    let e = Example {};
    e.hello_world();
}
