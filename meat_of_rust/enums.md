# Enums
Enums in Rust are more like algebraic data types in Haskell than C-like enums.

```rust
enum Colors {
    Red,
    Green,
    Blue
}

let color = Colors::Red; // basic usage
```

However the real power of a Rust enum comes from associating data and methods with the variants. A variant can have a single type of data, a tuple of data, or an anonymous struct of data.
```rust
enum DispenserItem {
    Empty, // You can always have a named variant with no data.
    Ammo(u8),
    Things(String, i32),
    Place{x: i32, y: i32},
}

use DispenserItem::*;
let item = Empty;
// let item = Ammo(69);
// let item = Things("hat".to_string(), 7);
let item = Place{ x: 24, y: 48 };
```
An enum is sort of like a union in C only so much better.
If you create an enum the value can be any one of these variants. 

For example: your DispenserItem could be an Empty with no data associated with it, but you can tell that it's an Empty. Or it could be an Ammo with a single byte in it, or it could be a Things with a String and a signed 32-bit integer in it. Or it could be a Place with x and y i32s. It can be any one of those, but only one at a time.

Even better, you can implement functions and methods for an enum.
```rust
impl DispenserItem {
    fn display(&self) { }
}
```

You can also use in enums with generics. Option is a generic enum in the standard library that you will use all the time.
```rust
enum Option<T> {
    Some(T),
    None
}
```
The T in angle brackets means any type. You don't have to use T, you could use some other valid identifier, but the idiomatic thing to do in Rust is to use T or some other capital letter. The Option enum represents when something is either absent or present. If you're trying to reach for a null or nil value like in other languages, you probably want to use an Option in Rust. You either have some value wrapped in the Some variant, or you have None.

```rust
if let Some(x) = my_variable {
    println!("value is {}", x);
}
```
Because enums can represent all sorts of data, you need to use patterns to examine them. If you want to check for a single variant, you use the "if let" expression. "if let" takes a pattern that will match one of the variants.

If the pattern does match, then the condition is true and the variables inside the pattern are created for the scope of the "if let" block. If the pattern doesn't match then the condition is false.

This is pretty handy if you care about one variant matching or not, but not as great if you need to handle all the variants at once.
```rust
match my_variable {
    Some(x) => {
        println!("value is {}", x);
    },
    None => {
        println!("no value");
    },
}
```
In that case, you use the match expression, which is match, a variable whose type supports matching, like an enum, the body of the match in braces, where you specify patterns followed by double arrows, which are equal signs followed by greater than symbols pointing to an expression that represents the return value of that arm of the match.

Match expressions require you to write a branch arm for every possible outcome. In other words, the patterns in a match expression must be exhaustive.

```rust
match my_variable {
    _ => {
        println!("who cares");
    },
}
```
A single underscore all by itself is a pattern that matches anything and can be used for a default or anything-else branch.

Note that even though you will often see blocks as the expression for a branch arm, any expression will do, including things like function calls and bare values. Either all branch arms need to return nothing or all branch arms need to return the same type.
```rust
match my_variable {
    Some(x) => x.squared + 1;
    None => 42;
}
```
Remember that if you actually use the return value of an expression that ends in a curly brace like match, if let, or if, or a nested block, then you have to put a semicolon after the closing brace.

```rust
match my_variable {
    Some(x) => x.squared + 1;
    None => 42;
};
```

## Option and Results
They're used all over the standard library, so you will encounter them constantly.

### Option
```rust
enum Option<T> {
    Some(T),
    None
}
```
Option is used whenever something might be absent. This is a none variant of an option:
```rust
let mut x: Option<i32> = None;
```
We don't need to make a use statement for an Option since it is included in the prelude of the standard library and they are always brought into a scope.

If you ever use Option with a concrete type, then the compiler will infer the type, which means you can leave the type annotation off of the declaration most of the time.
```rust
let mut x = None;
x = Some(5);
x.is_some(); // true
x.is_none(); // false
for i in x {
    println!("{}", i); // prints 5
}
```
There's a handy helper method called is_some() that returns true if x is the Some variant. There is also an is_none() method that does just the opposite.

Option implements the IntoIterator trait, so you can also treat it similar to a vector of 0 or 1 items and put it in a for loop.
There are a lot of usefull methods for option.

### Result
The other important enum is Result. Result is used whenever something might have a useful result, or might have an error.
This turns up especially often in the io module.

```rust
#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The type wrapped by Ok and the type wrapped by Err are both generic but independent of each other.

The #[must_use] annotation makes it a compiler warning to silently drop a result. You have to do something with it. Rust strongly encourages you to look at all possible errors and make a conscious choice what to do with each one.

```rust
use std::fs:file;
fn main(){
    // File::open("foo"); // unsafe since we ignore the result
    let res = File::open("foo");
    // let f = res.unwrap(); // crashes the app if there is an error
    let f = res.expect("file doesn't exist"); // crashes the app and returns the error message
}
```
If the Result is an Ok then this gives you the File struct that you wanted. If the Result is an Err then this crashes the program.
In some cases crashing the program may be what you want.

It's exactly the same as unwrap(), except that the string that you pass to expect() is also printed in the crash output, so you can provide yourself a little bit of custom context as to why the crash occurred.

Just like Option, there are helper methods like is_ok() and is_err() that return booleans.
```rust
use std::fs:file;
fn main(){
    let res = File::open("foo");
    if res.is_ok {
        let f = res.unwrap();
    }
}
```

Full pattern matching also works.
```rust
use std::fs:file;
fn main(){
    let res = File::open("foo");
    match res {
        Ok(F) => { /* do stuff */ },
        Err(E) => { /* do stuff */ },
    }
}
```