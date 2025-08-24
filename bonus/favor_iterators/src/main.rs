use std::ops::Index;

struct Row {
    language: String,
    message: String,
}

fn get_rows() -> Vec<Row> {
    vec![
        Row { language : "English".to_string(), message : "Hello".to_string() },
        Row { language : "French".to_string(), message : "Bonjour".to_string() },
        Row { language : "Spanish".to_string(), message : "Hola".to_string() },
        Row { language : "Russian".to_string(), message : "Zdravstvuyte".to_string() },
        Row { language : "Chinese".to_string(), message : "Nǐn hǎo".to_string() },
        Row { language : "Italian".to_string(), message : "Salve".to_string() },
        Row { language : "Japanese".to_string(), message : "Konnichiwa".to_string() },
        Row { language : "German".to_string(), message : "Guten Tag".to_string() },
        Row { language : "Portuguese".to_string(), message : "Olá".to_string() },
        Row { language : "Korean".to_string(), message : "Anyoung haseyo".to_string() },
        Row { language : "Arabic".to_string(), message : "Asalaam alaikum".to_string() },
        Row { language : "Danish".to_string(), message : "Goddag".to_string() },
        Row { language : "Swahili".to_string(), message : "Shikamoo".to_string() },
        Row { language : "Dutch".to_string(), message : "Goedendag".to_string() },
        Row { language : "Greek".to_string(), message : "Yassas".to_string() },
        Row { language : "Polish".to_string(), message : "Dzień dobry".to_string() },
        Row { language : "Indonesian".to_string(), message : "Selamat siang".to_string() },
        Row { language : "Hindi".to_string(), message : "Namaste, Namaskar".to_string() },
        Row { language : "Norwegian".to_string(), message : "God dag".to_string() },
        Row { language : "Turkish".to_string(), message : "Merhaba".to_string() },
        Row { language : "Hebrew".to_string(), message : "Shalom".to_string() },
        Row { language : "Swedish".to_string(), message : "God dag".to_string() },
                
    ]
}

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
}

fn main() {
    let rows = get_rows();
    
    let now = std::time::Instant::now();
    for row in rows.iter() {
        if row.language == "French" {
            println!("{}", row.message);
            break;
        }
    }
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());

    let now = std::time::Instant::now();
    rows.iter()
        .filter(|r| r.language == "French")
        .for_each(|r| println!("{}", r.message));
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());
    //*********************************************************//

    // Working with Data
    /*
        Iterators could be a class unto themselves. It's always worth looking at the operations offered by iterators. 
            'map' can be used to transform data on its way through the pipeline. 'filter_map' can combine filtering and mapping into a single operation. 
            'all', 'any' can be used to see if a predicate matches all or any element. 'skip' and 'nth' let you navigate within the iterator.
            'fold' can apply an accumulator, 'reduce' can shrink your data. With 'chain' and 'zip' you can combine iterators.
        In some cases, it's worth learning to make your own iterators. It's relatively simple (very similar to the stream we made).
        Remember, iterators don't yield. You can turn an iterator into a stream with a helper function from 'tokio-streams' (and also 'futures') if you do need to yield at each step in an async program.
     */
    let now = std::time::Instant::now();
    const MAX:u32 = 200000;
    let mut count = 0;
    for n in 2 .. MAX {
        if is_prime(n) {
            count+=1;
        }
    }
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();
    let count = (2..MAX)
        .filter(|n| is_prime(*n))
        .count();
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());

    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    let now = std::time::Instant::now();
    let count = (2..MAX)
        .into_par_iter()
        .filter(|n| is_prime(*n))
        .count();
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());
    //*********************************************************//

    // Understanding .iter() vs .into_iter()
    let mut v = vec!["one".to_string(), "two".to_string()];
    v.iter().for_each(|v| println!("{}", v.len())); // iterating non-destructively, receiving a reference of a pointer to each item not damaging the original
    println!("{v:?}");

    let mut v = vec!["one".to_string(), "two".to_string()];
    v.into_iter().for_each(|v| println!("{}", v.len())); // convert that collectiong into a iterator, moving everything out of it and taking ownership, that iterator destroy the original vector
    // println!("{v:?}"); // v was moved (destroyed)
}

// cargo add rayon