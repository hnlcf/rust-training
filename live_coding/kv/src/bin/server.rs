use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use kv::proto::{request::Command, Request, RequestGet, RequestPut, Response};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let addr = "0.0.0.0:8889";
    let listener = TcpListener::bind(addr).await?;
    let server_state = Arc::new(ServerState::new());

    info!("Listen to {:?}", addr);
    loop {
        let state = server_state.clone();
        let (stream, addr) = listener.accept().await?;
        info!("New client: {:?}", addr);

        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a message: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match state.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        state.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    None => unimplemented!(),
                };

                stream.send(response.into()).await?;
            }

            Ok::<(), anyhow::Error>(())
        });
    }
}
