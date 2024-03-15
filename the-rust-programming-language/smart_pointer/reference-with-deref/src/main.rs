mod box_t_like_reference;
mod cleanup_with_drop_trait;
mod custom_smart_pointer_with_mybox;
mod deref_coercions_with_fn_and_method;
mod interior_mutability;
mod multiple_owner_of_mutable_data;
mod reference_counter_smart_pointer;
mod regular_reference;
mod structs;

use crate::box_t_like_reference::box_t_like_reference;
use crate::cleanup_with_drop_trait::cleanup_manually_with_drop_fn;
use crate::cleanup_with_drop_trait::cleanup_with_drop_trait;
use crate::custom_smart_pointer_with_mybox::custom_smart_pointer_with_mybox;
use crate::deref_coercions_with_fn_and_method::deref_coercions_with_fn_and_method;
use crate::multiple_owner_of_mutable_data::multiple_owner_of_mutable_data;
use crate::reference_counter_smart_pointer::reference_counter_with_rc;
use crate::reference_counter_smart_pointer::List;
use crate::regular_reference::regular_reference;

fn main() {
    regular_reference();
    box_t_like_reference();
    custom_smart_pointer_with_mybox();
    deref_coercions_with_fn_and_method();
    cleanup_with_drop_trait();
    cleanup_manually_with_drop_fn();
    reference_counter_with_rc();
    multiple_owner_of_mutable_data();
}
