// #![warn(clippy::pedantic)]
// #![warn(missing_docs)]

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
 * https://github.com/thebracket/Ardan-NR-2023-07/blob/main/Day4/Documentation.md 
 * #![warn(missing_docs)] // good for libs
 * //! -> syntax indicates a scope-level comment
 * tests created in documented scopes will be tested in compile-time
 * /// -> syntax for function and structure
 * https://rust-lang.github.io/api-guidelines/documentation.html
 * Rust RFC 1574
 * cargo doc --open --no-deps
 * cargo install spellcheck
 * cargo spellcheck
*/
