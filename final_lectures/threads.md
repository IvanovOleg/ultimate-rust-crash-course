# Threads
Rust threading is portable. So this code we are about to look at should work across Mac, Linux, and Windows.
```rust
use std::thread;

fn main {
    let handle = thread::spawn( move || {
        // do stuff in a child thread
    });

    // do stuf simultaneously in the main thread

    // wait until thread has exited
    handle.join().unwrap();
}
```
We bring the thread module into scope first, then in our main function we call thread::spawn(). thread::spawn() takes a closure with no arguments. This closure is executed as the main function of the thread. So anything that we would want to do in our thread we would do here.

One common practice is to simply call a function from the closure. So we don't have a big, huge, long, inline closure.

spawn() returns a join handle. With that handle we can call join(), which will pause the thread that we're on until the thread we're joining has completed and exited. The thread that we spawn could have an error like a panic, or it could return a value successfully back to the thread that joins it.

So what we get from the join call is a result that wraps a possible success value returned from the thread, or an error if the thread panicked.

Threading is a bit heavyweight. Creating a new thread allocates an operating-system-dependent amount of RAM for the thread's own stack, often a couple of megabytes.

Whenever a CPU switches from running one thread to another, it has to do an expensive context switch. So the more threads you have trying to share a CPU core, the more overhead you will have in context switching. Even so, threads are a fantastic tool when you need to use CPU and memory concurrently, because they can run simultaneously on multiple cores, and actually accomplish more work!

However, if you just want to continue doing some work while you are waiting for something like disk i/o or network i/o, then I encourage you to look into async/await, which is a much more efficient approach for concurrently waiting for things.
