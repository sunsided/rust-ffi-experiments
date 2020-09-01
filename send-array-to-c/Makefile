.PHONY: all

all: clean array_utils main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

array_utils: build_dir src/array_utils.c
	clang -shared src/array_utils.c -o build/libarray_utils.so

main: build_dir src/main.rs
	rustc --crate-type=bin src/main.rs -o build/arrays -L. -Lbuild -larray_utils -C link-arg=-Wl,-rpath=.

ldd: main
	cd build && ldd arrays

ldd-debug: main
	cd build && LD_DEBUG=libs ldd arrays

run: all
	cd build && ./arrays
