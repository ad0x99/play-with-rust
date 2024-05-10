pub fn type_synonyms_with_type_aliases() {
    basic_definition();
    reduce_repetition();
}

fn basic_definition() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn reduce_repetition() {
    // Without Type Alias
    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| {
    //     println!("hi");
    // });

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}

    // Using Type Alias
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {}
    fn _returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
}
