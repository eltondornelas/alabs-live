use std::time::Instant;

use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i != 0)
    // inclusive range
}

fn main() {
    // let numbers: Vec<u64> = (0..1_000_000).collect();
    // let sum = numbers.par_iter().sum::<u64>(); // parallel iterator; like the example we used chunks
    // println!("Sum {sum}");

    let now = Instant::now();
    let numbers: Vec<u32> = (0..1000).collect();
    let mut primes: Vec<&u32> = numbers.par_iter().filter(|n| is_prime(**n)).collect();
    primes.par_sort_unstable(); // not worried about memory usage

    let elapsed = now.elapsed().as_secs_f32();
    println!("Found {} primes in {} secods", primes.len(), elapsed);
}
