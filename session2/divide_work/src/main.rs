fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();

    let chunks = to_add.chunks(N_THREADS);
    // do not create new vectors reallocating, the use the slice rust system to references the parts of the original vector
    // println!("{:?}", chunks);

    for chunk in chunks {
        // chunk is a slice/reference to part of a vector of u32's
        // println!("{:?}", chunk); // 625 vectors with 8 indexes
        // we want to each chunk to be owned by the thread that is working on; for now not worrying about borrowing since in this case borrowing is a bit more complex
        let my_chunk = chunk.to_owned(); // similar to clone
        thread_handles.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    // Total of each chunk's sum
    let mut sum = 0;
    for handle in thread_handles {
        let handle_sum = handle.join().unwrap();
        // println!("{handle_sum}"); // 625 values
        sum += handle_sum // n(n+1)/2 = 12497500
    }

    println!("Sum is {sum}")
}
