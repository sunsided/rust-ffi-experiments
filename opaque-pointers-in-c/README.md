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

### Leaking memory on purpose

To see what happens if we don't free the `MyVec` instance by calling `my_vec_destroy(...)` in C,
here's an output of Valgrind after running `valgrind ./main` from the `build` directory (or just use `make valgrind`):

```
==165275== Memcheck, a memory error detector
==165275== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==165275== Using Valgrind-3.15.0 and LibVEX; rerun with -h for copyright info
==165275== Command: ./main
==165275== 
The initial length is 0
After pushing 42, the length is 1
After pushing 123, the length is 2
Iterating over the items in my vec:
my_vec[0] = 42
my_vec[1] = 123
==165275== 
==165275== HEAP SUMMARY:
==165275==     in use at exit: 40 bytes in 2 blocks
==165275==   total heap usage: 3 allocs, 1 frees, 1,064 bytes allocated
==165275== 
==165275== LEAK SUMMARY:
==165275==    definitely lost: 24 bytes in 1 blocks
==165275==    indirectly lost: 16 bytes in 1 blocks
==165275==      possibly lost: 0 bytes in 0 blocks
==165275==    still reachable: 0 bytes in 0 blocks
==165275==         suppressed: 0 bytes in 0 blocks
==165275== Rerun with --leak-check=full to see details of leaked memory
==165275== 
==165275== For lists of detected and suppressed errors, rerun with: -s
==165275== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

Specifically, `definitely lost: 24 bytes in 1 blocks` indicates that memory was leaked.
In comparison, here's how the summary looks like if the `MyVec` instance is destroyed
correctly:

```
==165635== HEAP SUMMARY:
==165635==     in use at exit: 1,232 bytes in 6 blocks
==165635==   total heap usage: 10 allocs, 4 frees, 2,328 bytes allocated
==165635== 
==165635== LEAK SUMMARY:
==165635==    definitely lost: 0 bytes in 0 blocks
==165635==    indirectly lost: 0 bytes in 0 blocks
==165635==      possibly lost: 0 bytes in 0 blocks
==165635==    still reachable: 1,232 bytes in 6 blocks
==165635==         suppressed: 0 bytes in 0 blocks
```

As expected, `lost: 0 bytes in 0 blocks` indicates we're good.
