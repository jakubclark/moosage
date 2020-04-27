use async_stream::try_stream;
use moosage_common::chat::chat_service_server::ChatService as ChatServiceTrait;
use moosage_common::chat::{ChatMessage, Empty};
use std::pin::Pin;
use tokio::sync::watch;
use tonic::{Request, Response, Status};

type MessageProducer = watch::Sender<ChatMessage>;
type MessageConsumer = watch::Receiver<ChatMessage>;

#[derive(Debug)]
pub struct Chatter {
    producer: MessageProducer,
    consumer: MessageConsumer,
}

impl Chatter {
    pub fn new() -> Self {
        let (producer, consumer) = watch::channel(ChatMessage {
            text: String::from("🐮 The cow goes moo!"),
            user: String::from("master-cow"),
        });
        Self { producer, consumer }
    }
}

type Stream<T> = Pin<
    Box<dyn futures_core::Stream<Item = std::result::Result<T, Status>> + Send + Sync + 'static>,
>;

type MessageStream = Stream<ChatMessage>;

#[tonic::async_trait]
impl ChatServiceTrait for Chatter {
    type SubscribeStream = MessageStream;

    async fn subscribe(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let mut clone = self.consumer.clone();
        let stream = try_stream! {
            while let Some(message) = clone.recv().await {
                yield message
            }
        };

        let resp = Response::new(Box::pin(stream) as Self::SubscribeStream);

        Ok(resp)
    }

    async fn send_message(&self, request: Request<ChatMessage>) -> Result<Response<Empty>, Status> {
        let message = request.into_inner();
        println!("Broadcasting message: {:?}", message);
        let res = self.producer.broadcast(message);

        match res {
            Ok(_) => Ok(tonic::Response::new(Empty {})),
            Err(err) => {
                eprintln!("Error sending! err={:?}", err);
                Err(tonic::Status::internal(String::from("Error sending")))
            }
        }
    }
}