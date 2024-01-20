mod dangling_references;
mod functions;
mod loops;
mod numeric_operations;
mod ownership_and_function;
mod reference_and_borrowing;
mod return_values_and_scope;
mod scopes;
mod shadowing;
mod struct_method;
mod struct_method_multiple_impl;
mod struct_method_multiple_parameters;
mod structs;
mod the_slice_type;
mod variables;
mod enums;

fn main() {
    println!("==========Variable & Mutability=========");
    // Variable & Mutability
    variables::print_variable();

    println!("==========Constants=========");
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    println!("==========Shadowing=========");
    // Shadowing
    shadowing::shadowing();

    println!("==========Data Types=========");
    // Data Types
    let input = "15";
    // If we don’t add the :u32 type annotation shown in the preceding code
    // Rust will display an error
    let is_number: u32 = input.parse().expect("Not a number");
    println!("{is_number}");

    println!("==========Scalar Types=========");
    // Scalar Types
    let _signed_i8: i8 = 8;
    let _unsigned_u8: u8 = 8;
    let _signed_i32: i32 = 32;
    let _unsigned_u32: u32 = 32;
    let _signed_i64: i64 = 64;
    let _unsigned_u64: u64 = 64;
    let _signed_i128: i128 = 128;
    let _unsigned_u128: u128 = 128;
    let _arch_isize: isize = 64;
    let _arch_usize: usize = 32;

    println!("==========Floating-Point Types=========");
    // Floating-Point Types
    let f64 = 2.0;
    let f32: f32 = 3.0;
    println!("{f64}");
    println!("{f32}");

    println!("==========Numeric Operations=========");
    // Numeric Operations
    numeric_operations::print_numeric_operations();

    println!("==========The Boolean Type=========");
    // The Boolean Type
    let true_boolean = true;
    let false_boolean: bool = false; // with explicit type annotation
    println!("true_boolean: {true_boolean}");
    println!("false_boolean: {false_boolean}");

    println!("==========The Character Type=========");
    // The Character Type
    let char_z_lowercase = 'z';
    let char_z_uppercase: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("char_z_lowercase: {char_z_lowercase}");
    println!("char_z_uppercase: {char_z_uppercase}");
    println!("heart_eyed_cat: {heart_eyed_cat}");

    println!("==========The Tuple Type=========");
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {tup:?}");

    // Destructure tuple values
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    // Access tuple value using index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred_tup: {five_hundred}");
    println!("six_point_four_tup: {six_point_four}");
    println!("one_tup: {one}");

    println!("==========The Array Type=========");
    // The Array Type
    let array = [1, 2, 3, 4, 5];
    let same_array_value = [3; 5];

    println!("array: {array:?}");
    println!("same_array_value: {same_array_value:?}");

    // Access value in array
    let nums = [1, 2, 3, 4, 5];

    let first_value_of_nums = nums[0];
    let second_value_of_nums = nums[1];
    println!("nums: {nums:?}");
    println!("first_value_of_nums: {first_value_of_nums}");
    println!("second_value_of_nums: {second_value_of_nums}");

    println!("==========Functions=========");
    // Functions
    functions::print_func();

    println!("==========Loops=========");
    // Loops
    loops::print_loops();

    println!("==========Scopes=========");
    // Scopes
    scopes::print_scopes();
    ownership_and_function::run_ownership();
    return_values_and_scope::return_values_and_scope();
    return_values_and_scope::dont_take_ownership();
    return_values_and_scope::dont_take_ownership_with_reference();

    // Reference & Borrowing
    reference_and_borrowing::run_reference_and_borrowing();

    // Dangling References
    dangling_references::run_dangling_references();

    println!("==========The Slice Type=========");
    // The Slice Type
    the_slice_type::run_slice_type();

    println!("==========Structs=========");
    // Structs
    structs::run_structs();

    println!("==========Struct - Methods=========");
    // Struct Methods
    struct_method::run_struct_methods();

    println!("==========Enums=========");
    enums::run_enums()
}
