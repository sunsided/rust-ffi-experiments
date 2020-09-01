# Passing arrays from C to Rust

Here, a C application provides an array in [main.c](src/main.c) and passes it to a Rust function in 
[rsut_array_utils.rs](src/rust_array_utils.rs) that calculates the sum of its elements

For information about `-Wl,-rpath=.`, see [Passing arrays from Rust to C](../send-array-to-c).

TL;DR

```bash
make run
```
