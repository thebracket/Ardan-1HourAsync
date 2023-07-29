use rand::Rng;

async fn sleep_random() {
    let mut rng = rand::thread_rng();
    let secs = rng.gen_range(0..5);
    tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
}

#[tokio::main]
async fn main() {
    for _ in 0..10 {
        tokio::select! {
            _ = sleep_random() => println!("Task 1 Returned"),
            _ = sleep_random() => println!("Task 2 Returned"),
            _ = sleep_random() => println!("Task 3 Returned"),
        }
    }
}
