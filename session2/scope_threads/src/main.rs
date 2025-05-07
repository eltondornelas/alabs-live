use std::thread;

fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let sum = thread::scope(|s| {
        // this time thread is inside scopes
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            // spawning into a scope and not globably like last examples
            // rust knows that scope must end and for that you don't need to use .to_owned(), can use as reference
            let thread_handle = s.spawn(move || chunk.iter().sum::<u32>());

            thread_handles.push(thread_handle);
            //the scope will join when all of the threads complete
        }

        thread_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });

    println!("Sum: {sum}");
}
