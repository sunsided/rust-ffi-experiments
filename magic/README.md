# Building a safe wrapper around the `magic-sys` bindgen'd library

This library implements a safe wrapper around the `bindgen` created [magic-sys](../magic-sys)
library. Its code is based on the [Create Bindings for a C Library](https://s3.amazonaws.com/temp.michaelfbryan.com/wrap-libmagic/index.html)
("unofficial") documentation section.

To build the library and test it, run

```bash
cargo test
```
