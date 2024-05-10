mod advanced_traits;
mod advanced_types;
mod unsafe_rust;

fn main() {
    println!("Running unsafe rust...");
    unsafe_rust::run_unsafe_rust();

    println!("Running advanced traits...");
    advanced_traits::run_advanced_traits();

    println!("Running advanced types...");
    advanced_types::run_advanced_types();
}
