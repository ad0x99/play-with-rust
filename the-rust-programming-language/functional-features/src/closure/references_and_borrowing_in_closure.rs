use std::thread;

pub fn references_and_borrowing() {
    immutable_reference();
    mutable_reference();
    force_ownership_with_move_keyword()
}

fn immutable_reference() {
    let list = vec![1, 2, 3];
    println!("Immutable Ref - Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Immutable Ref - Before calling closure: {:?}", list);
    only_borrows();
    println!("Immutable Ref - After calling closure: {:?}", list);
}

fn mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Mutable Ref - Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Mutable Ref - Before calling closure: {:?}", list); // error
    borrows_mutably();
    println!("Mutable Ref - After calling closure: {:?}", list);
}

fn force_ownership_with_move_keyword() {
    let list = vec![1, 2, 3];
    println!("Move - Before defining closure: {:?}", list);

    // The new thread might finish before the rest of the main thread finishes, or the main thread might finish first.
    // If the main thread maintains ownership of list but ends before the new thread and drops list, the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure given to the new thread so the reference will be valid.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
