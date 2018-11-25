use database::{NewMessage, User, Message};
use uuid::Uuid;
use diesel::pg::PgConnection;
use std::time::SystemTime;


// It doesn't work becasue of lifetimes. Dunno how to fix it yet.
/*
pub fn create_message<'a>(sender: &'a User, recipients: Vec<&'a User>, message: &'a String) -> NewMessage<'a> {
    NewMessage {
        id: &Uuid::new_v4(),
        sender: &sender.id,
        recipients: &recipients.into_iter().map(|u| u.id).collect(),
        message: Some(message),
        contents_type: &"text".to_owned(),
        contents: None,
        date_sent: &SystemTime::now(),
        deleted: false,
    }
}
*/

use schema::messages;

pub fn send_message(msg: &NewMessage, conn: &PgConnection) -> bool {
    use diesel::{insert_into, RunQueryDsl};

    let result = insert_into(messages::table)
        .values(msg)
        .execute(conn);

    match result {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn find_message(id: &Uuid, conn: &PgConnection) -> Option<Message> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    let result = messages::table.filter(messages::id.eq(id))
        .load::<Message>(conn);
    match result {
        Ok(mut msg) => {
            if msg.len() == 1 {
                Some(msg.remove(0))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}