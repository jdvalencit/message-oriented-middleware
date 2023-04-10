use protomom::{
    crud_server::{Crud, CrudServer}, CreateReply, CreateRequest, DeleteReply, DeleteRequest, GetReply, GetRequest, PutReply,
    user_server::{User, UserServer}, CreateUserReply, CreateUserRequest,
    PutRequest, ReadReply, ReadRequest,
};
use std::{
    collections::HashMap, sync::RwLock, 
};
use tonic::{
    transport::Server, Request, Response, Status,
};
use lazy_static::lazy_static;
use uuid::Uuid;
use dotenv::dotenv;

mod crud;
use crud::user::{insert_user, get_user};

mod queue;
use queue::Queue;

pub mod protomom {
    tonic::include_proto!("protomom");
}

#[derive(Debug, Default)]

pub struct CrudServicer {}

lazy_static! {
    static ref LOCAL_QUEUES: RwLock<HashMap<String, Queue>> = RwLock::new(HashMap::new());
}

#[tonic::async_trait]
impl Crud for CrudServicer {
    async fn create_queue(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateReply>, Status> {
        println!("Request from client: {:?}", request);

        let id = request.into_inner().id;
        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if local_queues_ref.contains_key(&id) {
            let reply = protomom::CreateReply {
                message: format!("Error queue: {} was already created.", id),
                status: false,
            };
            return Ok(Response::new(reply));
        }
        local_queues_ref.insert(
            id.clone(),
            Queue::new("user1".to_string(), "key1".to_string()),
        );

        let reply = protomom::CreateReply {
            message: format!("Queue: {} was created.", id),
            status: true,
        };

        Ok(Response::new(reply))
    }

    async fn read_queue(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadReply>, Status> {
        let users = get_user("Tomas".to_string(), "password".to_string()).await.unwrap();
        println!("current users: {:?}", users);

        println!("Request from client: {:?}", request);
        let id = request.into_inner().id;

        let local_queues_ref = LOCAL_QUEUES.read().expect("Error accesing local queues");

        if let Some(queue) = local_queues_ref.get(&id) {
            let reply = protomom::ReadReply {
                message: format!("Queue: {} contains: \n{:?}.", id, queue),
            };

            return Ok(Response::new(reply));
        }

        let reply = protomom::ReadReply {
            message: format!("Queue: {} not found.", id),
        };

        Ok(Response::new(reply))
    }

    async fn delete_queue(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteReply>, Status> {
        println!("Request from client: {:?}", request);
        let users = get_user("Tomas".to_string(), "password".to_string()).await.unwrap();
        println!("current users: {:?}", users);

        let id = request.into_inner().id;
        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if let Some(_queue) = local_queues_ref.remove(&id) {
            let reply = protomom::DeleteReply {
                message: format!("Queue: {} was deleted.", id),
                status: true,
            };

            return Ok(Response::new(reply));
        }

        let reply = protomom::DeleteReply {
            message: format!("Queue: {} not found.", id),
            status: false,
        };

        Ok(Response::new(reply))
    }

    async fn put_queue(&self, request: Request<PutRequest>) -> Result<Response<PutReply>, Status> {
        println!("Request from client: {:?}", request);
        let req = request.into_inner();

        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if let Some(queue) = local_queues_ref.get_mut(&req.id) {
            queue.queue_data.push(req.content.clone());

            let reply = protomom::PutReply {
                message: format!("Message {} was added to queue {}.", req.content, req.id),
            };
            return Ok(Response::new(reply));
        }

        let reply = protomom::PutReply {
            message: format!("Queue: {} not found.", req.id),
        };

        Ok(Response::new(reply))
    }

    async fn get_queues(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        println!("Request from client: {:?}", request);

        let local_queues_ref = LOCAL_QUEUES.read().expect("Error accesing local queues");
        let queues = local_queues_ref.values();
        let reply = protomom::GetReply {
            message: format!("{:?}", queues),
        };

        Ok(Response::new(reply))
    }
}

pub struct UserServicer {}

#[tonic::async_trait]
impl User for UserServicer {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserReply>, Status> {
        println!("Request from client: {:?}", request);
        
        // let id = Uuid::new_v4();
        // let users = insert_user(id, "Tomas".to_string(), "password".to_string()).await?;
        let response = "Sucess";

        let reply = protomom::CreateUserReply  {
            message: format!("{:?}", response),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "[::1]:50051".parse()?;
    let crud_servicer = CrudServicer::default();


    Server::builder()
        .add_service(CrudServer::new(crud_servicer))
        .serve(addr)
        .await?;

    Ok(())
}
