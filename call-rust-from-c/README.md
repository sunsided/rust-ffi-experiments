# Calling a Rust function from C

The [lib.rs](src/lib.rs) file contains an exported `print()` function that is called
from `main()` in [main.c](src/main.c).

The Rust library is compiled to a static library (`.a`) via

```bash
rustc src/lib.rs --crate-type staticlib
```

When providing `--print native-static-libs` to the `rustc` call, it will list
the static libraries that the final application will need to link against:

```
note: Link against the following native artifacts when linking against this static library. The order and any duplication can be significant on some platforms.
note: native-static-libs: -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -ldl -lutil
```

The C program is compiled as usual and linked against the resulting libray:

```bash
clang src/main.c libprint.a -lpthread -ldl
```

The [Makefile](Makefile) provides a naive way of doing the above steps; to build an run, execute

```bash
make run
```

Run `make nm` to inspect the produced library for the `print` symbol.
