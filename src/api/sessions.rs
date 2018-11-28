use rocket_contrib::Json;
use rocket_contrib::UUID;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SessionCredentials {
    pub username: String,
    pub password: String,
    pub platform: String,
    pub client: String,
}

#[post("/session", data = "<args>")]
pub fn create_session(args: Json<SessionCredentials>) -> Result<(), u16> {
    Err(501)
}

#[get("/session")]
pub fn get_all_sessions() -> Result<(), u16> {
    Err(501)
}

#[delete("/session")]
pub fn delete_current_session() -> Result<(), u16> {
    Err(501)
}

#[delete("/session/<id>")]
pub fn delete_session(id: UUID) -> Result<(), u16> {
    Err(501)
}
