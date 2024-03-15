mod create_string;
mod indexing_into_strings;
mod ownership_of_push_str;
mod string_concatenation;
mod update_string;

pub fn run_strings() {
    println!("=============create_new_string============");
    create_string::create_new_string();

    println!("=============update_string============");
    update_string::update_string();

    println!("=============ownership_of_push_str============");
    ownership_of_push_str::ownership_of_push_str();

    println!("=============concatenate_basic_string============");
    string_concatenation::concatenate_basic_string();

    println!("=============concatenate_multiple_strings============");
    string_concatenation::concatenate_multiple_strings();

    println!("=============indexing_into_strings============");
    indexing_into_strings::indexing_into_strings();

    println!("=============slicing_strings============");
    indexing_into_strings::slicing_strings();
}
