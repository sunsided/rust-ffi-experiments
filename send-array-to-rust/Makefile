.PHONY: all

all: clean main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

array_utils: build_dir src/rust_array_utils.rs
	rustc src/rust_array_utils.rs --crate-type cdylib --out-dir=build

main: array_utils src/main.c
	cc -o build/arrays src/main.c -L. -Lbuild -lrust_array_utils -Wl,-rpath=.

ldd: main
	cd build && ldd arrays

ldd-debug: main
	cd build && LD_DEBUG=libs ldd arrays

run: all
	cd build && ./arrays
