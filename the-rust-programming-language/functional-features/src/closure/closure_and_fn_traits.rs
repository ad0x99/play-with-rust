pub fn closure_and_fn_traits() {
    sort_rectangle_fn_mut();
    sort_rectangle_fn_once_fixed()
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_rectangle_fn_mut() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("Sort by width: {:#?}", list);
}

fn sort_rectangle_fn_once_error() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // This code attempts to do this counting by pushing value—a String from the closure’s environment—into the sort_operations vector. The closure captures value and then moves value out of the closure by transferring ownership of value to the sort_operations vector.
    // This closure can be called once; trying to call it a second time wouldn’t work because value would no longer be in the environment to be pushed into sort_operations again! Therefore, this closure only implements FnOnce. When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut.
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}

fn sort_rectangle_fn_once_fixed() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);
}
