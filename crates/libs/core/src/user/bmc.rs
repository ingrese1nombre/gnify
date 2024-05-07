use gnify::{model::Record, source::BMC};
use ulid::Ulid;

use super::{view::DetailedUserView, User};

mod postgres; 

#[derive(Default)]
pub struct GetUser {
    pub id: Option<Ulid>,
    pub username: Option<String>,   
    pub email: Option<String>,   
}

impl GetUser {
    pub fn by_username(value: &str) -> Self {
        Self { username: Some(value.to_string()), ..Default::default() }
    } 
}

impl BMC for GetUser {
    type Output = Option<DetailedUserView>;
}


pub struct WriteUser {
    pub record: Record<User>
}