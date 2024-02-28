mod closure_and_fn_traits;
mod closure_definition_locked_type;
mod inventory;
mod references_and_borrowing_in_closure;

pub fn closure() {
    inventory::run_inventory();
    closure_definition_locked_type::run_closure_locked_type();
    references_and_borrowing_in_closure::references_and_borrowing();
    closure_and_fn_traits::closure_and_fn_traits()
}
