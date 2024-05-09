mod advanced_traits;
mod unsafe_rust;

fn main() {
    println!("Running unsafe rust...");
    unsafe_rust::run_unsafe_rust();

    println!("Running advanced traits...");
    advanced_traits::run_advanced_traits();
}
