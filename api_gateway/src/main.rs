#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to my mom"
}

#[get("/create_queue/<name>")]
fn create_queue(name: String) -> String {
    format!("Queue: {} was created.",name)
}

#[get("/read_queue/<id_queue>")]
fn read_queue(id_queue: String) -> String {
    format!("Queue: {} was returned.",id_queue)
}

#[get("/update_queue/<id_queue>")]
fn update_queue(id_queue: String) -> String {
    format!("Queue: {} was updated.",id_queue)
}

#[get("/delete_queue/<id_queue>")]
fn delete_queue(id_queue: String) -> String {
    format!("Queue: {} was deleted.",id_queue)
}


#[get("/write/<id_queue>/<msg>")]
fn write(id_queue: String,msg: String) -> String {
    format!("Message {} was added to queue {}.",msg,id_queue)
}   

#[get("/read/<id_queue>")]
fn read(id_queue: String) -> String {
    format!("Queue {} was read.",id_queue)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/crud", routes![create_queue,read_queue,update_queue,delete_queue])
        .mount("/queue", routes![write,read])
}