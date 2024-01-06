# Primitive types
## Scalar types
There are 4 scalar types:
1. Integers
2. Floats
3. Booleans
4. Characters

### Integers
| Unsigned | Signed  |
| -------- | ------- |
|    u8    |   i8    |
|   u16    |  i16    |
|   u32    | **i32** |
|   u64    |  i64    |
|  u128    | i128    |
|  usize   | isize   |

If you don't specify, by default Rust will use **i32**. Signed integers can store both positive and negative values. Unsigned can store only positive values. Usize represents the platform pointer type and can represent every memory address of the process. Usize you will use to indes into and array or vector. Isize also has same number of bits as a platform pointer type. The maximum isize value is upper bound of object's and array size. This ensures that isize can be used to calculate difference between pointers. Isize also allows any byte withing a value like a struct.

Integer literals can be specified in a number of ways:

|     Way        | Example     |
|----------      | ---------   |
|  Decimal       |  1000000    |
|   Hex          | 0xdeadbeef  |
|   Octal        | 0o775432111 |
|   Binary       | 0b11110011  |
| Byte (u8 only) |    b'A'     |

We can put underscore in the value and that underscore will be ignored:

10_000_000 is equal to 1000000
0xdead_beef is equal to 0xdeadbeef

### Floats

* f32
* **f64**

Floating literals follow the **IEEE-754** standard and look like:

3.14151

```rust
let x: u16 = 5;
let y: f32 = 3.14;
```

Or you can use a suffix which is usefull when you need to pass a literal to a generic function which can accept multiple numeric types:

```rust

let x = 5u16;
let y = 3.14f32;
```

Or you can use underscores to improve readability:

```rust

let x = 5_u16;
let y = 3.14_f32;
```

## Booleans
```rust
let x:bool = true;
let y:bool = false;
```

### Characters
Character represents a single unicode scalar value. Character is always contains 4 bytes (32 bits). Strings do not use characters inside.
```rust
let my_letter:char = 'r';
```

## Compound types
Compound types gather multiple values of other types.

### Tuple
Tuples store multiple values of any type:

```rust
let info = (1, 3.3, 999);
let info:(u8, f64, i32) = (1, 3.3, 999);
```

There are two types of accessing items of the tuple:
```rust
let first_item = info.0;
let (first_item, second_item, third_item) = info;
```
Tuples can store up to **12** items. This is a limitation.

### Arrays
Arrays store multiple values of the same type.
```rust
let buf = [1, 2, 3];
let trip = [0; 3]; // first item is a value, second item is how many
let typed: [u8; 3] = [1, 2, 3]; // types of array's items are always specified like this
```

You can access array's items like this:
```rust
let first_item = typed[0];
```
Arrays can contain up to 32 items without loosing functionality.