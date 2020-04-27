# moosage - 🐮

A cow-themed chat system, implemented in Rust using gRPC.

Mostly meant to learn more about [`tonic`](https://crates.io/crates/tonic), [`tokio`](https://crates.io/crates/tokio) and [`tracing`](https://crates.io/crates/tracing).

# crates

### [moosage-common](./moosage-common)

Contains the gRPC proto files, as well being responsible for generating rust code for the gRPC services/messages.

### [moosage](./moosage)

The server implementation of `moosage`.

Clients can subscribe to a stream of messages.

Clients can send messages, which are then broadcast to every client which subscribed to the stream of messages.

### [moosage-client](./moosage-client)

The client implementation of `moosage`.
