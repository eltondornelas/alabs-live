use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn main() {
    let my_shared = Mutex::new(0);
    /*
       // example 1 -> deadlock
       let lock = my_shared.lock().unwrap();
       let lock = my_shared.lock().unwrap();
       // this way it never gos out the "red light" because the first lock hasn't be released and you enter into a deadlock
    */

    /*
       // example 2 -> avoiding deadlock with scope
       {
           let lock = my_shared.lock().unwrap();
           // when lock leaves scope thats when it gets unlocked
       }
       let lock = my_shared.lock().unwrap();
    */

    /*
       // example 3 -> avoiding deadlock using try_lock
       // can it be my turn now?
       let lock = MY_SHARED.lock().unwrap();
       std::mem::drop(lock); // if not use this the lock is not dropped/realeased; this example need to be explicit; comment this and will only get "No lock"; it's like droping out of scope

       if let Ok(_lock) = MY_SHARED.try_lock() { // avoid hanging the program, but need to explicit drop the lock above
           println!("Got the lock");
       } else {
           println!("No lock");
       }
    */

    // example 4 -> poisoning
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join()); // no unwrap to see error

    let lock = MY_SHARED.lock(); // no unwrap to see error
    println!("{lock:?}");

    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner()
    });

    println!("data recovered = {recovered_data}")
}

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes")
}
