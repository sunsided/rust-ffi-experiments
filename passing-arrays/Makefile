.PHONY: all

all: array_utils main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

array_utils: build_dir src/array_utils.c
	clang -shared src/array_utils.c -o build/libarray_utils.so

main: build_dir src/main.rs
	rustc --crate-type=bin src/main.rs -o build/arrays -L. -Lbuild -larray_utils -C link-arg=-Wl,-rpath=.

ldd: main
	ldd build/arrays

ldd-debug: main
	LD_DEBUG=libs ldd build/arrays

run: all
	cd build && ./arrays
