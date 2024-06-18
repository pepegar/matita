use server::{matita::graph_store_server::GraphStoreServer, MatitaServer};
use tonic::transport::Server;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MatitaServer::default();

    Server::builder()
        .add_service(GraphStoreServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
