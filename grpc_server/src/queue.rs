#[derive(Debug)]
pub struct Queue {
    pub user_id: String,
    pub key: String,
    pub queue_data: Vec<String>,
}

impl Queue {
    pub fn new(uid: String, key: String) -> Queue {
        Queue {
            user_id: (uid),
            key: (key),
            queue_data: (Vec::new()),
        }
    }
}
