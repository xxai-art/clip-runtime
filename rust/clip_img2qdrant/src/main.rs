use anyhow::Result;
use clip_qdrant::qdrant_client;

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;

  println!("Hello, world!");
  Ok(())
}
