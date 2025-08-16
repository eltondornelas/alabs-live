mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(collector::data_collector());

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.

    Ok(())
}

// data collection server as high performance server
// need to run here as server and run the ../collector project to see it working
