async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
    }
}

async fn tocker() {
    for i in 0..10 {
        println!("tock {i}");
    }
}

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(ticker()),
        tokio::spawn(tocker()),
    );
}
