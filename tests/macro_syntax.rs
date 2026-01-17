use alchemist::alchemist;

#[alchemist(int)]
fn test_int_macro(x: i32) {
    let _ = x + 1;
}

#[alchemist(uint)]
fn test_uint_macro(x: u32) {
    let _ = x.wrapping_add(1);
}

#[alchemist(bool)]
fn test_bool_macro(x: bool) {
    let _ = !x;
}

#[alchemist(float)]
fn test_float_macro(x: f64) {
    assert!(!x.is_nan());
}

#[alchemist(char)]
fn test_char_macro(c: char) {
    assert!(c >= 'a' && c <= 'z');
}

#[alchemist(str)]
fn test_str_macro(s: String) {
    assert!(s.len() <= 19);
}

#[alchemist(int, str)]
fn test_multiple_params(x: i32, s: String) {
    let _ = format!("{}: {}", x, s);
}

#[alchemist(list(int))]
fn test_list_int(xs: Vec<i32>) {
    assert!(xs.len() <= 9);
}

#[alchemist(list(str))]
fn test_list_str(xs: Vec<String>) {
    for s in &xs {
        assert!(s.len() <= 19);
    }
}

#[alchemist(option(int))]
fn test_option_int(maybe: Option<i32>) {
    if let Some(x) = maybe {
        let _ = x + 1;
    }
}

#[alchemist(option(bool))]
fn test_option_bool(maybe: Option<bool>) {
    let _ = maybe.unwrap_or(false);
}

#[alchemist(list(bool), option(int))]
fn test_mixed_containers(xs: Vec<bool>, maybe: Option<i32>) {
    let _ = xs.len();
    let _ = maybe.unwrap_or(0);
}

#[alchemist(int, iterations = 5)]
fn test_custom_iterations(x: i32) {
    let _ = x;
}

#[alchemist(int, str, iterations = 10)]
fn test_iterations_with_multiple_params(x: i32, s: String) {
    let _ = format!("{}: {}", x, s);
}

#[alchemist(iterations = 3, int)]
fn test_iterations_first(x: i32) {
    let _ = x;
}
