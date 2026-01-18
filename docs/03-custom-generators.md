# Alchemist - Custom Generators

![Alchemist Banner](images/Custom%20Generators.png)

Alchemist supports custom types in the `#[alchemist]` macro. Any type that implements the [`Value`](https://git.critchlow.net/brodycritchlow/alchemist/src/branch/main/src/generators/mod.rs#L27-L29) trait can be used.

When the macro encounters a type it doesn't recognize as a builtin, it [falls back](https://git.critchlow.net/brodycritchlow/alchemist/src/branch/main/alchemist_macros/src/lib.rs#L126-L129) to calling `<YourType as alchemist::Value>::generate()`. This means you can use any custom struct, enum, or generic type as long as you provide a `Value` implementation.

## The Value Trait

```rust
pub trait Value: std::fmt::Debug {
    fn generate() -> Self;
}
```

Custom types must implement both `Value` and `Debug`. The `Debug` requirement ensures Alchemist can print generated values when a test fails.

## Custom Structs

```rust
use alchemist::{alchemist, Value};

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

#[alchemist(Point)]
fn test_point(p: Point) {
    let _ = p.x.saturating_add(p.y);
}
```

## Custom Enums

```rust
use alchemist::{alchemist, Value};

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

#[alchemist(Direction)]
fn test_direction(d: Direction) {
    let _ = format!("{:?}", d);
}
```

## Path Support

Types from other modules work with full paths:

```rust
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

#[alchemist(geometry::Rectangle)]
fn test_rectangle(rect: geometry::Rectangle) {
    let _ = rect.width.saturating_mul(rect.height);
}
```

## Generics

Generic types work if you provide a blanket implementation:

```rust
use alchemist::{alchemist, Value};

#[derive(Debug)]
struct Wrapper<T>(T);

impl<T: Value> Value for Wrapper<T> {
    fn generate() -> Self {
        Wrapper(T::generate())
    }
}

#[alchemist(Wrapper<i32>)]
fn test_wrapper(w: Wrapper<i32>) {
    let _ = w.0;
}
```

## Containers

Custom types work inside `list()` and `option()`:

```rust
#[alchemist(list(Point))]
fn test_list_of_points(points: Vec<Point>) {
    assert!(points.len() <= 9);
}

#[alchemist(option(Direction))]
fn test_optional_direction(dir: Option<Direction>) {
    let _ = dir;
}
```

## Gotchas

### Debug is required

The `Value` trait requires `Debug`. If you forget `#[derive(Debug)]`, you'll get a trait bound error.

### Non-Copy types in option()

The macro wraps your test in a closure. If your type doesn't implement `Copy`, you may get a "borrow of moved value" error. Add `#[derive(Clone, Copy)]` if your type supports it.

### Integer overflow

Generated values can be large. Use `saturating_add`, `saturating_mul`, etc. to avoid overflow panics—or let Alchemist find those bugs for you.
