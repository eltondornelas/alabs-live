use std::fmt::format;

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn greet(s: String) {
    println!("Hello {s}")
}

fn greet_borrow(s: &String) {
    // dont give me THE variable, just give me access to the variable but you still owned it
    println!("{s}")
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
}

fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

fn main() {
    /*
       let n : i32 = 5;

       // n+=1;
       {
           let n: i32 = 6;
           println!("{n}")
       }

       print!("{n}");

       let name = "notle".to_string();
       let mut name_mut = "notle".to_string();
       // greet(name.clone()); // better avoid because clone can be slow, ideal is to borrow
       greet_borrow(&name); // you can have as many imutable borrows; in case of mutable that means allowing the function to change the date, can only do this once
       greet_borrow_mut(&mut name_mut);
       greet(name);
       // greet(name);
    */

    let input: String = read_line();
    println!("You typed: [{input}]")
}
