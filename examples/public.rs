use make_public_macro::public;

#[derive(Debug)]
#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    let e = Example {
        first: "Raku".into(),
        second: 27u32,
    };
    println!("{:?}", e);
}
