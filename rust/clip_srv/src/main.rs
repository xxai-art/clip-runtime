#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  // let model_dir = var("MODEL_DIR")?;
  Ok(())
}
