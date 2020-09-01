// Not required here but good practice to ensure that `forget` is never used.
// See e.g. https://anssi-fr.github.io/rust-guide/05_memory.html
#![deny(clippy::mem_forget)]

use std::os::raw::c_int;
use std::ptr;

#[derive(Debug)]
pub struct MyVec(Vec<c_int>);

impl Drop for MyVec {
    fn drop(&mut self) {
        // Ideally, something useful happens here. We're just printing to stdout
        // in order to make it clear that the destructor was called.
        println!("Destroying {:?}", self);
    }
}

/// Create a new `MyVec`.
#[no_mangle]
pub extern "C" fn my_vec_new() -> *mut MyVec {
    // First we create a new MyVec.
    let obj = MyVec(Vec::new());

    // Next, we copy it to the heap so we have a stable pointer to it.
    let boxed_obj = Box::new(obj);

    // Finally, we return a pointer to it by converting the `Box<MyVec>` into a raw pointer.
    // This "forgets" the reference (without actually calling `forget`). We do have a potential
    // memory leak now, which is why we need to make sure that the value is actually dropped later.
    Box::into_raw(boxed_obj)
}

/// Get the number of integers inside `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_len(vec: *const MyVec) -> c_int {
    if vec.is_null() {
        return 0;
    }

    (&*vec).0.len() as c_int
}

/// Get a pointer to the array of integers inside `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_contents(vec: *mut MyVec) -> *mut c_int {
    if vec.is_null() {
        return ptr::null_mut();
    }

    let vec = &mut *vec;
    vec.0.as_mut_ptr()
}

/// Add an integer to the end of `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_push(vec: *mut MyVec, n: c_int) {
    if vec.is_null() {
        return;
    }

    let vec = &mut *vec;
    vec.0.push(n);
}

/// Destroy `MyVec` and free the underlying array of numbers.
#[no_mangle]
pub unsafe extern "C" fn my_vec_destroy(obj: *mut MyVec) {
    // As a rule of thumb, freeing a null pointer is just a noop.
    if obj.is_null() {
        return;
    }

    // Convert the raw pointer back to a `Box<MyVec>` then explicitly drop it.
    // We could skip the call to `drop(boxed)` here since the box is dropped anyways when
    // leaving the scope.
    let boxed = Box::from_raw(obj);
    drop(boxed);

    // Note that ptr::drop_in_place(obj); would also be an option
    // but may come with more overhead (i.e., `dealloc` might be required).
}
