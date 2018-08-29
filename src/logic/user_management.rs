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
