use alchemist::{Value, alchemist};

mod geometry {
    use alchemist::Value;

    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Value for Rectangle {
        fn generate() -> Self {
            Rectangle {
                width: u32::generate(),
                height: u32::generate(),
            }
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Value for Point {
    fn generate() -> Self {
        Point {
            x: i32::generate(),
            y: i32::generate(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Value for Direction {
    fn generate() -> Self {
        match rand::random::<u8>() % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }
}

#[derive(Debug)]
struct PositiveInt(u32);

impl Value for PositiveInt {
    fn generate() -> Self {
        PositiveInt(u32::generate().saturating_add(1))
    }
}

#[test]
fn test_custom_struct_generates() {
    for _ in 0..100 {
        let p: Point = Point::generate();
        let _ = p.x.saturating_add(p.y);
    }
}

#[test]
fn test_custom_enum_generates() {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for _ in 0..100 {
        let d = Direction::generate();
        seen.insert(format!("{:?}", d));
    }
    assert!(seen.len() >= 2, "should generate multiple variants");
}

#[test]
fn test_constrained_generator() {
    for _ in 0..100 {
        let p = PositiveInt::generate();
        assert!(p.0 >= 1, "PositiveInt should always be >= 1");
    }
}

#[alchemist(Point)]
fn test_custom_struct_in_macro(p: Point) {
    let _ = p.x.saturating_add(p.y);
}

#[alchemist(Direction)]
fn test_custom_enum_in_macro(d: Direction) {
    let _ = format!("{:?}", d);
}

#[alchemist(PositiveInt)]
fn test_constrained_in_macro(p: PositiveInt) {
    assert!(p.0 >= 1);
}

#[alchemist(Point, int)]
fn test_custom_with_builtin(p: Point, scale: i32) {
    let _ = p.x.saturating_mul(scale);
}

#[alchemist(list(Point))]
fn test_list_of_custom(points: Vec<Point>) {
    assert!(points.len() <= 9);
    for p in &points {
        let _ = p.x.saturating_add(p.y);
    }
}

#[alchemist(option(Direction))]
fn test_option_of_custom(maybe_dir: Option<Direction>) {
    if let Some(d) = maybe_dir {
        let _ = format!("{:?}", d);
    }
}

#[alchemist(list(Point), option(Direction), int)]
fn test_mixed_custom_and_builtin(points: Vec<Point>, dir: Option<Direction>, n: i32) {
    let _ = points.len();
    let _ = dir;
    let _ = n;
}

#[alchemist(geometry::Rectangle)]
fn test_path_support(rect: geometry::Rectangle) {
    let _ = rect.width.saturating_mul(rect.height);
}

#[alchemist(geometry::Rectangle, int)]
fn test_path_with_builtin(rect: geometry::Rectangle, scale: i32) {
    let _ = rect.width;
    let _ = scale;
}

#[alchemist(list(geometry::Rectangle))]
fn test_list_of_path_type(rects: Vec<geometry::Rectangle>) {
    assert!(rects.len() <= 9);
}

#[derive(Debug)]
struct Wrapper<T>(T);

impl<T: Value> Value for Wrapper<T> {
    fn generate() -> Self {
        Wrapper(T::generate())
    }
}

#[alchemist(Wrapper<i32>)]
fn test_generic_type(w: Wrapper<i32>) {
    let _ = w.0;
}

mod outer {
    pub mod middle {
        pub mod inner {
            use alchemist::Value;

            #[derive(Debug)]
            pub struct DeepType {
                pub value: i32,
            }

            impl Value for DeepType {
                fn generate() -> Self {
                    DeepType {
                        value: i32::generate(),
                    }
                }
            }
        }
    }
}

#[alchemist(outer::middle::inner::DeepType)]
fn test_deeply_nested_path(d: outer::middle::inner::DeepType) {
    let _ = d.value;
}

#[derive(Debug)]
struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A: Value, B: Value> Value for Pair<A, B> {
    fn generate() -> Self {
        Pair {
            first: A::generate(),
            second: B::generate(),
        }
    }
}

#[alchemist(Pair<i32, bool>)]
fn test_multiple_type_params(p: Pair<i32, bool>) {
    let _ = p.first;
    let _ = p.second;
}

#[alchemist(Pair<String, Pair<i32, bool>>)]
fn test_nested_generic_types(p: Pair<String, Pair<i32, bool>>) {
    let _ = p.first;
    let _ = p.second.first;
    let _ = p.second.second;
}
