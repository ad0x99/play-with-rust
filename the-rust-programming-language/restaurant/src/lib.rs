mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/*
* Using shortcut path
*/
// use front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     // Using shortcut path
//     hosting::add_to_waitlist();
// }

/*
* Bringing paths into scope with the `use` keyword
*/
// use crate::front_of_house::hosting;
//
// mod customer {
//     use crate::front_of_house::hosting;
//
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }


/*
* Relative path with  `super`
*/
// fn deliver_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
// }


/*
* Using `pub` with struct
*/
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast.
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//
//     // Change our mind about what bread we'd like.
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     // The next line won't compile if we uncomment it; we're not
//     // allowed to see or modify the seasonal fruit that comes
//     // with the meal.
//     // meal.seasonal_fruit = String::from("blueberries");
// }

/*
* Using `pub` with enums
*/
// mod back_of_house {
//     #[derive(Debug)]
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
//
//     println!("I'd like to order {:?} please", order1);
//     println!("I'd like to order {:?} please", order2);
// }

