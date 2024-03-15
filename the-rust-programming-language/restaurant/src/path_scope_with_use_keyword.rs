// This won't work because
// the`use`is no longer applied in customer’s scope,
// then the compiler will throw the error.
// use crate::front_of_house::hosting;
pub mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

