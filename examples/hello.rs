#[macro_use]
extern crate hello_world_macro;

#[derive(Debug, Hello)]
struct Example;

#[derive(Hello)]
enum Pet {
    Cat,
}

fn main() {
    let s = Example {};
    s.hello_world();

    let p = Pet::Cat;
    p.hello_world();
}
