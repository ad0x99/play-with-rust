mod hashmap;
mod strings;
mod vector;

fn main() {
    println!("=============VECTORS in Rust============");
    vector::run_vector();

    println!("=============STRINGS in Rust============");
    strings::run_strings();

    println!("=============HASHMAP in Rust============");
    hashmap::run_hashmap()
}
