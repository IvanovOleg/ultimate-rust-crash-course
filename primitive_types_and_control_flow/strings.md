# Strings
There are at least 6 types of strings in rust, but two of them are most important.
## String slice (str)
String slice has 2 types:
1. Strings slice (**str**)
2. Borrowed string slice (**&str**)

```rust
let msg = "Hello Мир"; // literal string is always a borrowed string slice (&str)
```
Data in the borrowed string slice cann't be modified. Borrowed string slice contains a pointer to some bytes and a length.
```
&str
ptr -> | A | B | ц | д |
len = 4
```
Borrowed string slice is a valid utf-8. Can not be indexed by the character position.
## String (String)
Data in string can be modified. Borrowed string slice can be converted into the string this way:
```rust
let msg = "Hello пр".to_string();
// or
let msg = String::from("Hello пр");
```
String contains pointer, length and capacity.
```
&str
ptr -> | A | B | ц | д |   |   |   |   |
len = 4
capacity = 8
```
String is a valid utf-8. Can not be indexed by the character position. You can retrive bytes and index them if you want like this:

```rust
let word = "Hello world!";
let msg_bytes = word.bytes(); // works fine with ASCII portion of utf symbols
word.chars(); // allows to retriew an iterator
```
**uinicode-segmentation** is a usefull package to work with graphemes. There are many helper methods for string manipulations. Iterators have a metod .nth for indexing:
```rust
.nth(3);
```