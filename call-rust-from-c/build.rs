use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/add.c");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/add.c")
        .static_flag(true)
        .compile("add");

    println!("cargo:rustc-link-search={}", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=add"); // the "-l" flag
}
