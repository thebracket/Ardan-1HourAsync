async fn async_line_count(n: u32, filename: String) -> anyhow::Result<()> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;

    println!("Reading {filename}...");

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines(); // Create a stream of lines
    while let Some(line) = lines.next_line().await? {
        println!("{n}: {line}");
    }
    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(async_line_count(1, "warandpeace.txt".to_string())),
        tokio::spawn(async_line_count(2, "warandpeace.txt".to_string())),
        tokio::spawn(async_line_count(3, "warandpeace.txt".to_string())),
    );
}
