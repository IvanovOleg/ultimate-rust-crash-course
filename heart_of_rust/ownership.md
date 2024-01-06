# Ownership
There are 3 rules of ownership:
1. Each value has an owner
2. There is only one owner of the value
3. Value gets dropped when it's owner goes out of scope.

```rust
let s1 = String::from("abc");
let s2 = s1;
println!("{}", s1); // produces a compiler error since value of s1 has been modev to s2, it is not a copy
```

Difference between stack and heap:
| Stack      | Heap          |
| ---------- |---------------|
| In order   | Unordered     |
| Fixed-size | Variable-size |
| LIFO       | Unordered     |
| Fast       | Slow          |

s1 will contain:
| Stack    | Value | Heap   |
| ------   | ---   | ---    |
| ptr      |   ->  |  a b c |
| len      |    3  |        |
| capacity |    3  |        |

Stack contains length, capacity and a pointer to bytes stored in heap. When we assing a value of s1 to s2, compiler forwards s2 pointer to a heap bytes of s1 (abc) and considers s1 as uninitialized. A value is stack + heap.

s2 will contain:
| Stack    | Value | Heap   |
| ------   | ---   | ---    |
| ptr      |   ->  |  a b c |
| len      |    3  |        |
| capacity |    3  |        |

Insted a moving value to s2, we can make a copy of it:
```rust
let s1 = String::from("abc");
let s2 = s1.clone();
println!("{}", s1); // works fine
```

Clone method makes a copy of stack and heap and sometimes called deep copy. Copy method copies just a stack and you will need to assign a value to it. When value is dropped the following actions will be executed:
1. Desctructor
2. Free heap
3. Stack pop

In this case there is no memory leak.

Another example:
```rust
let s1 = String::from("abc");
do_stuff(s1);
println!("{}", s1); // error since s1 has been moved to a function scope as a local variable
dn do_stuff(s: String){
    //
}
```
how to fix it? (but it is better to use references and borrowing)
```rust
let mut s1 = String::from("abc");
do_stuff(s1);
println!("{}", s1); // works fine since our function returns a value
dn do_stuff(s: String) -> String {
    s
}
```
