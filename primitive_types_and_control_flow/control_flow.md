# Control Flows
## Conditions
Each condition should evaluate to boolean. You can invert the condition by putting "!" in front of it.
```rust
if num == 5 {
    msg = "five";
}
```
Second condition example:
```rust
if num == 5 {
    msg = "five";
} else if num == 4 {
    msg = "four";
}
```

Second condition example:
```rust
if num == 5 {
    msg = "five";
} else if num == 4 {
    msg = "four";
} else {
    msg = "other";
}
```

If is an expression, not a statement. Statements don't return values, but expressions do. This also works:
```rust
msg = if num == 5 {
    "five"
} else if num == 4 {
    "four"
} else {
    "other" // no ; here since we need to return a value from the block
};
```
Several rules:
* no ; after the value since we need to return it
* return cann't be used here since it works only in function bodies
* returning values should be of the same type
* semicolon only at the end of expression
* braces are not optional

Expression mentioned above can also be written inline:
```rust
num = if a { b } else { c };
```
Nested version:
```rust
num = if a {
    if x { y } else { y }
} else {
    c
};
```
## Loops
### Unconditional loop
```rust
loop {
    break;
}
```
Breaking from a specific loop in a nested structure using a label:
```rust
'main: loop {
    loop {
        loop {
            break 'main; // breaks the main loop
        }
    }
}
```
Same with continue:
```rust
'main: loop {
    loop {
        loop {
            continue 'main; // terminates execution of the particular iteration of the particular loop, next iteration will be executed
    }
}
```

### Conditional loop
```rust
while dizzy() {
    // do stuff
}
```

### Iterable loop
```rust
for num in [7, 8, 9].iter() {
    // do stuff with num
}
```
For loop can be used to destructure array items:
```rust
let array = [(1, 2), (3, 4)];
for (x, y) in array.iter() {
    // do stuff with x and y
}
```
Using for loop with ranges:
```rust
for num in 0..50 { // start is inclusive, but end is exclusive, generates from 0 to 49
    // do stuff with num
}
```

```rust
for num in 0..=50 { // start and end are inclusive 0 - 50
    // do stuff with num
}
```