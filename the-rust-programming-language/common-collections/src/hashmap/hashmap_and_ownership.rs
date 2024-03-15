use std::collections::HashMap;

pub fn ownership_in_hashmap() {
    // Hashmap and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("The value of map is {:?}", map);

    // We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.
    // field_name and field_value are invalid at this point, try
    // using them and see what compiler error you get!
    // println!("field_name is {:?}", field_name);
}
