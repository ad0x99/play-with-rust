#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum DetailIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


pub fn run_enums() {
    println!("==========print_enum_usage()=========");
    print_enum_usage();
    println!("==========print_enum_usage()=========");

    println!("==========print_enum_struct()=========");
    print_enum_struct();
    println!("==========print_enum_struct()=========");

    println!("==========print_enum_method()=========");
    print_enum_method();
    println!("==========print_enum_method()=========");

    println!("==========print_option()=========");
    print_option();
    println!("==========print_option()=========");

    println!("==========value_in_cents()=========");
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("==========value_in_cents()=========");

    println!("==========plus_one()=========");
    print_plus_one();
    println!("==========plus_one()=========");

    println!("==========print_if_let()=========");
    print_if_let();
    println!("==========print_if_let()=========");

    println!("==========print_if_let_else()=========");
    print_if_let_else(Coin::Quarter(UsState::Alabama));
    println!("==========print_if_let_else()=========");
}

fn print_enum_usage() {
    let v4 = IpAddressKind::V4;
    let v6 = IpAddressKind::V6;


    println!("IP Address Kind: {:?}", v4);
    println!("IP Address Kind: {:?}", v6);
}

fn print_enum_struct() {
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
    println!("Home IP Address: {:#?}", home);
    println!("Loopback IP Address: {:#?}", loopback);

    let home_1 = IpAddr::V4(String::from("127.0.0.1"));
    let loopback_1 = IpAddr::V6(String::from("::1"));
    println!("Home 1 IP Address: {:#?}", home_1);
    println!("Loopback 1 IP Address: {:#?}", loopback_1);

    let home_2 = DetailIpAddr::V4(127, 0, 0, 1);
    let loopback_2 = DetailIpAddr::V6(String::from("::1"));
    println!("Home 2 IP Address: {:#?}", home_2);
    println!("Loopback 2 IP Address: {:#?}", loopback_2);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling... {:?}", &self);
    }
}

fn print_enum_method() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn print_option() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {some_number:#?}");
    println!("some_char: {some_char:#?}");
    println!("absent_number: {absent_number:#?}");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("{}", 1);
            1
        }
        Coin::Nickel => {
            println!("{}", 5);
            5
        }
        Coin::Dime => {
            println!("{}", 10);
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {six:?}");
    println!("none: {none:?}");
}

fn print_if_let() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("match - The maximum is configured to be {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("if let - The maximum is configured to be {max}");
    }
}

fn print_if_let_else(coin: Coin) {
    let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => println!("match - State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("if let else - State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

