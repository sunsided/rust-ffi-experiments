# Cargo Build Scripts

This project uses [build.rs](build.rs) to build a static C library from [add.c](src/add.c) as well as a
Rust binary in [main.rs](src/main.rs) invoking a method from there.

TL;DR:

```bash
cargo run --bin build-scripts
```
