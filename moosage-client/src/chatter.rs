use moosage_common::chat::{chat_service_client::ChatServiceClient, ChatMessage, Empty};
use std::sync::mpsc::{channel, Receiver};
use tokio::runtime::{Builder, Runtime};

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

// The order of the fields in this struct is important. They must be ordered
// such that when `BlockingClient` is dropped the client is dropped
// before the runtime. Not doing this will result in a deadlock when dropped.
// Rust drops struct fields in declaration order.
pub(crate) struct BlockingChatter {
    client: ChatServiceClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl BlockingChatter {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut rt = Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        let client = rt.block_on(ChatServiceClient::connect(dst))?;

        Ok(Self { rt, client })
    }

    pub fn send_message(
        &mut self,
        request: impl tonic::IntoRequest<ChatMessage>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        self.rt.block_on(self.client.send_message(request))
    }

    pub fn subscribe(&mut self) -> Result<Receiver<ChatMessage>> {
        let mut stream = self
            .rt
            .block_on(self.client.subscribe(Empty {}))?
            .into_inner();
        let (sender, receiver) = channel();
        self.rt.spawn(async move {
            while let Some(msg) = stream
                .message()
                .await
                .expect("Could not fetch next message")
            {
                if sender.send(msg).is_err() {
                    // The receiver has closed, so we can close the stream
                    break;
                }
            }
        });
        Ok(receiver)
    }
}
