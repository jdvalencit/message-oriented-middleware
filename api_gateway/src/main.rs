use dotenv::dotenv;
use rocket_basicauth::BasicAuth;

#[macro_use]
extern crate rocket;
use api_gateway::grpc_client::grpc_client::{
    grpc_create, grpc_delete, grpc_get, grpc_put, grpc_read,
};

#[get("/")]
fn index() -> &'static str {
    "Welcome to my mom"
}

#[post("/register-user")]
async fn register_user(auth: BasicAuth) -> String {
    println!("{:?}", auth.username);
    println!("{:?}", auth.password);
    "Fake".to_string()
}


#[post("/create-queue/<name>")]
async fn create_queue(name: String, auth: BasicAuth) -> String {
    grpc_create(name).await
}

#[get("/read-queue/<id_queue>")]
async fn read_queue(id_queue: String) -> String {
    grpc_read(id_queue.clone()).await
}

#[delete("/delete-queue/<id_queue>")]
async fn delete_queue(id_queue: String, auth: BasicAuth) -> String {
    grpc_delete(id_queue.clone()).await
}

#[put("/put/<id_queue>/<content>")]
async fn put(id_queue: String, content: String) -> String {
    grpc_put(id_queue.clone(), content.clone()).await
}

#[get("/get-queues")]
async fn get() -> String {
    grpc_get("fake".to_string()).await
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index, get, register_user])
        .mount("/crud", routes![create_queue, read_queue, delete_queue])
        .mount("/queue", routes![put])
}
