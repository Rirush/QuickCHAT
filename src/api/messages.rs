use rocket_contrib::Json;
use rocket_contrib::UUID;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PreparedMessage {
    pub recipient: Uuid,
    pub message: String,
    pub reply_to: Option<Uuid>,
}

#[post("/message", data = "<args>")]
pub fn send_message(args: Json<PreparedMessage>) -> Result<(), u16> {
    Err(501)
}

#[derive(FromForm)]
pub struct MessageQuery {
    pub user_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[get("/message?<args>")]
pub fn get_all_messages(args: MessageQuery) -> Result<(), u16> {
    Err(501)
}

#[get("/message/<id>")]
pub fn get_message(id: UUID) -> Result<(), u16> {
    Err(501)
}

#[derive(Deserialize)]
pub struct UpdatedMessage {
    pub message: String,
}

#[patch("/message/<id>", data = "<args>")]
pub fn update_message(id: UUID, args: Json<UpdatedMessage>) -> Result<(), u16> {
    Err(501)
}

#[delete("/message/<id>")]
pub fn delete_message(id: UUID) -> Result<(), u16> {
    Err(501)
}
