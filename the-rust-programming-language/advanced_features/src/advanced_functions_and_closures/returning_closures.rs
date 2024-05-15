// Error
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// Using trait object instead
pub fn returning_closures() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
