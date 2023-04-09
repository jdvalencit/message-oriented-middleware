use protomom::crud_client::CrudClient;
use protomom::{
    CreateRequest, DeleteRequest, GetRequest, PutRequest, ReadRequest
};

pub mod protomom {
    tonic::include_proto!("protomom");
}

pub async fn grpc_create(req_name: String) -> String {
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(CreateRequest {
        name: req_name,
    });

    //Returns response from server
    client
        .create_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}

pub async fn grpc_read(req_id: String) -> String {
    let mut client = CrudClient::connect("http://[::1]:50051")
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
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(DeleteRequest { id: req_id });

    //Returns response from server
    client
        .delete_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}

pub async fn grpc_put(req_id: String, cont: String) -> String {
    let mut client = CrudClient::connect("http://[::1]:50051")
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
    let mut client = CrudClient::connect("http://[::1]:50051")
        .await
        .expect("Error conecting client");
    let request = tonic::Request::new(GetRequest { id: req_id });

    //Returns response from server
    client
        .get_queue(request)
        .await
        .expect("Error receiving a response from server")
        .into_inner()
        .message
}
