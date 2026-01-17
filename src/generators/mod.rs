use rand::Rng;

macro_rules! impl_value_fixed {
    ($($t:ty),*) => {
        $(
            impl Value for $t {
                fn generate() -> Self {
                    rand::random::<$t>()
                }
            }
        )*
    };
}

macro_rules! impl_value_platform {
    ($($t:ty),*) => {
        $(
            impl Value for $t {
                fn generate() -> Self {
                    rand::rng().random::<u64>() as $t
                }
            }
        )*
    };
}

pub trait Value {
    fn generate() -> Self;
}

impl_value_fixed!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, f32, f64
);
impl_value_platform!(usize, isize);

impl Value for String {
    fn generate() -> Self {
        let len = (rand::random::<u8>() % 20) as usize;
        (0..len)
            .map(|_| (b'a' + rand::random::<u8>() % 26) as char)
            .collect()
    }
}

impl Value for char {
    fn generate() -> Self {
        (b'a' + rand::random::<u8>() % 26) as char
    }
}
