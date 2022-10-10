use anyhow::Result;
use std::fmt::Debug;
use tokio::sync::{mpsc, oneshot};

pub struct Actor<State, Request, Reply> {
    /// Receiver corresponding to the sender in PID
    pub receiver: mpsc::Receiver<ActorMessage<Request, Reply>>,

    /// Actor state
    pub state: State,
}

impl<State, Request, Reply> Actor<State, Request, Reply>
where
    State: Default + Send + 'static,
    Request: HandleCall<Reply = Reply, State = State> + Send + 'static,
    Reply: Debug + Send + 'static,
{
    /// Create a pair of Actor and PID with a mspc channel, whose max number is `buffersize`
    ///
    /// Move the actor to a new async task and return the PID
    pub fn spawn(buffersize: usize) -> Pid<Request, Reply> {
        let (sender, receiver) = mpsc::channel(buffersize);
        let mut actor: Actor<State, Request, Reply> = Actor {
            receiver,
            state: State::default(),
        };

        tokio::spawn(async move {
            while let Some(msg) = actor.receiver.recv().await {
                let reply = msg.data.handle_call(&mut actor.state).unwrap();
                msg.sender.send(reply).unwrap();
            }
        });

        Pid { sender }
    }
}

#[derive(Debug)]
pub struct ActorMessage<Request, Reply> {
    /// Message data to be sent to the actor
    data: Request,

    /// Oneshot sender for reply
    sender: oneshot::Sender<Reply>,
}

pub trait HandleCall {
    type State;
    type Reply;

    /// Call by Actor. Handle the data received from the PID and return the reply
    fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply>;
}

#[derive(Debug, Clone)]
pub struct Pid<Request, Reply> {
    /// Sender corresponding to the receiver in Actor
    sender: mpsc::Sender<ActorMessage<Request, Reply>>,
}

impl<Request, Reply> Pid<Request, Reply> {
    /// Send a message to the actor and return the reply
    pub async fn send(&self, data: Request) -> Result<Reply> {
        let (sender, receiver) = oneshot::channel();
        let msg = ActorMessage { data, sender };
        let _ = self.sender.send(msg).await;
        Ok(receiver.await?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    impl HandleCall for usize {
        type State = usize;
        type Reply = usize;

        fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply> {
            *state += 1;
            println!("State: {:?}", state);
            Ok(self + 1)
        }
    }

    #[tokio::test]
    async fn test_spawn() {
        let pid: Pid<usize, usize> = Actor::spawn(20);
        let result = pid.send(42).await.unwrap();
        assert_eq!(result, 43);

        let pid1 = pid.clone();
        let result = pid1.send(99).await.unwrap();
        assert_eq!(result, 100);
    }
}
