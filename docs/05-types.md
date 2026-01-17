# Alchemist - Types

![Alchemist Banner](images/Types.png)

Our type system is ever-growing and has been made easier to read and write thanks to the
implementation of macros that create Value implementations. For more information, see the
commit that introduced the [macros](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab), as well as the [generator implementation file](https://git.critchlow.net/brodycritchlow/alchemist/src/branch/main/src/generators/mod.rs).
This documentation will not only show what types are available in Alchemist, but also how our
system works.

At a high level, our type system works like this:


1. The `#[alchemist]` procedural macro (the part of the library that reads your code) parses the
    types you specify in your test.
2. For each type, it generates a call to the `generate()` function from the `alchemist::Value`
    trait.
3. This `generate()` function is responsible for creating a random value of the specified type.

The `generate()` function is implemented in one of three ways:


* [`impl_value_fixed`](https://git.critchlow.net/brodycritchlow/alchemist/src/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab/src/generators/mod.rs#L3-L13): This macro is used for fixed-size
    primitive types like i32 and f64. It uses the `rand::random()` function to generate a random
    value.
* [`impl_value_platform`](https://git.critchlow.net/brodycritchlow/alchemist/src/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab/src/generators/mod.rs#L15-L25): This macro is used for
    platform-dependent types like usize and isize. It generates a u64 and then casts it to the
    target type.
* [Manual Implementation](https://git.critchlow.net/brodycritchlow/alchemist/src/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab/src/generators/mod.rs#L36-L43): For special types like String
    and char, the `generate()` function is implemented manually to provide more control over the
    generated values.


In all cases, the `generate()` function will always return a randomly generated value of the
specified type.

| Type | Date Added | Commit |
| --- | --- | --- |
| `i32` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `String` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `bool` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `f64` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `char` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `u32` | 2026-01-16 | [45e23d0](https://git.critchlow.net/brodycritchlow/alchemist/commit/45e23d040b9c2121245a993a130d284ff164dad7) |
| `i8` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `i16` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `i64` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `i128` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `u8` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `u16` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `u64` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `u128` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `isize` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `usize` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |
| `f32` | 2026-01-17 | [8a54f83](https://git.critchlow.net/brodycritchlow/alchemist/commit/8a54f8304ed4dbca8123ada77ba57ba5c9f207ab) |