mod using_shortcut_path;
mod path_scope_with_use_keyword;
mod relative_path_with_super;
mod using_pub_with_struct;
mod using_pub_with_enums;
mod new_name_with_as_key_word;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to wait list...")
        }

        fn seat_at_table() {
            println!("Seating at the table...")
        }
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/*
* Using shortcut path
*/
pub fn print_shortcut_path() {
    using_shortcut_path::eat_at_restaurant();
}


/*
* Bringing paths into scope with the `use` keyword
*/
pub fn print_using_path_with_use_key_word() {
    path_scope_with_use_keyword::customer::eat_at_restaurant();
}


/*
* Relative path with `super`
*/
pub fn print_relative_path_with_super() {
    relative_path_with_super::back_of_house::fix_incorrect_order();
}


/*
* Using `pub` with struct
*/
pub fn print_using_pub_with_struct() {
    using_pub_with_struct::eat_at_restaurant();
}

/*
* Using `pub` with enums
*/
pub fn print_using_pub_with_enums() {
    using_pub_with_enums::eat_at_restaurant();
}

/*
* Adding new name with `as` keyword
*/
pub fn print_add_new_name_with_as_keyword() {
    new_name_with_as_key_word::function1().expect("Do something in fn 1");
    new_name_with_as_key_word::function2().expect("Do something in fn 2");
}