use authentication::{greet_user, login, read_line};

fn main() {
    let mut tries: i32 = 0;

    loop {
        println!("Enter your username:");
        let username: String = read_line();

        println!("Enter your password:");
        let password: String = read_line();

        if login(&username, &password) {
            println!("Welcome");
            break;
        } else {
            println!("Incorrect username or password");
            tries += 1;

            if tries >= 3 {
                println!("Too many failed logins");
                break;
            }
        }
    }
}
