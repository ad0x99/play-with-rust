fn deliver_order() {
    println!("Delivery order, please wait...")
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("Cooking the order, please wait...")
    }
}
