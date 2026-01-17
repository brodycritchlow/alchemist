# Alchemist - Getting Started

![Alchemist Banner](images/Getting%20Started.png)

If you are an experienced user with rust, you can skip to [Installing Alchemist](#installing-alchemist). Otherwise, I strongly recommend reading [Chapter 7 of The Rust Programming Language](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)—especially [“Paths for Referring to an Item in the Module Tree”](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) and [“Exposing Paths with the pub Keyword”](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword)—which explain how libraries, modules, visibility, and imports work in Rust.

## Installing Alchemist

Since Alchemist is not yet published on crates.io, you must add it manually to your `Cargo.toml` using the Git repository:

```toml
[dependencies]
alchemist = { git = "https://git.critchlow.net/brodycritchlow/alchemist" }
```

By default, Cargo will use the latest commit on the default branch.

## Using a specific branch, tag, or commit

If you want to pin Alchemist to a specific version, you can specify a branch, tag, or commit hash:

```toml
[dependencies]
alchemist = { git = "https://git.critchlow.net/brodycritchlow/alchemist", branch = "main" }
```

```toml
[dependencies]
alchemist = { git = "https://git.critchlow.net/brodycritchlow/alchemist", tag = "v0.1.0" }
```

```toml
[dependencies]
alchemist = { git = "https://git.critchlow.net/brodycritchlow/alchemist", rev = "abc123" }
```

You are now ready to learn more about Alchemist. I recommend starting with [Configuration](configuration.md) before moving on to the other sections:

- [Custom Generators](custom-generators.md)
- [Failure](failure.md)
- [Types](types.md)

