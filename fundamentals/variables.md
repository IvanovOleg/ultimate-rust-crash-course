# Variables and Constants

## Variables
Define a variable with automatic type, you may skip type definition if it is clear
```rust
let bunnies = 2;
```

Define a variable with a certain type (32 bit integer in this case)
```rust
let ducks: i32 = 4;
```

Define multiple variables, is equal to separate definition of let dogs = 5; and let cats = 6;
```rust
let (dogs, cats) = (5, 6);
```

By default variables are immutable in rust, but you can override it:
```rust
let errors = 1;
errors = 2; // will produce an error

let mut no_errors = 0; // By making the variable mutable you can override it's value
no_errors = 1;
```

```rust
fn main() {
    //constants, capital letters, type is required, value should be available on the compilation time, can be used globally (outside of modules), very fast
    const CONSTANT_PI: f64 = 3.14;
}

```

## Constants
Constants are good for something that has the same value during the app execution. Constants, capital letters, type is required, value should be available on the compilation time, can be used globally (outside of modules), very fast.
```rust
const CONSTANT_PI: f64 = 3.14;
```