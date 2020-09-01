# Rust FFI experiments

An experiment for learning Rust's FFI. Some of the projects provide a binary written in C linking to a library
written in Rust, others provide a binary written in Rust linking to a C library. In the first case, Makefiles
are provided (usually with a `make run` command); in the second case `build.rs` scripts are used so that `cargo run`
should do the trick.

Here are some pointers: 
 
- [Calling `libc`'s `puts` from Rust](call-c-from-rust)
- [Cargo Build Scripts](build-scripts) (i.e., `build.rs`)
- [Calling a Rust function from C](call-rust-from-c)
- [Sending a Rust slice to a C function](send-array-to-c)
- [Sending a C array to a Rust function](send-array-to-rust)
- [Passing POD (Plain Old Data) types from C to Rust](send-pod-to-rust)
- [Passing complex types from Rust to C through opaque pointers](opaque-pointers-in-c) (with Valgrind examples)
- [Basic Callbacks: Passing a Rust function pointer to C](basic-callbacks)
- [Stateful Callbacks: Calling from C into a Rust function pointer (passing some state)](stateful-callbacks)

Using `bindgen`:

- [Creating a C library binding with `bindgen`](magic-sys)
