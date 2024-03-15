// Error
// pub enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
