use alchemist::alchemist;

#[alchemist(i8)]
fn test_i8_macro(x: i8) {
    let _ = x;
}

#[alchemist(i16)]
fn test_i16_macro(x: i16) {
    let _ = x;
}

#[alchemist(i32)]
fn test_i32_macro(x: i32) {
    let _ = x;
}

#[alchemist(i64)]
fn test_i64_macro(x: i64) {
    let _ = x;
}

#[alchemist(i128)]
fn test_i128_macro(x: i128) {
    let _ = x;
}

#[alchemist(isize)]
fn test_isize_macro(x: isize) {
    let _ = x;
}

#[alchemist(u8)]
fn test_u8_macro(x: u8) {
    let _ = x;
}

#[alchemist(u16)]
fn test_u16_macro(x: u16) {
    let _ = x;
}

#[alchemist(u32)]
fn test_u32_macro(x: u32) {
    let _ = x;
}

#[alchemist(u64)]
fn test_u64_macro(x: u64) {
    let _ = x;
}

#[alchemist(u128)]
fn test_u128_macro(x: u128) {
    let _ = x;
}

#[alchemist(usize)]
fn test_usize_macro(x: usize) {
    let _ = x;
}

#[alchemist(f32)]
fn test_f32_macro(x: f32) {
    assert!(!x.is_nan());
}

#[alchemist(f64)]
fn test_f64_macro(x: f64) {
    assert!(!x.is_nan());
}

#[alchemist(bool)]
fn test_bool_macro(x: bool) {
    let _ = !x;
}

#[alchemist(char)]
fn test_char_macro(c: char) {
    assert!(c >= 'a' && c <= 'z');
}

#[alchemist(str)]
fn test_str_macro(s: String) {
    assert!(s.len() <= 19);
}

#[alchemist(i32, str)]
fn test_multiple_params(x: i32, s: String) {
    let _ = format!("{}: {}", x, s);
}

#[alchemist(list(i32))]
fn test_list_int(xs: Vec<i32>) {
    assert!(xs.len() <= 9);
}

#[alchemist(list(str))]
fn test_list_str(xs: Vec<String>) {
    for s in &xs {
        assert!(s.len() <= 19);
    }
}

#[alchemist(option(i32))]
fn test_option_int(maybe: Option<i32>) {
    if let Some(x) = maybe {
        let _ = x + 1;
    }
}

#[alchemist(option(bool))]
fn test_option_bool(maybe: Option<bool>) {
    let _ = maybe.unwrap_or(false);
}

#[alchemist(list(bool), option(i32))]
fn test_mixed_containers(xs: Vec<bool>, maybe: Option<i32>) {
    let _ = xs.len();
    let _ = maybe.unwrap_or(0);
}

#[alchemist(i32, iterations = 5)]
fn test_custom_iterations(x: i32) {
    let _ = x;
}

#[alchemist(i32, str, iterations = 10)]
fn test_iterations_with_multiple_params(x: i32, s: String) {
    let _ = format!("{}: {}", x, s);
}

#[alchemist(iterations = 3, i32)]
fn test_iterations_first(x: i32) {
    let _ = x;
}
