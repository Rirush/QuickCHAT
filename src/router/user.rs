use super::Result;
use super::ErrorInformation;

#[derive(FromForm)]
pub struct CheckAvailbaleArgs {
    username: String
}

#[derive(Serialize)]
pub struct CheckAvailbaleResult {
    available: bool
}

use super::Connection;
use std::ops::Deref;
use super::guard::Authorization;
use rocket_contrib::Json;
#[get("/checkAvailable?<args>")]
pub fn check_available_handler(conn: Connection, args: CheckAvailbaleArgs) -> Json<Result<CheckAvailbaleResult>> {
    use ::logic::user_management::find_user;
    use ::logic::user_management::check_username;
    
    if !check_username(&args.username) {
        return Json(Result {
            error: false,
            error_info: None,
            result: Some(CheckAvailbaleResult {
                available: false
            })
        })
    }

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    match user {
        Some(_) => Json(Result {
            error: false,
            error_info: None,
            result: Some(CheckAvailbaleResult {
                available: false
            })
        }),
        None => Json(Result {
            error: false,
            error_info: None,
            result: Some(CheckAvailbaleResult {
                available: true
            })
        })
    }
}

#[derive(Deserialize)]
pub struct AuthorizeUserArgs {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct AuthorizeUserResult {
    user_id: String,
    session_token: String
}

#[post("/authorize", data = "<args>")]
pub fn authorize_user_handler(conn: Connection, args: Json<AuthorizeUserArgs>) -> Json<Result<AuthorizeUserResult>> {
    use ::logic::user_management::find_user;
    use ::logic::security::hash_password;

    let args = args.0;

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    match user {
        Some(u) => { 
            if hash_password(&args.password, &u.salt) == *u.password {
                // TODO: Generate session token and store it somewhere
                Json(Result {
                    error: false,
                    error_info: None,
                    result: Some(AuthorizeUserResult {
                        user_id: format!("{}", u.id),
                        session_token: "unimplemented".to_owned()
                    })
                })
            } else {
                Json(Result {
                    error: true,
                    error_info: Some(ErrorInformation {
                        description: "Incorrect password".to_owned(),
                        error_code: "INCORRECT_PASSWORD".to_owned()
                    }),
                    result: None
                })
            }
        },
        None => {
            Json(Result {
                error: true,
                error_info: Some(ErrorInformation {
                    description: "No such user found".to_owned(),
                    error_code: "INCORRECT_USERNAME".to_owned()
                }),
                result: None
            })
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterUserArgs {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct RegisterUserResult {
    user_id: String,
    session_token: String
}

#[post("/register", data = "<args>")]
pub fn register_user_handler(conn: Connection, args: Json<RegisterUserArgs>) -> Json<Result<RegisterUserResult>> {
    use ::logic::security::hash_password;
    use ::logic::security::generate_salt;
    use ::logic::user_management::find_user;
    use ::logic::user_management::check_username;
    use ::logic::user_management::check_password;
    use ::database::NewUser;

    let args = args.0;

    if !check_username(&args.username) {
        return Json(Result {
            error: true,
            error_info: Some(ErrorInformation {
                description: "Invalid username".to_owned(),
                error_code: "INVALID_USERNAME".to_owned()
            }),
            result: None
        })
    }

    if !check_password(&args.password) {
        return Json(Result {
            error: true,
            error_info: Some(ErrorInformation {
                description: "Password is too short".to_owned(),
                error_code: "INVALID_PASSWORD".to_owned()
            }),
            result: None
        })
    }

    let user = find_user(&args.username.to_lowercase(), &conn.deref());
    if let Some(_) = user {
        return Json(Result {
            error: true,
            error_info: Some(ErrorInformation {
                description: "Username already taken".to_owned(),
                error_code: "USERNAME_TAKEN".to_owned()
            }),
            result: None
        })
    }

    let salt = generate_salt();
    let hashed_password = hash_password(&args.password, &salt);

    use uuid::Uuid;
    let new_user = NewUser {
        id: &Uuid::new_v4(),
        username: &args.username.to_lowercase(),
        password: &hashed_password,
        salt: &salt
    };

    use ::schema::users;
    use diesel::insert_into;
    use diesel::RunQueryDsl;
    let result = insert_into(users::table)
        .values(&new_user)
        .execute(conn.deref());

    match result {
        Ok(_) => {
            // TODO: Generate session token and store it somewhere
            Json(Result {
                error: false,
                error_info: None,
                result: Some(RegisterUserResult {
                    user_id: format!("{}", new_user.id),
                    session_token: "unimplemented".to_owned()
                })
            })
        },
        Err(_) => {
            Json(Result {
                error: true,
                error_info: Some(ErrorInformation {
                    description: "Internal server error".to_owned(),
                    error_code: "INTERNAL_SERVER_ERROR".to_owned()
                }),
                result: None
            })
        }
    }

}
