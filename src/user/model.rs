use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: User, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(user: &str, conn: &PgConnection) -> Vec<User> {
        all_users
            .filter(users::username.eq(user))
            .load::<User>(conn)
            .expect("error!")
    }
}
