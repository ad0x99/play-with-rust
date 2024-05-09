pub fn method_with_same_name() {
    associated_fn_with_self();
    associated_fn_without_self();
}

fn associated_fn_with_self() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    fn main() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

    main();
}

fn associated_fn_without_self() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    fn main() {
        println!("A baby dog is called a {}", Dog::baby_name());
        // println!("A baby dog is called a {}", Animal::baby_name()); // ERROR

        // Add type annotation to specify which implemented method to be called
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    main();
}
