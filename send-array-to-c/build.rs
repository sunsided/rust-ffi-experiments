use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/array_utils.c");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/array_utils.c")
        .static_flag(true)
        .compile("array_utils");

    println!("cargo:rustc-link-search={}", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=array_utils"); // the "-l" flag
}
