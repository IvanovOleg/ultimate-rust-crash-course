# Closures
You'll encounter closures when you want to spawn a thread, or when you want to do some functional programming with iterators, and in some other common places in the standard library.

So let's learn about closures. A closure is an anonymous function that can borrow or capture some data from the scope it is nested in.

```rust
| x, y | { x + y }
```
The syntax is a parameter list between two pipes without type annotations, followed by a block. This creates an anonymous function called a closure that you can call later.

The types of the arguments and the return value are all inferred from how you use the arguments and what you return.

```rust
let add = | x, y | { x + y };
add(1,2); // returns 3
```

You don't have to have any parameters. You can just leave the parameter list empty.

```rust
| x, y | { x + y }
```
Technically you can leave the block empty as well, but that's not very interesting.
```rust
|| {}
```
What is really interesting is that a closure will borrow a reference to values in the enclosing scope.
```rust
let s = "straw".to_string();
let f = || {
    println!("{}", s);
};

f(); // prints "straw"
```
Here we create a string s and then we create a closure that borrows a reference to s, which works because the "println!" macro actually wants a reference anyway.

Then we assign the closure to the variable f, and whenever we call f it prints a strawberry. This is great if your closure never outlives the variable it is referencing, but the compiler won't let us send this over to another thread because another thread might live longer than this thread.

Lucky for us, closures also support move semantics, so we can force the closure to move any variables it uses into itself and take ownership of them.

```rust
let s = "straw".to_string();
let f = move || {
    println!("{}", s);
};

f(); // prints "straw"
```
Now s is owned by the closure and it will live until the closure itself goes out of scope and gets dropped. So we could send this closure over to another thread, or return it as the value of a function, or do whatever we want with it.

If you want to do some functional-style programming, closures will be your close friends! Call iter() on a vector to get an iterator and a whole bunch of methods that use closures will be available to you.

```rust
let mut v = !vec[2, 3, 4];
v.iter()
    .map( |x| x * 3 )
    .filter( |x| *x > 10 )
    .fold ( 0, |acc, x| acc + x );
```

Here is an example of using map() and a closure to multiply each item in a vector by 3, then filter() and a closure to discard any values that aren't greater than 10, and then fold() with an initial value and a closure to sum the remaining values together.
