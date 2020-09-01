# Passing arrays from C to Rust

Here, a Rust application provides a slice in [main.rs](src/main.rs) and sets it to a C function in 
[array_utils.c](src/array_utils.c) that calculates the sum of its elements. 

TL;DR

```bash
make run
```

## Finding the shared object

After linking the executable, it may still fail at runtime with an error like the following:

```
./arrays: error while loading shared libraries: libarray_utils.so: cannot open shared object file: No such file or directory
```

Inspecting the binary with `ldd` shows that `libarray_utils.so` was indeed linked,
but cannot be found:

```
ldd build/arrays
        linux-vdso.so.1 (0x00007ffc21f77000)
        libarray_utils.so => not found
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f32d61a3000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f32d6180000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f32d6165000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f32d5f73000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f32d6247000)
```

A quick fix is to specify the `LD_LIBRARY_PATH` explicitly, e.g. like so

```bash
LD_LIBRARY_PATH=. ./arrays 
```

For GCC and Clang we could specify `-Wl,-rpath,.` to add `.` to the search paths.
For `rustc`, we can pass these arguments using `-C link-arg=...` such that
the build instruction becomes something along the lines of

```bash
rustc \
    --crate-type=bin \
    -C link-arg=-Wl,-rpath=. \
    -L. -Lbuild \
    -larray_utils \
    src/main.rs
```

There's a nice article about `rpath` and `runpath` at [Shared Libraries: Understanding Dynamic Loading](https://amir.rachum.com/blog/2016/09/17/shared-libraries/).
