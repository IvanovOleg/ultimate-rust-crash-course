# Structs
Struct are similar to classes in other languages. Structs can have data fields, methods and associated functions.
```rust
struct RedFox {
    enemy: bool,
    life: u8,
}
```
You can create an instance of a struct by specifying a value to each of it's fields:
```rust
let fox = RedFox {
    enemy: true,
    life: 70,
}
```

Typically you will implement an associated function to use a constructor to create a struct with default values and then call that. Methods in the associated function are defined in the implementation block which is separate from the struct definition.

```rust
impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}
```

You can also create an instance of a struct, get and set fields or call methods like this:

```rust
let fox == RedFox::new();
let life_left = fox.life;
fox.enemy = false;
fox.some_method();
```

Methods are also defined in the implementation block:
```rust
impl RedFox {
    // associated function
    fn function() ...
    // methods
    fn move (self) ...
    fn borrow (&self) ...
    fn mut_borrow (&mut self) ...
}
```

Methods always take some sort of self as a first argumet.

Difference between functions and methods:
1. Function is used to pass or return the data, while the method operates the data in a struct.
2. Function is an independent functionality, while the method lies under object-oriented programming.
3. In functions, we don't need to declare the struct, while to use methods we need to declare the struct.

**Rust doesn't have a struct inheritance. It has traits instead.**