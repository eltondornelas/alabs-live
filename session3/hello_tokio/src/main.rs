// use tokio::runtime;

async fn hello() {
    println!("Hello Tokio");
}

// #[tokio::main(flavor = "current_thread")] // by default is multi thread, meaning if you want single thread needs to adds this parameter
#[tokio::main]
async fn main() {
    // this way by default is multi thread turning all the features on
    hello().await;
}

// fn main() {
//     // let rt = runtime::Builder::new_current_thread()
//     //     .enable_all()
//     //     .build()
//     //     .unwrap();

//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4)
//         .build()
//         .unwrap();

//     rt.block_on(hello());
// }

// cargo add tokio -F full
