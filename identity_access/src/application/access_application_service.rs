use actix::{Actor, Context, Handler};
use crate::domain::command::create_user_command::CreateUserCommand;
use crate::domain::model::user::User;

pub struct AccessApplicationService {}

impl Actor for AccessApplicationService {
    type Context = Context<Self>;
}

impl Handler<CreateUserCommand> for AccessApplicationService {
    type Result = User;

    fn handle(&mut self, msg: CreateUserCommand, ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
