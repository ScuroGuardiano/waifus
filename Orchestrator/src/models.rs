use diesel::prelude::*;
use diesel::sql_types::Timestamp;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::waifu_databases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WaifuDatabase {
    pub id: i32,
    pub hashed_secret: String,
    pub waifu_name: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub created_at: Timestamp
}
