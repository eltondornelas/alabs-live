use authentication::{LoginAction, login, read_line};

fn main() {
    let mut tries: i32 = 0;

    loop {
        println!("Enter your username:");
        let username: String = read_line();

        println!("Enter your password:");
        let password: String = read_line();

        match login(&username, &password) {
            // LoginAction::Granted(authentication::LoginRole::Admin) => println!("Admin"),            
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }

            Some(LoginAction::Denied) => {
                // Do nothing
            }

            None => {
                println!("New user system")
            }
        }

        println!("Incorrect username or password");
        tries += 1;

        if tries >= 3 {
            println!("Too many failed logins");
            break;
        }
    }
}
