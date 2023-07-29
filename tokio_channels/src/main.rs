enum Message {
    Tick,
}

async fn sender(tx: tokio::sync::mpsc::Sender<Message>) {
    loop {
        tx.send(Message::Tick).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn receiver(mut rx: tokio::sync::mpsc::Receiver<Message>) {
    while let Some(message) = rx.recv().await {
        match message {
            Message::Tick => println!("Tick"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);
    tokio::spawn(sender(tx));
    receiver(rx).await;
}
