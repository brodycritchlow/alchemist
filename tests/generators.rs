use alchemist::Value;

#[test]
fn test_i32_generates() {
    for _ in 0..100 {
        let _: i32 = i32::generate();
    }
}

#[test]
fn test_u32_generates() {
    for _ in 0..100 {
        let _: u32 = u32::generate();
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
fn test_f64_generates() {
    for _ in 0..100 {
        let v: f64 = f64::generate();
        assert!(!v.is_nan());
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
fn test_string_generates() {
    for _ in 0..100 {
        let s: String = String::generate();
        assert!(s.len() <= 19, "string length should be < 20");
        assert!(s.chars().all(|c| c >= 'a' && c <= 'z'), "string should be lowercase a-z");
    }
}
