use std::sync::mpsc;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>(); // transmitter (sends data into the channel) and receiver (recipient receives data from the channel)

    let handle = std::thread::spawn(move || {
        // only 1 receiver and n transmitter
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }

    println!("Sending quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
