use std::time::Duration;

async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! {
        // neither return anything so is just ignoring with _
        _ = do_work() => println!("do_work finished first"),
        _ = timeout(3.0) => println!("timeout finished first"),

        // select will cancel the futures you don't want, deleting them from the task pool
        // you could put a 1st select item a rest call and the 2nd item the timeout for example...
        // another example is calling samething but for multiple destinations like ask DNS for google, cloudfare, localnetwork... and get what responses first
    }    
}
