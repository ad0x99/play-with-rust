#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run_structs() {
    print_user_struct();
    print_tuple_structs();
    run_area_calculation();
}

fn print_user_struct() {
    let mut user = User {
        active: true,
        username: String::from("ad0x99"),
        email: String::from("ad0x99@gmail.com"),
        sign_in_count: 1,
    };

    // The user instance is mutable, then
    // we can reassign a new value for email field
    user.email = String::from("changed.email@gmail.com");

    println!("{user:#?}");
    // To access specific value from a struct
    // We use dot notation
    println!("email: {}", user.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    // In this point, we can no longer use `user1` after creating user2
    // Because the String in the username was moved to user2
    // println!("{user:#?}");
    println!("{user2:#?}");
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn print_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black color: {:?}", black);
    println!("origin: {:?}", origin);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn run_area_calculation() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect instance: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
