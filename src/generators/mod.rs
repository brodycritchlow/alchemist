pub trait Value {
    fn generate() -> Self;
}

impl Value for i32 {
    fn generate() -> Self {
        rand::random::<i32>()
    }
}

impl Value for String {
    fn generate() -> Self {
        let len = (rand::random::<u8>() % 20) as usize;
        (0..len)
            .map(|_| (b'a' + rand::random::<u8>() % 26) as char)
            .collect()
    }
}

impl Value for bool {
    fn generate() -> Self {
        rand::random::<bool>()
    }
}

impl Value for f64 {
    fn generate() -> Self {
        rand::random::<f64>()
    }
}

impl Value for char {
    fn generate() -> Self {
        (b'a' + rand::random::<u8>() % 26) as char
    }
}

impl Value for u32 {
    fn generate() -> Self {
        rand::random::<u32>()
    }
}