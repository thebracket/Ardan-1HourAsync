use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello() {
    println!("Hello World");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Applications that receive events need to subscribe
    //let subscriber = tracing_subscriber::FmtSubscriber::new();

    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Add span events
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber)?;

    hello().await;
    Ok(())
}