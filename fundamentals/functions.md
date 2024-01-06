# Functions and Macros

## Functions

```rust
fn main() {
    let x = do_stuff(2.0, 12.5);
}

// It doesn't metter where in the file you define a function, -> defines the output type
fn do_stuff(qty: f64, oz: f64) -> f64 {
    return qty * oz;
}

// You can use a short return expression by not putting ; at the end of the last line (tail expression):
fn do_stuff_short(qty: f64, oz: f64) -> f64 {
    qty * oz
}
```

## Macros
Macro is a shortcut for a function that simplifies definition.
```rust
fn main() {
    println!("Hello, world!"); // ! at the end is a sign of a macro
}
```