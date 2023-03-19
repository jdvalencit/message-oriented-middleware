use protomom::crud_server::{Crud, CrudServer};
use protomom::{
    CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetReply, GetRequest, PutReply,
    PutRequest, ReadReply, ReadRequest, UpdateReply, UpdateRequest,
};
use tonic::transport::Server;
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

    async fn read_queue(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadReply>, Status> {
        println!("Request from client: {:?}", request);

        let reply = protomom::ReadReply {
            message: format!("Queue: {} was read.", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn update_queue(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateReply>, Status> {
        println!("Request from client: {:?}", request);

        let reply = protomom::UpdateReply {
            message: format!("Queue: {} was updated.", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn delete_queue(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteReply>, Status> {
        println!("Request from client: {:?}", request);

        let reply = protomom::DeleteReply {
            message: format!("Queue: {} was deleted.", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn put_queue(&self, request: Request<PutRequest>) -> Result<Response<PutReply>, Status> {
        println!("Request from client: {:?}", request);
        let req = request.into_inner();

        let reply = protomom::PutReply {
            message: format!("Message {} was added to queue {}.", req.content, req.id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn get_queue(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        println!("Request from client: {:?}", request);

        let reply = protomom::GetReply {
            message: format!("Queue: {} was returned.", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let create_queue = CreateQueue::default();

    Server::builder()
        .add_service(CrudServer::new(create_queue))
        .serve(addr)
        .await?;

    Ok(())
}
