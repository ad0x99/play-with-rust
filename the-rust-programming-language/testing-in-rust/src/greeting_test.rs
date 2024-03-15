fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

fn greeting_fail(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Thomas");
        assert!(result.contains("Thomas"))
    }

    #[test]
    fn greeting_fail_contains_name() {
        let result = greeting_fail("Thomas");
        assert!(
            result.contains("Thomas"),
            "Greeting did not contain name, value was `{result}`"
        )
    }
}
