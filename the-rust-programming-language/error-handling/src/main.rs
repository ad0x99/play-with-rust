mod unrecoverable_error;
mod recoverable_error;
mod propagating_error;

fn main() {
    println!("========Unrecoverable Errors========");
    // unrecoverable_error::run_unrecoverable_error();

    println!("========Recoverable Errors========");
    // recoverable_error::run_unrecoverable_error();

    println!("========Propagating Errors========");
    propagating_error::run_propagating_error().expect("Error");
}
