use super::ErrorInformation;
use super::Result;

#[derive(FromForm)]
pub struct CheckAvailbaleArgs {
    username: String,
}

#[derive(Serialize)]
pub struct CheckAvailbaleResult {
    available: bool,
}

use super::guard::Authorization;
use super::Connection;
use rocket_contrib::Json;
use std::ops::Deref;
#[get("/checkUsername?<args>")]
pub fn check_username_handler(
    conn: Connection,
    args: CheckAvailbaleArgs,
) -> Json<Result<CheckAvailbaleResult>> {
    use logic::user_management::{check_username, find_user};

    if !check_username(&args.username) {
        return method_success![CheckAvailbaleResult { available: false }];
    }

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    match user {
        Some(_) => method_success![CheckAvailbaleResult { available: false }],
        None => method_success![CheckAvailbaleResult { available: true }],
    }
}

#[derive(Deserialize)]
pub struct RegisterUserArgs {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResult {
    user_id: String,
    session_token: String,
}

#[post("/register", data = "<args>")]
pub fn register_user_handler(
    conn: Connection,
    args: Json<RegisterUserArgs>,
) -> Json<Result<RegisterUserResult>> {
    use database::NewUser;
    use logic::security::{generate_salt, hash_password};
    use logic::user_management::{check_password, check_username, find_user};

    let args = args.0;

    if !check_username(&args.username) {
        return method_error![code = "INVALID_USERNAME", details = "Invalid username"];
    }

    if !check_password(&args.password) {
        return method_error![code = "INVALID_PASSWORD", details = "Password is too short"];
    }

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    if let Some(_) = user {
        return method_error![
            code = "USERNAME_TAKEN",
            details = "Username is already taken"
        ];
    }

    let salt = generate_salt();
    let hashed_password = hash_password(&args.password, &salt);

    use uuid::Uuid;
    let new_user = NewUser {
        id: &Uuid::new_v4(),
        username: &args.username.to_lowercase(),
        password: &hashed_password,
        salt: &salt,
    };

    use diesel::insert_into;
    use diesel::RunQueryDsl;
    use schema::users;
    let result = insert_into(users::table)
        .values(&new_user)
        .execute(conn.deref());

    match result {
        Ok(_) => {
            use logic::security::generate_session_token;
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

            method_success![RegisterUserResult {
                user_id: format!("{}", new_user.id),
                session_token: token
            }]
        }
        Err(_) => method_error![
            code = "INTERNAL_SERVER_ERROR",
            details = "Internal server error"
        ],
    }
}
