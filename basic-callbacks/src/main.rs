use std::os::raw::c_double;

type Progress = extern "C" fn(f32);

extern "C" {
    fn long_computation(n: c_double, progress: Progress) -> c_double;
}

fn main() {
    println!("Starting a long computation ...");
    let ret = unsafe { long_computation(6.48074069840786f64, progress) };
    println!("Computation finished, returning {}", ret);
}

extern "C" fn progress(percent: f32) {
    println!("Progress: {:.2}%", percent);
}
