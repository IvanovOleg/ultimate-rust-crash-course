# Traits
Traits are similar to interfaces in the other languages. Rust takes the composition over inheritance approach.

```rust
struct RedFox {
    enemy: true,
    life: u32,
}

trait Noisy {
    fn get_noise(&self) -> &str;
}
```

Traits define required behavior. In another word, functions and methods a struct must implement in order to have that trait. Noisy trait specifies that struct must have the get_noise method that returns a borrowed string slice if it wants to be noisy.

An implementation of the noisy trait for the RedFox will look like this:

```rust
impl Noisy for RedFox {
    fn get_noise(&self) -> &str { "Meow?" }
}
```

Why not just to implement that method directly in the RedFox struct? The answer is that one we have a trait involved, we can start writing generic functions which can accept any value that implements the trait.

```rust
fn print_noise<T: Noisy>(item: T){
    println!("{}", item.get_noise());
}
```
This functions takes the item of type "T" which is defined to be anything that implements the "Noisy" trait. The function may use any bevior on the item that "Noisy" trait defines. Now we have a generic function that can take any type as long as it's satisfies the Noisy trait.

```rust
fn print_noise<T: Noisy>(item: T){
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get noise(&self) -> &str { "BYTE!" }
}

fn main() {
    print_noise(5_u8); // prints "BYTE!"
}
```
As long as one of either a trait or a struct is defined in your project. You can implement any trait for any struct. That means that you can implement your traits on any type from anywhere including built-in types, type you import from some other package. And on your structs you can implement any traits either built-in or imported from any other package.

## Copy
There is a special trait called "Copy". If your type implements Copy, then it will be copied instead of moved in move situations.
This makes sense for small values that fit entirely on the stack,
which is why the simple primitive types like integers floats and booleans implement Copy. If a type uses the heap at all then it cannot implement Copy. You can opt-in to implementing Copy with your own type if your type only uses other Copy types.

## Trait inheritance
Let's imagine we have a game with a guse a pegasus and a horse. These have several attributes in common. Guse and pegasus can fly. A pegasus and a horse can be ridden. Guse and a horse can explode.

| Character | Flies | Is ridden | Explosive |
| --------- | ----- | --------- | --------- |
| Guse      |   x   |           |     x     |
| Pegasus   |   x   |     x     |           |
| Horse     |       |     x     |     x     |

It can be tricky to define an object inheritance in this case, but it is a pretty straight-forward with traits. Traits implement inheritance. So a trait can inherit from another trait.

movement -> run -> ride | fly

demage -> explode

horse:
  - movement
  - run
  - ride
  - damage
  - explode

If a struct is going to implement a trait that inherits from the other trait means that struct should iplement both. Trait also can have a default behavior. So if you design them correctly, you don't need to specify them and use a default behavior instead.

Example of the default behavior usage:
```rust
trait Run {
    fn run(&self) {
        println!("I am running")
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    let robot = Robot {};
    robot.run();
}
```

You can't define fields as a part of traits. You need to define setters and getters methods instead.