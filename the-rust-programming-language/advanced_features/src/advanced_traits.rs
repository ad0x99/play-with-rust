mod associated_types;
mod generic_type_parameters_and_operator_overloading;
mod method_with_same_name;
mod newtype_pattern_with_external_traits;

pub fn run_advanced_traits() {
    println!("========Running associated types========");
    associated_types::associated_types();

    println!("========Running generic type parameters & operator overloading========");
    generic_type_parameters_and_operator_overloading::generic_type_parameters_and_operator_overloading();

    println!("========Running method with same name========");
    method_with_same_name::method_with_same_name();

    println!("========Running newtype pattern to implement external traits========");
    newtype_pattern_with_external_traits::newtype_pattern_with_external_trail();
}
