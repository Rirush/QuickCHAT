use schema::*;

use uuid::Uuid;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Message {
    pub id: Uuid,
    pub sender: Uuid, 
    pub recipients: Vec<Uuid>,
    pub message: Option<String>,
    pub contents_type: String,
    pub contents: Option<Vec<Uuid>>,
    pub date_sent: SystemTime,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub id: &'a Uuid,
    pub sender: &'a Uuid,
    pub recipients: &'a Vec<Uuid>,
    pub message: Option<&'a String>,
    pub contents_type: &'a String,
    pub contents: Option<&'a Vec<Uuid>>,
    pub date_sent: &'a SystemTime,
    pub deleted: bool,
}

#[derive(Queryable)]
pub struct MessageType {
    pub type_name: String,
}

#[derive(Queryable)]
pub struct Content {
    pub id: Uuid,
    pub content: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "contents"]
pub struct NewContent<'a> {
    pub id: &'a Uuid,
    pub content: &'a Vec<u8>,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub username: &'a String,
    pub password: &'a String,
    pub salt: &'a String,
}
