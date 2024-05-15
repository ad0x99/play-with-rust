pub fn function_pointers() {
    function_pointer();
    closures_or_functions();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointer() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

fn closures_or_functions() {
    // Using closures
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    println!("List of strings: {:?}", list_of_strings);

    // Using functions
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("List of strings: {:?}", list_of_strings);

    assert_eq!(list_of_strings, list_of_strings);

    // Using function argument in Enum
    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("List of status: {:?}", list_of_statuses);
}
