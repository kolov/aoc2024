pub mod day1;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    day1::main().await
}


