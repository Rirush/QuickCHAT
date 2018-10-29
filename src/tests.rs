use construct_rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::Value;

#[test]
fn not_found_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");
    let mut response = client.get("/404").dispatch();

    assert_eq!(response.status(), Status::NotFound);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], true);
    assert_eq!(v["error_info"]["error_code"], "NOT_FOUND");
    assert_eq!(v["result"], Value::Null);
}

#[test]
fn username_available_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/user/checkUsername?username=username")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], false);
    assert_eq!(v["error_info"], Value::Null);
    assert_eq!(v["result"]["available"], true);
}

#[test]
fn invalid_username_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");
    let mut response = client.get("/user/checkUsername?username=____").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], false);
    assert_eq!(v["error_info"], Value::Null);
    assert_eq!(v["result"]["available"], false);
}

#[test]
fn short_username_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");
    let mut response = client.get("/user/checkUsername?username=oo").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], false);
    assert_eq!(v["error_info"], Value::Null);
    assert_eq!(v["result"]["available"], false);
}

#[test]
fn successful_registration_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");

    let register_data = json!({
        "username": "testuser",
        "password":" validpassword"
    });

    let mut response = client
        .post("/user/register")
        .body(register_data.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], false);
    assert_eq!(v["error_info"], Value::Null);
    assert_ne!(v["result"]["user_id"], Value::Null);
    assert_ne!(v["result"]["session_token"], Value::Null);

    delete_user("testuser");
}

#[test]
fn authorization_test() {
    dotenv::dotenv().ok();
    let client = Client::new(construct_rocket()).expect("valid rocket instance");

    create_user("authtestuser".to_string(), "validpassword".to_string());
    let auth_data = json!({
        "username": "authtestuser",
        "password": "validpassword"
    });

    let mut response = client
        .post("/session")
        .body(auth_data.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], false);
    assert_eq!(v["error_info"], Value::Null);
    assert_ne!(v["result"]["user_id"], Value::Null);
    assert_ne!(v["result"]["session_token"], Value::Null);

    let incorrect_auth_data = json!({
        "username": "authtestuser",
        "password": "invalidpassword"
    });

    let mut response = client
        .post("/session")
        .body(incorrect_auth_data.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], true);
    assert_eq!(v["error_info"]["error_code"], "INCORRECT_PASSWORD");
    assert_eq!(v["result"], Value::Null);

    delete_user("authtestuser");

    let mut response = client
        .post("/session")
        .body(incorrect_auth_data.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let v: Value = serde_json::from_str(&response.body_string().expect("request body"))
        .expect("valid json body");

    assert_eq!(v["error"], true);
    assert_eq!(v["error_info"]["error_code"], "INCORRECT_USERNAME");
    assert_eq!(v["result"], Value::Null);
}

fn delete_user(u: &str) {
    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
    use schema::users::{self, columns::username};
    use std::env;

    let connection =
        PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("pg connection");
    diesel::delete(users::table.filter(username.eq(u)))
        .execute(&connection)
        .unwrap();
}

fn create_user(u: String, p: String) {
    use database::NewUser;
    use diesel::{Connection, PgConnection, RunQueryDsl};
    use logic::security::{generate_salt, hash_password};
    use schema::users;
    use std::env;

    let salt = generate_salt();
    let hashed_password = hash_password(&p, &salt);

    use uuid::Uuid;
    let new_user = NewUser {
        id: &Uuid::new_v4(),
        username: &u,
        password: &hashed_password,
        salt: &salt,
    };

    let connection =
        PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("pg connection");

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)
        .unwrap();
}
