use protomom::crud_client::CrudClient;
use protomom::CreateRequest;

pub mod protomom {
    tonic::include_proto!("protomom");
}

pub async fn grpc(req_name: String){
    let mut client = CrudClient::connect("http://[::1]:50051").await.expect("Error conecting client");
    let request = tonic::Request::new(CreateRequest {
        name: req_name.into(),
    });

    let response = client.create_queue(request).await.expect("Errir receiving a response from server");
    println!("Response from server: {:?}", response)
}