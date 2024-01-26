mod basic_hashmap;
mod hashmap_and_ownership;
mod updating_hashmap;

pub fn run_hashmap() {
    println!("=============Basic Operation in Hashmap============");
    basic_hashmap::initialize_hashmap();

    println!("=============Ownership in Hashmap============");
    hashmap_and_ownership::ownership_in_hashmap();

    println!("=============Updating in Hashmap============");
    updating_hashmap::updating_in_hashmap();
}
