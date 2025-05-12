fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handlers = core_ids.into_iter().map(|id| {
        std::thread::spawn(move || {
            let success = core_affinity::set_for_current(id); // some OP might not let you put into affinity to CPU core 0 (MACOS unless you are running in root) since quite a lot of system program runs there
            if success {
                println!("Hello from a thread on core {id:?}");
            } else {
                println!("Unnable to set affinity to core {id:?}");
            }

        })
    }).collect::<Vec<_>>(); // _ = doens't matter the type it gives

    for handle in handlers.into_iter() {
        handle.join().unwrap();
    }
}
