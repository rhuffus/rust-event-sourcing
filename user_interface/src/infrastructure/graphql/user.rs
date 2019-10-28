use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(juniper::GraphQLObject)]
pub struct User {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

pub enum Event {
    AccountOpened { initial_balance: Money },
    MoneyDeposited {
        transfer_id: Uuid,
        amount: Money,
        balance: Money,
    },
    MoneyWithdrawn {
        transfer_id: Uuid,
        amount: Money,
        balance: Money,
    },
    AccountOverdrawn { balance: Money },
    AccountClosed,
}
