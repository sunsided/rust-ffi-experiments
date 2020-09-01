use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn puts(data: *const c_char) -> c_int;
}

fn main() {
    // The CString::new() method might fail if the string contains \0 characters.
    // Since we know it doesn't, we can just unwrap() the result.
    let hello_world = CString::new("Hello World! (from libc)").unwrap();

    // All calls to foreign code are `unsafe`.
    unsafe {
        puts(hello_world.as_ptr());
    }
}
