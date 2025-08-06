use std::time::Duration;

// another tracing big feature is instrumentation
#[tracing::instrument]
async fn hello_world() {
    println!("Hello!");
    tokio::time::sleep(Duration::from_secs(1)).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing_subscriber::fmt::format::FmtSpan;
    // turning on tracing
    // let subscriber = tracing_subscriber::FmtSubscriber::new(); // just creates

    // let subscriber = tracing_subscriber::fmt().json() // if want to be in json needs to add the feature json on the toml
    let subscriber = tracing_subscriber::fmt() // another way of creating with more configurations
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path) "logscan" in this case
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber)?; // setting to be the global default

    // emitting events
    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("Something went horribly wrong");
    hello_world().await;

    Ok(())
}

// cargo add tracing
// cargo add tracing-subscriber
// Tokio Console