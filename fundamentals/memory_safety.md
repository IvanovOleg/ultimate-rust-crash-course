# Memory safety
```rust
fn main() {
    let enigma: i32;
    println!("{}", enigma); // produces a compilation error since var was initialized but not defined

    // conditional example 1
    let conditional_value: i32;
    if true {
        conditional_value = 1;
    }
    println!("{}", conditional_value); // produces a compilation error since compiler can't guarantee that value will be defined

    //conditional example 2
    let conditional_value: i32;
    if true {
        conditional_value = 1;
    } else {
        conditional_value = 2;
    }
    println!("{}", conditional_value); // no error since compiler can guarantee that a value will be set before being used
}

```