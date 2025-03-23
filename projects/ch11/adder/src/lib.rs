pub fn greeting(name: &str) -> String {
    String::from("hello!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("sqln");
        assert!(result.contains("sqln"));
    }
}
