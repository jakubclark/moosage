use crate::chatter::Chatter;
use moosage_common::chat::chat_service_server::ChatServiceServer;
use tonic::transport::Server;
pub(crate) mod chatter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let service = ChatServiceServer::new(Chatter::new());

    println!("🐮 Starting moosage server on {:?}", addr);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
