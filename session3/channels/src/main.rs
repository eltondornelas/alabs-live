use std::{sync::mpsc, time::Duration};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);
    let handle = tokio::runtime::Handle::current(); // a way to sync function call to a async one

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    let tx_reply = tx_reply.clone(); // reply channel can be cloned since it is "multi-sender and single receiver"
                    handle.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                    /* can not await since it's not inside a asynchtonous context, that is outside tha function,
                    meanwhile inside the handle (function) you are because the spawn will move into the runtime and run the future in there
                    */
                    // println!("{s}") // sync
                }
            }
        }
    });

    // Receive messages
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("{reply}");
        }
    });

    // Lauch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }

    // broadcast
    // let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);
    // tx.send("hello".to_string());

    // tokio::spawn(async move {
    //     while let Some(r) = rx.recv().await {
    //         println!("{r}");
    //     }
    // });
}
