use std::slice;

pub fn unsafe_function_and_method() {
    calling_unsafe_function();
    safe_abstraction_over_unsafe_code();
    using_extern_function_to_call_external_code();
}

fn calling_unsafe_function() {
    unsafe fn dangerous() {
        println!("Running dangerous function...it should be run in unsafe block.")
    }

    unsafe {
        dangerous();
    }

    // Calling the dangerous function outside of unsafe block is not allowed.
    // dangerous();
}

fn safe_abstraction_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let raw_pointer = &mut v[..];

    let (a, b) = split_at_mut(raw_pointer, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("Values of slice a is: {:?}", a);
    println!("Values of slice b is: {:?}", b);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let pointer = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), len - mid),
        )
    }
}

// Error: cannot borrow `*values` as mutable more than once at a time
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (&mut values[..mid], &mut values[mid..])
// }

fn using_extern_function_to_call_external_code() {
    // Within the extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level.
    extern "C" {
        fn abs(input: i32) -> i32;

    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    main();

    // Use extern to create an interface that allows other languages to call Rust functions
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}
