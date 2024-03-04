mod closure_captures_their_env;
mod methods_produce_other_iterators;
mod process_series_of_items;

pub fn iterators() {
    process_series_of_items::run_iterator();
    methods_produce_other_iterators::methods_produce_other_iterators();
    closure_captures_their_env::close_captures_their_environment()
}
