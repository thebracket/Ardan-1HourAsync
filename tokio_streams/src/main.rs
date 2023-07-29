use tokio_stream::StreamExt;

struct MyStream {
    counter: u32,
}

impl tokio_stream::Stream for MyStream {
    type Item = u32;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        self.counter += 1;
        // Pretend there's some work here
        for _ in 0..1000 {
        }
        if self.counter < 100 {
            std::task::Poll::Ready(Some(self.counter))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

async fn ticker() {
    loop {
        print!("T");
        tokio::task::yield_now().await;
    }
}

async fn streamer() {
    let mut stream = MyStream { counter: 0 };
    while stream.next().await.is_some() {
        print!(".");
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(ticker());
    streamer().await;
}
