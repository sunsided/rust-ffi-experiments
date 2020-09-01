# Passing complex types from Rust to C through opaque pointers

In this project, a Rust library provides a complex type `MyVec` in [my_vec.rs](src/my_vec.rs)
that is constructed and used in [main.c](src/main.c).

On the Rust side, a type `MyVec` is provided:

```rust
#[derive(Debug)]
pub struct MyVec(Vec<c_int>);
```

For this type, a set of free functions is provided (i.e., not `impl MyVec { ... }`). On the C side,
[my_vec.h](src/my_vec.h) defines the expected interface and a forward declaration of the `MyVec` type:

```c
typedef struct MyVec MyVec;

MyVec* my_vec_new();
int    my_vec_len(const MyVec*);
void   my_vec_push(MyVec*, int);
int*   my_vec_contents(MyVec*);
void   my_vec_destroy(MyVec*);
```

TL;DR

```bash
make run
```
