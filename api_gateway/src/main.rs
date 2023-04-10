use dotenv::dotenv;
use rocket_basicauth::BasicAuth;

#[macro_use]
extern crate rocket;
use api_gateway::grpc_client::grpc_client::{
    grpc_create, grpc_delete, grpc_get, grpc_put, grpc_read, grpc_create_user
};

#[get("/")]
fn index() -> &'static str {
    "Welcome to my mom"
}

#[post("/register-user")]
async fn register_user(auth: BasicAuth) -> String {
    let user = auth.username.to_string();
    let pass = auth.password.to_string();
    grpc_create_user(user, pass).await
}

#[post("/create-queue/<id_queue>")]
async fn create_queue(id_queue: String, auth: BasicAuth) -> String {
    let user = auth.username.to_string();
    let pass = auth.password.to_string();
    grpc_create(id_queue, user, pass).await
}

#[get("/read-queue/<id_queue>")]
async fn read_queue(id_queue: String) -> String {
    grpc_read(id_queue.clone()).await
}

#[delete("/delete-queue/<id_queue>")]
async fn delete_queue(id_queue: String, auth: BasicAuth) -> String {
    let user = auth.username.to_string();
    let pass = auth.password.to_string();
    grpc_delete(id_queue.clone(), user, pass).await
}

#[put("/put/<id_queue>/<content>")]
async fn put(id_queue: String, content: String) -> String {
    grpc_put(id_queue.clone(), content.clone()).await
}

#[get("/get-queues")]
async fn get() -> String {
    grpc_get().await
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index, get, register_user])
        .mount("/crud", routes![create_queue, read_queue, delete_queue])
        .mount("/queue", routes![put])
}
