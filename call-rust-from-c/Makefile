.PHONY: all

all: lib main

lib: src/lib.rs
	rustc src/lib.rs --crate-type staticlib --print native-static-libs -o build/libprint.a

main: lib src/main.c
	clang src/main.c build/libprint.a -lpthread -ldl -o build/call-rust-from-c

nm: lib
	nm build/libprint.a | grep print --color=auto

run: all
	build/call-rust-from-c
