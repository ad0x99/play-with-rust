mod dereferencing_a_raw_pointer;
mod mutable_static_variable;
mod unsafe_function_and_method;

pub fn run_unsafe_rust() {
    dereferencing_a_raw_pointer::dereferencing_raw_pointer();
    unsafe_function_and_method::unsafe_function_and_method();
    mutable_static_variable::accessing_or_modifying_mutable_static_variable();
}
