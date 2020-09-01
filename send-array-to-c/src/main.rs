use std::os::raw::c_int;

extern "C" {
    fn sum(my_array: *const c_int, length: c_int) -> c_int;
}

fn main() {
    let numbers: [c_int; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    // An unsafe block is required since we're dealing with raw pointers.
    let total = unsafe {
        sum(numbers.as_ptr(), numbers.len() as c_int)
    };

    println!("The total is {} (calculated in C)", total);
    assert_eq!(total, numbers.iter().sum());
}
