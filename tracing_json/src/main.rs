#[tokio::main]
async fn main() {
    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt().json()
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Log some events
    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("This is an error!");
}
