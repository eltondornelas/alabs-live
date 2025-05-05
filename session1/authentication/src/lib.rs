pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");

    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Dornelas", greet_user("Dornelas"));
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(login("ADMIN", "password"));
        assert!(!login("abc", "def"));
        assert!(!login("123", "456"));
    }
}

// cargo new --lib authentication
