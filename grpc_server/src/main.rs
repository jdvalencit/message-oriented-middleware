use tonic::{transport::Server};
use protomom::crud_server::CrudServer;

use protomom::crud_server::Crud;
use protomom::{CreateReply, CreateRequest};
use tonic::{Request, Response, Status};

pub mod protomom {
    tonic::include_proto!("protomom");
}

#[derive(Debug, Default)]
pub struct CreateQueue {}

#[tonic::async_trait]
impl Crud for CreateQueue {
    async fn create_queue(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateReply>, Status> {
        println!("Request from client: {:?}", request);

        let reply = protomom::CreateReply {
            message: format!("Queue: {} was created.", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    let create_queue = CreateQueue::default();

    Server::builder()
        .add_service(CrudServer::new(create_queue))
        .serve(addr)
        .await?;

    Ok(())
}