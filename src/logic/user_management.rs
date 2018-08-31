use ::schema::users;
use ::database::User;
use diesel::pg::PgConnection;

pub fn find_user(username: &String, conn: &PgConnection) -> Option<User> {
    use diesel::{ExpressionMethods,QueryDsl,RunQueryDsl};

    let result = users::table.filter(users::username.eq(username))
        .load::<User>(conn);
    match result {
        Ok(mut user) => {
            if user.len() == 1 {
                Some(user.remove(0))
            } else {
                None
            }
        },
        Err(_) => {
            None
        }
    }
}

pub fn check_username(username: &String) -> bool {
    use regex::Regex;
    
    lazy_static! {
        static ref EXPR: Regex = Regex::new(r"^[a-zA-Z0-9]{3,32}$").unwrap();
    }
    EXPR.is_match(username)
}

pub fn check_password(password: &String) -> bool {
    password.len() >= 8
}
