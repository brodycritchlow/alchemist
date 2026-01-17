# Alchemist - Failure

![Alchemist Banner](images/Failures.png)

When a test fails, Alchemist catches the panic and reports which iteration failed along with the generated values that caused the failure. This is implemented using [`std::panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) in the [macro source](https://git.critchlow.net/brodycritchlow/alchemist/src/branch/main/alchemist_macros/src/lib.rs#L191-L201).

## Failure Output

Given this test:

```rust
#[alchemist(int, int)]
fn test_addition(x: i32, y: i32) {
    assert!(x + y == y + x);
}
```

When it fails, you'll see output like this:

```
thread 'test_addition' panicked at tests/failure_test.rs:5:13:
attempt to add with overflow

Test failed on iteration 10 of 100
Generated values:
  x = -1513508590
  y = -1870612591
```

The failure header appears in red and the generated values in yellow, making failures easy to identify in terminal output.
