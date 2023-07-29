use std::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(100);
    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        let mut n = 0;
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let my_tx = tx.clone();
            handle.spawn(async move {
                my_tx.send(n).await.unwrap();
            });
            n += 1;
        }
    });

    while let Some(n) = rx.recv().await {
        println!("Received {n} from the system thread");
    }
}
