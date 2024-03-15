mod drop_vector_element;
mod initialize_vector;
mod iterate_through_vector;
mod ownership_and_borrowing_with_vector;
mod read_vector_elements;
mod store_multiple_types_with_enum;
mod update_vector;

pub fn run_vector() {
    println!("=============initialize_a_new_vector============");
    initialize_vector::initialize_a_new_vector();

    println!("=============update_value_to_vector============");
    update_vector::update_value_to_vector();

    println!("=============read_elements_of_vector============");
    read_vector_elements::read_elements_of_vector();

    println!("=============ownership_and_borrowing_with_vector============");
    ownership_and_borrowing_with_vector::ownership_and_borrowing_with_vector();

    println!("=============iterate_through_vector============");
    iterate_through_vector::iterate_through_vector();

    println!("=============store_multiple_types_with_enum============");
    store_multiple_types_with_enum::store_multiple_types_with_enum();

    println!("=============drop_vector_element============");
    drop_vector_element::drop_vector_element()
}
