use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something_sync() {
    println!("Not async!");
}

async fn say_hello() {
    println!("Hello");
    // second_function().await; // whenever await the function that are currently in, pauses and the next function is enqueued as a task
    join!(second_function(), say_goodbye()); // like Mono.zip() in webflux

    let n = double(4).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;
    println!("{results:?}");

    do_something_sync();
}

async fn second_function() {
    println!("Hello again");
}

async fn say_goodbye() {
    println!("Goodbye");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    // let future = say_hello(); // won't work without the futures
    // need a runtime in place, can not run async functions without async runtime tu handle scheduling through the executor to handle events

    block_on(say_hello())
    // handing control over to the runtime, running by the context of this block_on

    // by default futures are single thread
}
