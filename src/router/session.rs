use super::{Connection, ErrorInformation, Result};
use rocket_contrib::Json;

#[derive(Deserialize)]
pub struct AuthorizeUserArgs {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthorizeUserResult {
    user_id: String,
    session_token: String,
}

#[post("/", data = "<args>")]
pub fn create_session_handler(
    conn: Connection,
    args: Json<AuthorizeUserArgs>,
) -> Json<Result<AuthorizeUserResult>> {
    use logic::security::{generate_session_token, hash_password};
    use logic::user_management::find_user;
    use std::ops::Deref;

    let args = args.0;

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    match user {
        Some(u) => {
            if hash_password(&args.password, &u.salt) == *u.password {
                let token = generate_session_token();

                use redis::{Client, Commands};
                use std::env;
                let client = Client::open(env::var("REDIS_URL").unwrap().as_ref()).unwrap();
                let connection = client.get_connection().unwrap();

                match connection.hset::<_, _, _, i32>(
                    format!("session:{}:{}", &args.username, &token),
                    "valid",
                    "true",
                ) {
                    Err(error) => {
                        println!("{}", error);
                        panic!("failed to create session, this should never happen");
                    }
                    Ok(_) => {}
                }
                method_success![AuthorizeUserResult {
                    user_id: format!("{}", u.id),
                    session_token: token
                }]
            } else {
                method_error![code = "INCORRECT_PASSWORD", details = "Incorrect password"]
            }
        }
        None => method_error![code = "INCORRECT_USERNAME", details = "No such user found"],
    }
}
