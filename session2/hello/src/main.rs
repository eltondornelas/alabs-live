fn hello_thread() {
    println!("Hello from thread");
}


fn main() {
    println!("Hello with from the main thread");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap(); 
    // when you join a thread you wait it to finish (join); if you don't it may detach right away with the main thread or it may take a little longer;
    // if you comment the join command and run it you can see that sometimes it does not show the thread execution print
    // when you start a thread you have no control how it will be schedule, the operating system will do for you
    // threads must be threated as a independent program, the data comes in do something then comes out
}
