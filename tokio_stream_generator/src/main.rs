use tokio_stream::StreamExt;

struct Doubler {
    counter: u32,
}

impl tokio_stream::Stream for Doubler {
    type Item = u32;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        self.counter *= 2;
        if self.counter < u32::MAX/2 {
            std::task::Poll::Ready(Some(self.counter))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

#[tokio::main]
async fn main() {
    let mut stream = Doubler { counter: 1 };
    while let Some(n) = stream.next().await {
        println!("{n}");
    }
}
