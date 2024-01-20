pub fn print_loops() {
    println!("==========return_value_from_loop=========");
    return_value_from_loop();
    println!("==========return_value_from_loop=========");

    println!("==========loop_label=========");
    loop_label();
    println!("==========loop_label=========");

    println!("==========for_loop=========");
    for_loop();
    println!("==========for_loop=========");
}

fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

fn loop_label() {
    let mut count = 0;

    // This outer loop has the label `counting_up`
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}