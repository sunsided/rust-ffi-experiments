extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system `magic` shared library.
    println!("cargo:rustc-link-lib=magic");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Create bindings for everything magical.
        .whitelist_function("magic_.*")
        .whitelist_var("MAGIC_.*")
        // Doesn't appear to work right now, but maybe some day ...
        .generate_comments(true)
        // Suppress linter warnings.
        .raw_line("#![allow(non_camel_case_types)]")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("lib.rs"))
        .expect("Couldn't write bindings!");
}
