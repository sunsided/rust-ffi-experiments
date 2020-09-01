# Passing POD (Plain Old Data) types from C to Rust

In this project, a C application provides a POD type in [main.c](src/main.c) and passes it to a Rust function in 
[add_points.rs](src/add_points.rs) that calculates the sum of its fields.

On the C side, the `Point` data structure is defined as

```c
typedef struct Point {
    double x;
    double y;
} Point;
```

Similarly on the Rust side, `Point` is defined as

```rust
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

Here, the `#[repr(C)]` attribute instructs the Rust compiler to not rearrange or pad the
`struct` in order to maintain the same layout as the C side.

TL;DR

```bash
make run
```

For information about `-Wl,-rpath=.`, see [Passing arrays from Rust to C](../send-array-to-c);
for `--crate-type cdylib`, see [Passing arrays from C to Rust](../send-array-to-rust).
