#[macro_use]
extern crate rocket;
use api_gateway::grpc_client::grpc_client::{
    grpc_create, grpc_delete, grpc_get, grpc_put, grpc_read, grpc_update,
};

#[get("/")]
fn index() -> &'static str {
    "Welcome to my mom"
}

#[get("/create_queue/<name>")]
async fn create_queue(name: String) -> String {
    grpc_create(name.clone()).await;
    format!("Queue: {} was created.", name)
}

#[get("/read_queue/<id_queue>")]
async fn read_queue(id_queue: String) -> String {
    grpc_read(id_queue.clone()).await;
    format!("Queue: {} was read.", id_queue)
}

#[get("/update_queue/<id_queue>")]
async fn update_queue(id_queue: String) -> String {
    grpc_update(id_queue.clone()).await;
    format!("Queue: {} was updated.", id_queue)
}

#[get("/delete_queue/<id_queue>")]
async fn delete_queue(id_queue: String) -> String {
    grpc_delete(id_queue.clone()).await;
    format!("Queue: {} was deleted.", id_queue)
}

#[get("/put/<id_queue>/<content>")]
async fn put(id_queue: String, content: String) -> String {
    grpc_put(id_queue.clone(), content.clone()).await;
    format!("Message {} was added to queue {}.", content, id_queue)
}

#[get("/get/<id_queue>")]
async fn get(id_queue: String) -> String {
    grpc_get(id_queue.clone()).await;
    format!("Queue {} was returned.", id_queue)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/crud",
            routes![create_queue, read_queue, update_queue, delete_queue],
        )
        .mount("/queue", routes![put, get])
}
