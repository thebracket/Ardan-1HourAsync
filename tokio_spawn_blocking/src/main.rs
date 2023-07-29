use tokio::task::spawn_blocking;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
}

async fn slow_counter() -> usize {
    spawn_blocking(move || {
        (2 .. 100_000).filter(|&x| is_prime(x)).count()
    }).await.unwrap()
}

async fn ticker() {
    loop {
        println!("Still alive!");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
 async fn main() {
    tokio::spawn(ticker());
    let counted_primes = slow_counter().await;
    println!("{counted_primes}");
}
