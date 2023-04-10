use futures::future::join_all;
use lazy_static::lazy_static;
use protomom::{
    crud_client::CrudClient, user_client::UserClient, CreateRequest, CreateUserRequest,
    DeleteRequest, GetRequest, PutRequest, ReadRequest,
};
use std::{collections::HashMap, env, iter::Cycle, sync::RwLock, vec::IntoIter};

pub mod protomom {
    tonic::include_proto!("protomom");
}

lazy_static! {
    static ref IP_VECTOR: Vec<String> = env::var("SERVER_IPS")
        .unwrap_or_else(|_| "http://[::1]:50051".to_string())
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    static ref ROUND_ROBIN_IPS: RwLock<Cycle<IntoIter<String>>> =
        RwLock::new(IP_VECTOR.clone().into_iter().cycle());
    static ref QUEUE_MAPPING: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub async fn grpc_create(req_name: String, req_user: String, req_password: String) -> String {
    if QUEUE_MAPPING
        .write()
        .expect("Error accesing queue mapping")
        .contains_key(&req_name)
    {
        return format!("Error queue: {} was already created.", req_name);
    }

    let server_ip = ROUND_ROBIN_IPS
        .write()
        .expect("Error accesing server ips")
        .next()
        .expect("Error getting server ip");

    let mut client = CrudClient::connect(server_ip.clone())
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(CreateRequest {
        id: req_name.clone(),
        user: req_user.clone(),
        password: req_password.clone(),
    });

    //Returns response from server
    let response = client
        .create_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner();
    if response.status {
        QUEUE_MAPPING
            .write()
            .expect("Error accesing queue mapping")
            .insert(req_name, server_ip);
    }
    response.message
}

pub async fn grpc_read(req_id: String) -> String {
    let mom_ip = QUEUE_MAPPING
        .read()
        .expect("Error accesing server ips")
        .get(&req_id)
        .unwrap_or(&"http://[::1]:50051".to_string())
        .clone();
    let mut client = CrudClient::connect(mom_ip)
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(ReadRequest { id: req_id });

    //Returns response from server
    client
        .read_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}

pub async fn grpc_delete(req_id: String, req_user: String, req_password: String) -> String {
    let mom_ip = QUEUE_MAPPING
        .read()
        .expect("Error accesing server ips")
        .get(&req_id)
        .unwrap_or(&"http://[::1]:50051".to_string())
        .clone();
    let mut client = CrudClient::connect(mom_ip)
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(DeleteRequest {
        id: req_id.clone(),
        user: req_user.clone(),
        password: req_password.clone(),
    });

    //Returns response from server
    let response = client
        .delete_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner();

    if response.status {
        QUEUE_MAPPING
            .write()
            .expect("Error accesing queue mapping")
            .remove(&req_id);
    }
    response.message
}

pub async fn grpc_put(req_id: String, cont: String) -> String {
    let mom_ip = QUEUE_MAPPING
        .read()
        .expect("Error accesing server ips")
        .get(&req_id)
        .unwrap_or(&"http://[::1]:50051".to_string())
        .clone();
    
    let mut client = CrudClient::connect(mom_ip)
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(PutRequest {
        id: req_id,
        content: cont,
    });

    //Returns response from server
    client
        .put_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}

pub async fn grpc_get() -> String {
    let futures = IP_VECTOR.iter().map(|s| get(s.clone()));
    let results = join_all(futures).await;
    results.into_iter().fold(String::new(), |acc, s| acc + &s)
}

async fn get(mom_ip: String) -> String {
    match CrudClient::connect(mom_ip.clone()).await {
        Ok(mut client) => {
            let request = tonic::Request::new(GetRequest {});
            // Returns response from server
            return client
                .get_queues(request)
                .await
                .expect("Error receiving a response from server")
                .into_inner()
                .message;
        }
        Err(_) => {
            let empty_string = "";

            // Returns empty response
            return empty_string.to_string();
        }
    }
}

pub async fn grpc_create_user(req_user: String, req_password: String) -> String {
    let mut client = UserClient::connect("http://[::1]:50051".to_string())
        .await
        .expect("Error connecting client");
    let request = tonic::Request::new(CreateUserRequest {
        user: req_user.clone(),
        password: req_password.clone(),
    });

    // Returns response from server
    client
        .create_user(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}
