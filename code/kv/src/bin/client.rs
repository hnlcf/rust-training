use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kv::proto::{Request, Response};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let addr = "localhost:8889";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let msg = Request::new_put("hello", b"world");
    stream.send(msg.into()).await?;
    let msg = Request::new_get("changfeng");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        info!("Got a repsonse: {:?}", msg);
    }
    Ok(())
}
