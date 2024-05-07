pub fn accessing_or_modifying_mutable_static_variable() {
    static_variable();
}

fn static_variable() {
    // Static variable
    static HELLO_WORLD: &str = "Hello, world";

    println!("Value of static variable is: {HELLO_WORLD}");

    // Mutable static variable
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe { println!("Value of COUNTER is: {COUNTER}") };
}

// Implementing an Unsafe Trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
