fn main() {
    tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .compile(&["../proto/chat/chat_service.proto"], &["../proto"])
    .expect("Could not compile proto files");
}
