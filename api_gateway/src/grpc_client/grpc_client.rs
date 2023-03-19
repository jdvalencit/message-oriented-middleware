use protomom::crud_client::CrudClient;
use protomom::{CreateRequest, DeleteRequest, GetRequest, PutRequest, ReadRequest, UpdateRequest};

pub mod protomom {
    tonic::include_proto!("protomom");
}

pub async fn grpc_create(req_name: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(CreateRequest {
        name: req_name.into(),
    });

    let response = client
        .create_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}

pub async fn grpc_read(req_id: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(ReadRequest { id: req_id.into() });

    let response = client
        .read_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}

pub async fn grpc_update(req_id: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(UpdateRequest { id: req_id.into() });

    let response = client
        .update_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}

pub async fn grpc_delete(req_id: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(DeleteRequest { id: req_id.into() });

    let response = client
        .delete_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}

pub async fn grpc_put(req_id: String, cont: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(PutRequest {
        id: req_id.into(),
        content: cont.into(),
    });

    let response = client
        .put_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}

pub async fn grpc_get(req_id: String) {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(GetRequest { id: req_id.into() });

    let response = client
        .get_queue(request)
        .await
        .expect("Error receiving a response from server");
    println!("Response from server: {:?}", response)
}
