fn test() {
    println!("test");
}

fn main() {
    // explicitly sized pool; limiting what rayon create since by default is the number of cpu of your system
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // pool.join(test, test);

    /*
       pool.scope(|scope| {
           scope.spawn_broadcast(|_scope, broadcast_context| {
               println!("Hello from broadcast threast {}", broadcast_context.index());
           }); // creates a copy of your function for every single thread you have in the pool
       })
    */

    pool.spawn(|| println!("Hello from pool thread"));
    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                // rayon allow create more pools, could put the line 3 here for example
                println!("Hello from scoped thread {n}");
            });
        }
    });

    println!("Hello from the main thread");
    // don't need to join since Rayon do that already since it is task oriented
}
