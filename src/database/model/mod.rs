use ::schema::*;

use uuid::Uuid;

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: String,
}
