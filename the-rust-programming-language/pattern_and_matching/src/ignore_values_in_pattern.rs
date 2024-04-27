pub fn run_ignore_values() {
    entire_underscore(3, 4);
    nested_underscore();
    unused_variable_with_underscore();
    remaining_parts_with_three_dot();
}

fn entire_underscore(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn nested_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}

fn unused_variable_with_underscore() {
    let _x = 5;
    // let y = 10; // Got warning

    let s = Some(String::from("Hello!"));

    // In this case, the value s will be moved into _s, which prevents us from using s again
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // This code works just fine because we never bind s to anything; it isn’t moved.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn remaining_parts_with_three_dot() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // unambiguous example with error
    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {second}");
    //     }
    // }
}
