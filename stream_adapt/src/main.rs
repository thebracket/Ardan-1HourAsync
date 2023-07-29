use pin_project_lite::pin_project;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio_stream::StreamExt;

pin_project! {
    struct ToUpper {
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}

impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}

impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx).map(|opt| {
            opt.map(|res| {
                res.map(|line| {
                    line.to_uppercase() + "\n"
                })
            })
        })
    }
}

#[tokio::main]
async fn main() {
    let file = tokio::fs::File::open("Cargo.toml").await.unwrap();
    // convert the `AsyncRead` into a buffered reader, then a line stream, then your adapter
    let stream = BufReader::new(file).lines();
    let stream = tokio_stream::wrappers::LinesStream::new(stream);
    let mut stream = ToUpper::new(stream);
    while let Some(line) = stream.next().await {
        print!("{}", line.unwrap());
    }
}
