use anyhow::Result;
use futures::Stream;
use std::{collections::HashMap, pin::Pin, sync::Arc, thread};
use tokio::sync::{
    mpsc::{self, Sender},
    RwLock,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    engine,
    proto::{
        pow_builder_server::{PowBuilder, PowBuilderServer},
        Block, BlockHash, BlockStatus, ClientInfo,
    },
};

const BUFFER_SIZE: usize = 8;

/// Table that records all clients subscribing the service.
#[derive(Debug, Default)]
struct Shared {
    clients: HashMap<String, Sender<Result<BlockHash, Status>>>,
}

impl Shared {
    pub fn new() -> Self {
        Self {
            clients: Default::default(),
        }
    }

    /// Send the `BlockHash` calculated to all clients subscribing the service.
    pub async fn broadcast(&self, msg: Option<BlockHash>) {
        for (name, tx) in &self.clients {
            match tx
                .send(
                    msg.clone().ok_or_else(|| {
                        Status::resource_exhausted("Failed to find the suitable hash")
                    }),
                )
                .await
            {
                Ok(()) => (),
                Err(err) => println!(
                    "Broadcast error to {} with {:?}. Error: {:?}",
                    name, msg, err
                ),
            }
        }
    }
}

#[derive(Debug)]
struct PowService {
    /// Send `Block` to PoW engine
    tx: mpsc::Sender<Block>,
    shared: Arc<RwLock<Shared>>,
}

impl PowService {
    pub fn new(tx: mpsc::Sender<Block>, mut rx: mpsc::Receiver<Option<BlockHash>>) -> Self {
        let service = Self {
            tx,
            shared: Arc::new(RwLock::new(Shared::new())),
        };

        let shared = service.shared.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                shared.read().await.broadcast(msg).await;
            }
        });

        service
    }
}

#[tonic::async_trait]
impl PowBuilder for PowService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockHash, Status>> + Send + Sync>>;

    async fn subscribe(
        &self,
        request: Request<ClientInfo>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let name = request.into_inner().name;
        let rx = {
            let (tx, rx) = mpsc::channel(BUFFER_SIZE);
            self.shared.write().await.clients.insert(name, tx);
            rx
        };
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn submit(&self, request: Request<Block>) -> Result<Response<BlockStatus>, Status> {
        let block = request.into_inner();
        match self.tx.send(block.clone()).await {
            Ok(()) => Ok(Response::new(BlockStatus { code: 0 })),
            Err(err) => {
                println!(
                    "Failed to submit {:?} to PoW engine. Error: {:?}",
                    block, err
                );
                Ok(Response::new(BlockStatus { code: 500 }))
            }
        }
    }
}

/// Create [`PowService`], add it to server and launch server.
///
/// Create two channels:
/// - Server(PowService) -> PoW Engine for `Block`
/// - PoW Engine -> Server(PowService) for `BlockHash`
pub async fn launch_server(addr: &str) -> Result<()> {
    let (tx1, mut rx1) = mpsc::channel(BUFFER_SIZE);
    let (tx2, rx2) = mpsc::channel(BUFFER_SIZE);
    thread::spawn(move || {
        while let Some(block) = rx1.blocking_recv() {
            tx2.blocking_send(engine::pow(block)).unwrap();
        }
    });

    let addr = addr.parse().unwrap();
    let service = PowService::new(tx1, rx2);
    Server::builder()
        .add_service(PowBuilderServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
