# Scope

```rust
fn main() {
    //scope of the variable limits the place where you can use it
    let x = 5;
    {
        let y = 6;
        println!("{}, {}", x, y); // no error
    }
    println!("{}, {}", x, y); // error, since y is defined in a block

    let z  = 1;
    {
        let z = 2;
        println!("{}", z); // prints 2 since the value of the var z is shadowed inside the block
    }
    println!("{}", z); // prints 1 since the value of the var z inside the block is out of scope

    //shadow variable inside the block
    let mut s = 5;
    let s = s; //becomes immutable
    // you can change the type when you shadow a variable
    let meme = "hello";
    let meme = make_image(meme);
}
```
