# Passing arrays from C to Rust

Here, a C application provides an array in [main.c](src/main.c) and passes it to a Rust function in 
[rsut_array_utils.rs](src/rust_array_utils.rs) that calculates the sum of its elements

For information about `-Wl,-rpath=.`, see [Passing arrays from Rust to C](../send-array-to-c).

TL;DR

```bash
make run
```

In the [Makefile](Makefile), `rustc` is called with the `--crate-type cdylib` parameter:

> This instructs rustc to generate a dynamic library which can be called from 
> C (or any other language). Because the Rust standard library isn't guaranteed
> to be available on all machines, a cdylib will also contain bits of the standard
> library which are required to run.
