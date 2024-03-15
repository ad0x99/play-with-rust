mod dangling_references;
mod generic_lifetimes_in_functions;
mod lifetime_annotations_in_method_definitions;
mod lifetimes_all_together;
mod lifetimes_elision;
mod lifetimes_in_struct;

pub fn run_lifetimes() {
    println!("=========Preventing Dangling Reference=========");
    dangling_references::prevent_dangling_reference();

    println!("=========Generic Lifetimes in Functions=========");
    generic_lifetimes_in_functions::generic_lifetimes_in_fn();

    println!("=========Lifetimes in Struct=========");
    lifetimes_in_struct::lifetimes_annotation_in_struct();

    println!("=========Lifetimes Elision=========");
    lifetimes_elision::lifetimes_elision();

    println!("=========Lifetime Annotations in Method Definitions=========");
    lifetime_annotations_in_method_definitions::lifetimes_annotation_in_method_definitions();

    println!("=========Lifetimes All Together=========");
    lifetimes_all_together::find_longest_string_and_announce()
}
