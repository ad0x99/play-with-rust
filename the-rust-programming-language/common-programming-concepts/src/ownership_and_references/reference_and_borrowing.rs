pub fn run_reference_and_borrowing() {
    println!("==========reference_and_borrowing()=========");
    reference_and_borrowing();

    println!("==========only_once_mutable_reference_at_a_time()=========");
    only_once_mutable_reference_at_a_time();

    println!("==========multiple_mutable_references()=========");
    multiple_mutable_references();

    println!("==========mutable_and_immutable_references()=========");
    mutable_and_immutable_references();

    println!("==========reference_scopes()=========");
    reference_scopes();
}

fn reference_and_borrowing() {
    // First we make the variable as mutable with `mut` keyword
    let mut s = String::from("hello");

    // Then, we create a mutable reference with `&mut s` where we call the change function
    change(&mut s);
}

// And update the function signature to accept a mutable reference with `some_string: &mut String`.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn only_once_mutable_reference_at_a_time() {
    let mut s = String::from("hello");

    // This will be failed
    // Because cannot borrow `s` as mutable more than once at a time
    let r1 = &mut s;
    // let r2 = &mut s;

    println!("r1: {r1}");
    // println!("{r1}, {r2}");
}

fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;
}

fn mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}");
    // println!("{r1}, {r2}, and {r3}");
}

fn reference_scopes() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}
