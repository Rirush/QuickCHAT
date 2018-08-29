use super::Connection;

#[get("/")]
pub fn index_handler(conn: Connection) -> &'static str {
    "unimplemented"
}
