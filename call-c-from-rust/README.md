# Calling `libc`'s `puts()` function

Since `libc` is linked into every binary by default, we don't need to provide
`rustc` with any linker arguments.

To build and run, execute:

```bash
cargo run --bin call-c-from-rust
```
