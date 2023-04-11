use dotenv::dotenv;
use lazy_static::lazy_static;
use protomom::{
    crud_server::{Crud, CrudServer},
    user_server::{User, UserServer},
    CreateReply, CreateRequest, CreateUserReply, CreateUserRequest, DeleteReply, DeleteRequest,
    GetReply, GetRequest, PutReply, PutRequest, ReadReply, ReadRequest,
};
use std::{collections::HashMap, sync::RwLock};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

mod crud;
use crud::user::{get_user, insert_user};

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

        let req = request.into_inner();

        match get_user(req.user.clone(), req.password).await {
            Ok(users) => {
                if users.is_empty(){
                    let reply = protomom::CreateReply {
                        message: format!("Error validating user {}", req.user),
                        status: false,
                    };
                    return Ok(Response::new(reply));
                }
                println!("Current user: {:?}",users);
            }
            Err(_) => {
                let reply = protomom::CreateReply {
                    message: format!("Error validating user {}", req.user),
                    status: false,
                };
                return Ok(Response::new(reply));

            }
        }

        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if local_queues_ref.contains_key(&req.id) {
            let reply = protomom::CreateReply {
                message: format!("Error queue: {} was already created.", req.id),
                status: false,
            };
            return Ok(Response::new(reply));
        }
        local_queues_ref.insert(
            req.id.clone(),
            Queue::new(req.user, req.id.clone()),
        );

        let reply = protomom::CreateReply {
            message: format!("Queue: {} was created.", req.id),
            status: true,
        };

        Ok(Response::new(reply))
    }

    async fn read_queue(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadReply>, Status> {
        println!("Request from client: {:?}", request);
        let id = request.into_inner().id;

        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if let Some(queue) = local_queues_ref.get_mut(&id) {

            if queue.queue_data.is_empty(){
                let reply = protomom::ReadReply {
                    message: format!("Queue: {} is empty.", id),
                };

                return Ok(Response::new(reply))
            }

            let reply = protomom::ReadReply {
                message: format!("Queue: {} contains: \n{:?}.", id, queue.queue_data.pop().unwrap()),
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

        let req = request.into_inner();

        match get_user(req.user.clone(), req.password).await {
            Ok(users) => {
                if users.is_empty(){
                    let reply = protomom::DeleteReply {
                        message: format!("Error validating user {}", req.user),
                        status: false,
                    };
                    return Ok(Response::new(reply));
                }
                println!("Current user: {:?}",users);
            }
            Err(_) => {
                let reply = protomom::DeleteReply {
                    message: format!("Error validating user {}", req.user),
                    status: false,
                };
                return Ok(Response::new(reply));

            }
        }

        let mut local_queues_ref = LOCAL_QUEUES.write().expect("Error accesing local queues");

        if let Some(queue) = local_queues_ref.get(&req.id){
            if queue.user_id != req.user {
                let reply = protomom::DeleteReply {
                    message: format!("Queue: {} not available for user {}.", req.id, req.user),
                    status: true,
                };

                return Ok(Response::new(reply));
            }
            if let Some(_queue) = local_queues_ref.remove(&req.id) {
                let reply = protomom::DeleteReply {
                    message: format!("Queue: {} was deleted.", req.id),
                    status: true,
                };

                return Ok(Response::new(reply));
            }

        }

        let reply = protomom::DeleteReply {
            message: format!("Queue: {} not found.", req.id),
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

#[derive(Debug, Default)]
pub struct UserServicer {}

#[tonic::async_trait]
impl User for UserServicer {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Request from client: {:?}", request);
        
        let req = request.into_inner();

        let req_user = req.user.clone();
        let req_password = req.password.clone();

        let uid = Uuid::new_v4();

        match insert_user(uid, req_user, req_password).await {
            Ok(_user) => {
                let response = "Sucess";

                let reply = protomom::CreateUserReply {
                    message: format!("{:?}", response),
                    status: true,
                };

                return Ok(Response::new(reply));
            }
            Err(_) => {
                let response = "Internal error, user already exists";

                let reply = protomom::CreateUserReply {
                    message: format!("{:?}", response),
                    status: false,
                };

                return Ok(Response::new(reply));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "[::1]:50051".parse()?;
    let crud_servicer = CrudServicer::default();
    let user_servicer = UserServicer::default();

    Server::builder()
        .add_service(CrudServer::new(crud_servicer))
        .add_service(UserServer::new(user_servicer))
        .serve(addr)
        .await?;

    Ok(())
}
