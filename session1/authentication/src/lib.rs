use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");

    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

// Array (in memory static will not changed length once created) != Vector (are stored in the heap so they are slightly slow to access but still fast and can change size)
/* pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User)
    ]

    // let mut users = Vec::new();
    // users.push(User::new(...))
}
 */

pub fn get_default_users() -> HashMap<String, User> {
    // hashmaps are slower than vectors for insertion but for search they are way faster
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file!
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

/* // example for vecs interating
fn get_admin_users() {
    let users: Vec<String> = get_users()
    .into_iter() // move the original data and the original vector will no longer be able to be used
    .filter(|u| u.role == LoginRole::Admin)
    .map(|u| u.username)
    .collect(); // take w.e passed the filter e turn it to the type and need to be explicit
}
 */

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();

    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    /*  // for vector example
       if let Some(user) = users.iter().find(|user| user.username == username) {
           if user.password == password {
               return Some(LoginAction::Granted(user.role.clone()));
           } else {
               return Some(LoginAction::Denied);
           }
       }
    */
    None
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
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
    }
}

// cargo new --lib authentication
// cargo add serde -F derive
// cargo add serde_json
