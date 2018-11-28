use rocket_contrib::Json;
use rocket_contrib::UUID;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[post("/user", data = "<args>")]
pub fn create_user(args: Json<UserCredentials>) -> Result<(), u16> {
    Err(501)
}

#[get("/user")]
pub fn get_current_user() -> Result<(), u16> {
    Err(501)
}

#[derive(Deserialize)]
pub struct PatchCredentials {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<Uuid>,
}

#[patch("/user", data = "<args>")]
pub fn update_current_user(args: Json<PatchCredentials>) -> Result<(), u16> {
    Err(501)
}

#[delete("/user")]
pub fn delete_user() -> Result<(), u16> {
    Err(501)
}

#[get("/user/<id>")]
pub fn get_user(id: UUID) -> Result<(), u16> {
    Err(501)
}
