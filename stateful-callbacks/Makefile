.PHONY: all

all: clean main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

stateful: build_dir src/stateful.rs
	rustc src/stateful.rs --crate-type cdylib --out-dir=build

main: stateful src/main.c
	clang -o build/main src/main.c -L. -Lbuild -lstateful -Wl,-rpath=.

run: all
	cd build && ./main
