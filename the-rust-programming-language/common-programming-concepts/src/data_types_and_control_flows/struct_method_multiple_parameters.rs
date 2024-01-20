#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn run_method_multiple_parameters() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // We passed `&rect2`, which is an immutable borrow to rect2, an instance of Rectangle
    // This makes sense because we only need to read `rect2` (rather than write, which would mean we’d need a mutable borrow), and we want main to retain ownership of `rect2` so we can use it again after calling the `can_hold` method.
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

pub fn run_associated_function() {
    let sq = Rectangle::square(3);
    println!("The square of the rectangle is {:#?}", sq);
}
