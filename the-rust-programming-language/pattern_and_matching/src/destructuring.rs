pub fn run_destructuring() {
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    destructuring_structs_and_tuples();
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    fn basic_destructuring() {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;

        println!("x: {a}, y: {b}");
        assert_eq!(0, a);
        assert_eq!(7, b);

        // Shorthand destructuring
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;

        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    fn destructure_literal_values() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    basic_destructuring();
    destructure_literal_values();
}

fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x dir {x}, in the y dir {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
    }
}

fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

fn destructuring_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("feat: {feet}, inches: {inches}, x: {x}, y: {y}")
}
