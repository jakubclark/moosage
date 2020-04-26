use async_stream::try_stream;
use moosage_common::chat::chat_service_server::{
    ChatService as ChatServiceTrait, ChatServiceServer,
};
use moosage_common::chat::{ChatMessage, Empty};
use tokio::macros::support::Pin;
use tokio::sync::watch;
use tonic::transport::Server;
use tonic::Status;

#[derive(Debug)]
struct Chatter {
    producer: watch::Sender<ChatMessage>,
    consumer: watch::Receiver<ChatMessage>,
    consumers: Vec<watch::Receiver<ChatMessage>>,
}

impl Chatter {
    pub fn new() -> Self {
        let (producer, consumer) = watch::channel(ChatMessage {
            text: String::from("🐮 The cow goes moo!"),
        });
        Self {
            producer,
            consumer,
            consumers: vec![],
        }
    }
}

type Stream<T> = Pin<
    Box<dyn futures_core::Stream<Item=std::result::Result<T, Status>> + Send + Sync + 'static>,
>;

#[tonic::async_trait]
impl ChatServiceTrait for Chatter {
    type SubscribeStream = Stream<ChatMessage>;

    async fn subscribe(
        &self,
        _: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status> {
        let mut clone = self.consumer.clone();

        let stream = try_stream! {
            while let Some(message) = clone.recv().await {
                yield message
            }
        };

        let resp = tonic::Response::new(Box::pin(stream) as Self::SubscribeStream);

        Ok(resp)
    }
    async fn send_message(
        &self,
        request: tonic::Request<ChatMessage>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let inner = request.into_inner();
        println!("Got a message: {:?}", inner);

        let res = self.producer.broadcast(inner);

        return match res {
            Ok(_) => Ok(tonic::Response::new(Empty {})),
            Err(err) => {
                eprintln!("Error sending! err={:?}", err);
                Err(tonic::Status::internal(String::from("Error sending")))
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let service = ChatServiceServer::new(Chatter::new());

    println!("🐮 Starting moosage server on {:?}", addr);

    Server::builder()
        .add_service(service)
        .serve(addr)
        .await?;

    println!("{:?}", addr);

    Ok(())
}
