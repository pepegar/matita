use anyhow::Result;
use dashmap::DashMap;
use tonic::{Request, Response, Status};

use matita::graph_store_server::GraphStore;
use matita::WriteRequest;

use self::matita::{GetTriplesRequest, GetTriplesResponse, Triple, WriteResponse};

pub mod matita {
    tonic::include_proto!("matita");
}

#[derive(Debug)]
pub struct MatitaServer {
    cache: DashMap<String, String>,
}

impl MatitaServer {
    fn write_triple(&self, triple: Triple) -> Result<()> {
        let key = "sp:".to_string() + triple.subject.as_str();

        self.cache.insert(key, "".to_string());
        Ok(())
    }

    pub(crate) fn new(cache: DashMap<String, String>) -> MatitaServer {
        MatitaServer { cache }
    }
}

#[tonic::async_trait]
impl GraphStore for MatitaServer {
    async fn write_triples(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        let _ = request.into_inner().triples.iter().for_each(|triple| {
            let _ = self.write_triple(triple.clone()).unwrap();
        });
        let reply = WriteResponse { success: true };
        Ok(Response::new(reply))
    }

    async fn get_triples(
        &self,
        _request: Request<GetTriplesRequest>,
    ) -> Result<Response<GetTriplesResponse>, Status> {
        Ok(Response::new(GetTriplesResponse {
            triples: self
                .cache
                .iter()
                .map(|_x| Triple {
                    subject: "".to_string(),
                    predicate: "".to_string(),
                    object: "".to_string(),
                })
                .collect::<Vec<Triple>>(),
        }))
    }
}
