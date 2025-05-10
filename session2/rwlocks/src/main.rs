use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));
// wrapping types together

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add to the user list (or q to quit)");
        let input: String = read_line();
        if input == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap(); // same as adding a lock from a mutex
            lock.push(input);
        }
    }
}

// read a lot and write ocassionally
// cell -> everything that referes to a unit of memory
// once the main thread quits, the child threads quit and in this case we don't want to join, because would block de program forever since it never ends
// RwLock are more efficient than a Mutex in a frequent read and write pattern.
// needs to Lazy initialize something if the constructor of what you first want to put into there is not constant
