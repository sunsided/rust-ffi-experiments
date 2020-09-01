# Stateful Callbacks: Calling from C into a Rust function pointer (passing some state)

In this project, [stateful.rs](src/stateful.rs) defines a `Process` function pointer
taking a pointer to some state, as well as a regular argument. In [main.c](src/main.c),
a `struct` is implemented to hold the state.

```rust
type Process = extern "C" fn(*mut c_void, c_int);

#[no_mangle]
pub extern "C" fn generate_numbers(upper: c_int, callback: Process, state: *mut c_void) {
    // ...
    callback(state, parameter);
    // ...
}
```

On the C side, the `state` pointer is cast to a pointer of the actual State type and then processed:

```c
void increment_statistics(void *state, int value) {
    MyState *const state = (MyState*)data;
    // ...
}
```

The `main` method simply calls out to rust, passing the callback function pointer as well
as the address of the state object:

```c
generate_numbers(upper_limit, increment_statistics, &state);
```

TL;DR

```bash
make run
```
