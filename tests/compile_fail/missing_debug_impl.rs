use alchemist::{alchemist, Value};

struct NoDebugImpl {
    x: i32,
}

impl Value for NoDebugImpl {
    fn generate() -> Self {
        NoDebugImpl { x: 0 }
    }
}

#[alchemist(NoDebugImpl)]
fn test_missing_debug(n: NoDebugImpl) {
    let _ = n.x;
}

fn main() {}
