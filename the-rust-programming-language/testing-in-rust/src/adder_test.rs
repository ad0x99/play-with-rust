fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_three(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_adds_two_success() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn it_adds_three_failed() {
        assert_eq!(4, add_three(2))
    }
}
