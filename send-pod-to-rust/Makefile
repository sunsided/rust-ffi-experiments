.PHONY: all

all: clean add_points main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

add_points: build_dir src/add_points.rs
	rustc src/add_points.rs --crate-type cdylib --out-dir=build

main: build_dir src/main.c
	cc -o build/main src/main.c -L. -Lbuild -ladd_points -Wl,-rpath=.

ldd: main
	cd build && ldd main

ldd-debug: main
	cd build && LD_DEBUG=libs ldd main

run: all
	cd build && ./main
