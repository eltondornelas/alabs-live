fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is unparked, briefly");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || {
            parkable_thread(i);
        });

        threads.push(thread);
    }

    loop {
        println!("Thread to unpark: ");
        let input = read_line();
        if input == "q" {
            break;
        }

        if let Ok(number) = input.parse::<usize>() {
            // turbofish syntax
            if number < 10 {
                threads[number].thread().unpark();
            }
        }
    }
}

// parking -> create a thread and don't want to it do anything at all until it awakened up from the outside
// threads can park itself and you can unpark from the outside, you can externally park the thread
// when parking gotta make sure that at some point you unpark or terminate the process
