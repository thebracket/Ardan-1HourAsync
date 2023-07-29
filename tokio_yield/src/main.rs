async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

async fn tocker() {
    for i in 0..10 {
        println!("tock {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(ticker()),
        tokio::spawn(tocker()),
    );
}
