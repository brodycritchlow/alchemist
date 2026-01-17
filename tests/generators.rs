use alchemist::Value;

#[test]
fn test_i8_generates() {
    for _ in 0..100 {
        let _: i8 = i8::generate();
    }
}

#[test]
fn test_i16_generates() {
    for _ in 0..100 {
        let _: i16 = i16::generate();
    }
}

#[test]
fn test_i32_generates() {
    for _ in 0..100 {
        let _: i32 = i32::generate();
    }
}

#[test]
fn test_i64_generates() {
    for _ in 0..100 {
        let _: i64 = i64::generate();
    }
}

#[test]
fn test_i128_generates() {
    for _ in 0..100 {
        let _: i128 = i128::generate();
    }
}

#[test]
fn test_u8_generates() {
    for _ in 0..100 {
        let _: u8 = u8::generate();
    }
}

#[test]
fn test_u16_generates() {
    for _ in 0..100 {
        let _: u16 = u16::generate();
    }
}

#[test]
fn test_u32_generates() {
    for _ in 0..100 {
        let _: u32 = u32::generate();
    }
}

#[test]
fn test_u64_generates() {
    for _ in 0..100 {
        let _: u64 = u64::generate();
    }
}

#[test]
fn test_u128_generates() {
    for _ in 0..100 {
        let _: u128 = u128::generate();
    }
}

#[test]
fn test_usize_generates() {
    for _ in 0..100 {
        let _: usize = usize::generate();
    }
}

#[test]
fn test_isize_generates() {
    for _ in 0..100 {
        let _: isize = isize::generate();
    }
}

#[test]
fn test_bool_generates() {
    let mut seen_true = false;
    let mut seen_false = false;
    for _ in 0..100 {
        match bool::generate() {
            true => seen_true = true,
            false => seen_false = true,
        }
    }
    assert!(seen_true && seen_false, "bool should generate both values");
}

#[test]
fn test_f32_generates() {
    for _ in 0..100 {
        let v: f32 = f32::generate();
        assert!(!v.is_nan(), "f32 should not be NaN");
    }
}

#[test]
fn test_f64_generates() {
    for _ in 0..100 {
        let v: f64 = f64::generate();
        assert!(!v.is_nan(), "f64 should not be NaN");
    }
}

#[test]
fn test_char_generates_lowercase() {
    for _ in 0..100 {
        let c: char = char::generate();
        assert!(c >= 'a' && c <= 'z', "char should be lowercase a-z");
    }
}

#[test]
fn test_char_full_coverage() {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for _ in 0..1000 {
        seen.insert(char::generate());
    }
    assert_eq!(seen.len(), 26, "should generate all lowercase letters a-z");
}

#[test]
fn test_string_generates() {
    for _ in 0..100 {
        let s: String = String::generate();
        assert!(s.len() <= 19, "string length should be < 20");
        assert!(
            s.chars().all(|c| c >= 'a' && c <= 'z'),
            "string should be lowercase a-z"
        );
    }
}
