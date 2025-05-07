use std::sync::atomic::AtomicI32;

// static mut COUNTER: i32 = 0;
// exterior mutability

static COUNTER: AtomicI32 = AtomicI32::new(0);
// interior mutability (no mut); if the type can change itself but the outside doens't change; Atomic works only with numeric types

fn main() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // since you can't control the orders the thread are running, don't need to use SeqCst (sequential operation) in this exampls
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}

/*
fn using_unsafe_method() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    unsafe {
        println!("{COUNTER}")
        // in the video it compiles; and every run it shows a different result
        // this shows the "data race" problem, where the value wasn't updated and back to memory before the new thread takes it updated, meaning the value will be out of order
    }
}
 */
