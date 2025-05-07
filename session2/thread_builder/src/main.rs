use std::thread;

fn my_thread() {
    println!(
        "Hello from a thread named {}",
        thread::current().name().unwrap()
    )
}

fn main() {
    thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4) // 64 bits = 8 bytes; the default is 2MB (based on linux) so usually is not necessary inform
        .spawn(my_thread)
        // .unwrap(); // the video stops here; without join it was not guaranteed to print
        .unwrap()
        .join()
        .unwrap();
}
