// synchronous
// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n-1) + fibonacci(n-2)
//     }
// }

// use futures::future::BoxFuture;
// use futures::future::FutureExt;
use async_recursion::*;
use std::{future::Future, pin::Pin};

// assynchronous
#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    // 'static -> static lifetime indicating it's going to last forever, pinning it
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

// future
async fn one() {
    println!("One");
}

// future
async fn two() {
    println!("Two");
}

async fn all_one_of_them(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    // Pin -> held in memory;
    // Box -> space of source of data
    // dyn -> dynamic, the box size will be determined at runtime rather than at compile time since it depend of what you put in it
    // Future -> going to receive a Future wich intern is going to be a Output type
    // obs: the tokio macro would be way simple

    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid choice"),
    }
}

#[tokio::main]
async fn main() {
    let n = 10;
    println!("fibonacci({n}) = {}", fibonacci(n).await);

    // let future = async {
    let mut future = async {
        // if want to take a reference need to "mut"
        println!("Hello World");
    };

    tokio::pin!(future);
    (&mut future).await;

    (all_one_of_them(1).await).await;
    ()
}

// Pinning ->  means starting a async data type in a fixed position in a system memory so it can be safely used by multiple async tasks
// cargo add futures
// cargo add async_recursion
