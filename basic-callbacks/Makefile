.PHONY: all

all: clean main

clean:
	rm -rf build/

build_dir:
	mkdir -p build

basic: build_dir src/basic.c
	clang -shared src/basic.c -o build/libbasic.so

main: basic src/main.rs
	rustc --crate-type=bin src/main.rs -o build/main -L. -Lbuild -lbasic -C link-arg=-Wl,-rpath=.

run: all
	cd build && ./main
