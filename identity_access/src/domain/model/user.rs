use uuid::Uuid;

pub struct User {
    id: u32,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

