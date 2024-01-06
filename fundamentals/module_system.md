# Module system
main.rs:
```rust
// you can use 'use' keyword to import a library function without specifying an absolute path
use module_system::greet;
use rand::{thread_rng, Rng};
// import from standard library
// use std::collections::HashMap;

// crates.io is package registry for rust for enything except the standard library
// you need to add it as a dependency item in the Cargo.toml file

// name of the module, then :: as a scope operator, then name of the function in the library
fn main() {
    module_system::greet(); // you will get error in case you didn't make the greet function public
    greet(); // works because of 'use' that allows a relative path
    let x = rand::thread_rng().gen_range(0..100);
    let y = thread_rng().gen_range(0..10);
}

```
lib.rs:
```rust
// private function
// fn greet(){}

//public function
pub fn greet() {
    println!("Hello!");
}
```
