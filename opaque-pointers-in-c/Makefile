.PHONY: all

all: clean my_vec main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

my_vec: build_dir src/my_vec.rs
	rustc src/my_vec.rs --crate-type cdylib --out-dir=build

main: build_dir src/main.c
	cc -o build/main src/main.c -L. -Lbuild -lmy_vec -Wl,-rpath=.

ldd: main
	cd build && ldd main

ldd-debug: main
	cd build && LD_DEBUG=libs ldd main

run: all
	cd build && ./main
