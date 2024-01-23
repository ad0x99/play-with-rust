use std::fmt::Result;
use std::io::Result as IoResult;

pub fn function1() -> Result {
    Ok(println!("Do something in fn 1"))
}

pub fn function2() -> IoResult<()> {
    Ok(println!("Do something in fn 2"))
}