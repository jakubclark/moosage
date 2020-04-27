use moosage_common::{chat, chat::chat_service_client::ChatServiceClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatServiceClient::connect("http://[::1]:50051").await?;
    let mut stream = client.subscribe(chat::Empty {}).await?.into_inner();

    let handle = tokio::spawn(async move {
        while let Some(message) = stream
            .message()
            .await
            .expect("Could not fetch next message")
        {
            println!("{}: {}", message.user, message.text);
        }
    });

    handle.await?;

    Ok(())
}
