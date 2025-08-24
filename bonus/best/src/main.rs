// #![warn(clippy::pedantic)]

#[allow(dead_code)]
fn ignore_me() {

}

fn main() {
    println!("Hello, world!");
    for i in 0..17 {
        println!(".)")
    }

    #[rustfmt::skip]  // skipping formatting
    mod section {
        const N:i32 = 1;
    }
}

/*
 * cargo fmt --check --all
 
 * can create a rustfmt.toml
 
 * cargo install cargo-watch
 * cargo watch -x 'fmt' // keep formatting 
 * 
 * cargo clippy
 * #![warn(clippy::pedantic)]  // mini code review; #! = global
 * #[allow(dead_code)]
 * 
*/
