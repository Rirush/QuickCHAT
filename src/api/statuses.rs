use rocket_contrib::Json;

#[derive(Deserialize)]
pub struct OnlineStatus {
    pub online: bool,
}

#[patch("/status/online", data = "<args>")]
pub fn update_online(args: Json<OnlineStatus>) -> Result<(), u16> {
    Err(501)
}

#[derive(FromForm)]
pub struct Timestamp {
    pub timestamp: u64,
}

#[get("/status/updates?<args>")]
pub fn get_updates(args: Timestamp) -> Result<(), u16> {
    Err(501)
}
