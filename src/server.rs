use tonic::{Request, Response, Status};

use matita::graph_store_server::GraphStore;
use matita::WriteRequest;

use self::matita::WriteResponse;

pub mod matita {
    tonic::include_proto!("matita");
}

#[derive(Debug, Default)]
pub struct MatitaServer {}

#[tonic::async_trait]
impl GraphStore for MatitaServer {
    async fn write_triples<'a>(
        &'a self,
        _request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        todo!()
    }
}
