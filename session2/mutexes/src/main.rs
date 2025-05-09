use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();

    for (i, _) in (0..10).enumerate() {
        let handle = std::thread::spawn(move || {
            let mut lock = NUMBERS.lock().unwrap();
            let n = i as u32;
            lock.push(n);
        }); // when out of scope the lock is terminated

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();

    println!("{:#?}", lock);
    // mutexes are not that fast as Atomic
}
