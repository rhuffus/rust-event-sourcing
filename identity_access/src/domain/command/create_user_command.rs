use uuid::Uuid;
use actix::Message;
use common::domain::command_response::CommandResponse;

pub struct CreateUserCommand {
    id: u32,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl Message for CreateUserCommand {
    type Result = CommandResponse;
}
