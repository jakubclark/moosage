use moosage_common::chat::{ChatMessage, User, Uuid};
mod chatter;
use crate::chatter::BlockingChatter;
use uuid::Uuid as UuidGenerator;

fn main() -> anyhow::Result<()> {
    let mut client = BlockingChatter::connect("http://[::1]:50051")?;

    let uuid = UuidGenerator::new_v4().as_bytes().to_vec();

    let name = String::from("rust");
    let id = Some(Uuid { uuid });

    let user = Some(User { name, id });
    let text = String::from("Hello from a blocking gRPC client!");

    let msg = ChatMessage { user, text };

    client
        .send_message(msg)
        .expect("Could not send message to the chat system");

    let receiver = client
        .subscribe()
        .expect("Could not subscribe to chat system");

    while let Ok(msg) = receiver.recv() {
        println!("{}: {}", msg.user.unwrap().name, msg.text);
    }

    Ok(())
}
