enum Message {
    Tick(u32),
}

async fn sender(tx: tokio::sync::mpsc::Sender<Message>, n: u32) {
    loop {
        tx.send(Message::Tick(n)).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn receiver(
    mut rx: tokio::sync::mpsc::Receiver<Message>,
    mut rx2: tokio::sync::mpsc::Receiver<Message>
) 
{
    loop {
        tokio::select! {
            Some(Message::Tick(n)) = rx.recv() => println!("Received message {n}"),
            Some(Message::Tick(n)) = rx2.recv() => println!("Received message {n}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);
    let (tx2, rx2) = tokio::sync::mpsc::channel::<Message>(100);
    tokio::spawn(sender(tx, 1));
    tokio::spawn(sender(tx2, 2));
    receiver(rx, rx2).await;
}
