# Basic Callbacks: Passing a Rust function pointer to C

Here, a the C function `long_computation(...)` in [basic.c](src/basic.c) implements a
long-running operation. In order to report progress, it takes a function pointer to a
callback method. The main application in [main.rs](src/main.rs) calls `long_computation()`
and passes a Rust function pointer that prints the progress.

In C, the callback and method signatures are

```c
typedef void (*Progress)(float);
double long_computation(double n, Progress progress);
```

In Rust, the callback function pointer is defined as

```rust
type Progress = extern "C" fn(f32);
```

which makes the extern signature:

```rust
extern "C" {
    fn long_computation(n: c_double, progress: Progress) -> c_double;
}
``` 

The callback function then is simply implemented as

```rust
extern "C" fn progress(percent: f32) {
    println!("Progress: {:.2}%", percent);
}
```

and passed as usual:

```rust
let ret = unsafe { long_computation(..., progress) };
```

TL;DR

```bash
make run
```
