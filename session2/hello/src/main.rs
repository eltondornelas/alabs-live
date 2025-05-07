fn hello_thread(n: u32) {
    println!("Hello from thread {n}");
}


fn main() {
    println!("Hello with from the main thread");

    let mut thread_handles = Vec::new();
    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i)); // move => decorator
        // the thread tends to own the data that goes in, thats why you need to move it, once the scopes only exists inside de loop
        thread_handles.push(thread_handle);
    }

    // when main thread ends you have to wait the other threads to end or it will end right away too
    thread_handles.into_iter().for_each(|h| h.join().unwrap());
    // this proves that it's not deterministic, the order that they will be running is determinated enterily by the Operating System

/* 
    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap(); 
    // when you join a thread you wait it to finish (join); if you don't it may detach right away with the main thread or it may take a little longer;
    // if you comment the join command and run it you can see that sometimes it does not show the thread execution print
    // when you start a thread you have no control how it will be schedule, the operating system will do for you
    // threads must be threated as a independent program, the data comes in do something then comes out
 */
}
