# Collections
## Vector
```rust
Vec<T>
```
Vector is a generic collection that holds a bunch of one type. You can use it as a list or an array in the other languages. The most commonly used collection.
```rust
let mut v: Vec<i32> = Vec::new();
v.push(2);
v.push(4);
v.push(6);
let x = v.pop(); // x is 6
println!("{}", v[1]); // prints 4
```
Vector acts like a stack, so push adds an item to the end. Pop returs last item and removes it from the vector. Sincnee vector stores an item with a known size in memory next to each other, you can index it. If index is out of bound, rust will panic.

```rust
let v = vec![2, 4, 6]; // this macro makes vector creation easier
```
Vectors are easy to control and have a lot of useful methods.

## HashMap
```rust
HashMap<K,V>
```
Hashmap is a generic collection where you specify a type for a key and a type for a value and you access values by key. In some languages it is called a dictionary. The point of HashMaps is to insert, lookup and remove values by key in constant time.

```rust
let mut h: HashMap<u8,bool> = HashMap::new();
h.insert(5, true);
h.insert(6, false);
let have_five = h.remove(&5).unwrap();
```

## VecDeque
VecDeque uses a ring buffer to implement a double-ended queue, which can efficiently add or remove items from the front and the back, but everything else is a little less efficient than a regular Vector.

## LinkedList
LinkedList has the dubious distinction of being quick at adding or removing items at an arbitrary point in the list, but slow doing absolutely anything else.

## HashSet
HashSet is a hashing implementation of a set that performs set operations really efficiently.

## BinaryHeap
BinaryHeap is like a priority queue which always pops
off the max value.

## BTreeMap and BTreeSet
BTreeMap and BTreeSet are alternate map and set implementations using a modified binary tree. You usually only choose these over the hash variants if you need the map keys or set values to always be sorted.