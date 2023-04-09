use futures::future::join_all;
use lazy_static::lazy_static;
use protomom::crud_client::CrudClient;
use protomom::{CreateRequest, DeleteRequest, GetRequest, PutRequest, ReadRequest};
use std::collections::HashMap;
use std::iter::Cycle;
use std::sync::RwLock;
use std::vec::IntoIter;

pub mod protomom {
    tonic::include_proto!("protomom");
}

lazy_static! {
    static ref IP_VECTOR: Vec<String> = vec![
        String::from("http://[::1]:50051"),
        String::from("http://[::1]:50052")
    ];
    static ref ROUND_ROBIN_IPS: RwLock<Cycle<IntoIter<String>>> =
        RwLock::new(IP_VECTOR.clone().into_iter().cycle());
    static ref QUEUE_MAPPING: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub async fn grpc_create(req_name: String) -> String {
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
        name: req_name.clone(),
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
        .expect("Error getting server ip")
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

pub async fn grpc_delete(req_id: String) -> String {
    let mom_ip = QUEUE_MAPPING
        .read()
        .expect("Error accesing server ips")
        .get(&req_id)
        .expect("Error getting server ip")
        .clone();
    let mut client = CrudClient::connect(mom_ip)
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(DeleteRequest { id: req_id.clone() });

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
        .expect("Error getting server ip")
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

pub async fn grpc_get(req_id: String) -> String {
    let futures = IP_VECTOR.iter().map(|s| get(req_id.clone(), s.clone()));
    let results = join_all(futures).await;
    results.into_iter().fold(String::new(), |acc, s| acc + &s)
}

async fn get(req_id: String, mom_ip: String) -> String {
    let mut client = CrudClient::connect(mom_ip.clone())
        .await
        .expect("Error connecting client");
    let request = tonic::Request::new(GetRequest { id: req_id });

    // Returns response from server
    client
        .get_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}
