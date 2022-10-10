use anyhow::Result;
use pow::service;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8889";
    println!("Listening on {:?}", addr);

    service::launch_server(addr).await?;

    Ok(())
}
