use anyhow::Result;
use pow::proto::{pow_builder_client::PowBuilderClient, Block, ClientInfo};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "http://localhost:8889";
    let mut client = PowBuilderClient::connect(addr).await?;

    let mut stream = client
        .subscribe(ClientInfo {
            name: "Client-1".into(),
        })
        .await?
        .into_inner();
    let res = client
        .submit(Block {
            data: b"hello world".to_vec(),
            ..Default::default()
        })
        .await?
        .into_inner();
    println!("Submitted: {:?}", res);

    while let Some(result) = stream.message().await? {
        println!(
            "Result:\nid: {}\nhash: {}\nnonce: {}",
            hex::encode(result.id),
            hex::encode(result.hash),
            result.nonce
        );
    }

    Ok(())
}
