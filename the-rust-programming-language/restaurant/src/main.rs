use restaurant;

fn main() {
    println!("=====eat_at_restaurant=====");
    restaurant::eat_at_restaurant();

    println!("=====print_shortcut_path=====");
    restaurant::print_shortcut_path();

    println!("=====print_using_path_with_use_key_word=====");
    restaurant::print_using_path_with_use_key_word();

    println!("=====print_relative_path_with_super=====");
    restaurant::print_relative_path_with_super();

    println!("=====print_using_pub_with_struct=====");
    restaurant::print_using_pub_with_struct();

    println!("=====print_using_pub_with_enums=====");
    restaurant::print_using_pub_with_enums();

    println!("=====print_add_new_name_with_as_keyword=====");
    restaurant::print_add_new_name_with_as_keyword();
}