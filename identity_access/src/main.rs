use actix::prelude::*;

use crate::application::access_application_service::AccessApplicationService;
use crate::domain::command::create_user_command::CreateUserCommand;

mod application;
mod domain;

fn main() -> std::io::Result<()> {
    let system = System::new("test");

    // start new actor
    let addr = AccessApplicationService {}.start();

    let create_user_command = CreateUserCommand {
        id: 0,
        uuid: Default::default(),
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };
    // send message and get future for result
    let res = addr.send(create_user_command);

    Arbiter::spawn(
        res.map(|res| {
            println!("RESULT: {}", res == 20);
        })
            .map_err(|_| ()));

    system.run()
}
