pub mod remove_duplication;
pub mod generic_in_functions;
pub mod generic_in_struct;
mod generic_in_enums;
mod generic_in_methods;

pub fn run_generic_data_types() {
    println!("========Removing Duplication========");
    remove_duplication::find_largest_number();

    println!("========Generic In Functions========");
    generic_in_functions::find_largest_number();

    println!("========Generic In Struct========");
    generic_in_struct::run_generic_in_struct();

    println!("========Generic In Methods========");
    generic_in_methods::run_generic_in_methods();
}