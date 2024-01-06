# References and Borrowing
## References
When you create a reference to s1, rust creates a pointer to s1 and manages it's lifecycle.
At the end of the function a reference gets dropped, not a value.
```rust
let s1 = String::from("abc");
do_stuff(&s1); // reference, not the value gets moved to the function
println!("{}", s1); // works since a value wasn't moved
dn do_stuff(s: &String){
    //
}
```
### Lifetimes
Lifetimes concept is a rule that references must always be valid. Compiler will not allow you to create a reference that outlives the referenced data and you can never point to null. References are immutable by default even if a value being referenced is mutable.

```rust
let s1 mut = String::from("abc");
do_stuff(&s1); // reference, not the value gets moved to the function
println!("{}", s1); // works since a value wasn't moved
dn do_stuff(s: &String){
    s.insert_str(0, "Hi, "); // Error
}
```
But if we make a mutable reference to a mutable value, we can change the value.

```rust
let s1 mut = String::from("abc");
do_stuff(&mut s1); // reference, not the value gets moved to the function
println!("{}", s1);
dn do_stuff(s: &mut String){
    s.insert_str(0, "Hi, "); // Works since a reference and a value are mutable, . operator automatically dereferences to an actual value
    // (*s).insert_str(0, "Hi, "); // Manual dereferencing, asterisk has a low precedence and usually should be put inside of prencencies
    *s = String::from("Replacement");
}
```

At the any given type rust allows you to have only one mutable reference or any number of immutable references. This rule works with threads as well.