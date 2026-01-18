use alchemist::Value;

#[derive(Debug)]
struct NoValueImpl {
    x: i32,
}

fn main() {
    let _ = <NoValueImpl as Value>::generate();
}
