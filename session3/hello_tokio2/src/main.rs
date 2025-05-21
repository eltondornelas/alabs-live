async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

async fn hello2() -> u32 {
    println!("Hello Tokio2");
    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    // hello().await;

    // let result = tokio::join!(hello(), hello2());
    // println!("{result:?}");
    
    // let (one, two) = result;

    // tokio::spawn(ticker()); // spawn returns a type that will spawn and runs elsewhere, you can await if want the result to comeback
    // hello().await;

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    ); // multi thread

    println!("Finished");

}
