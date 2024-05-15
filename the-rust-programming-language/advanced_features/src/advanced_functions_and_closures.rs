mod function_pointers;
mod returning_closures;

pub fn run_advanced_functions_and_closures() {
    function_pointers::function_pointers();
    let _ = returning_closures::returning_closures();
}
