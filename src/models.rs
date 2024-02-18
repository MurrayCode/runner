use crate::schema::records;
use crate::schema::runs;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Debug, AsChangeset, Selectable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = runs)]
pub struct NewRun {
    pub user_id: i32,
    pub distance: i32,
    pub duration: i32,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct Run {
    pub id: i32,
    pub user_id: i32,
    pub distance: i32,
    pub duration: i32,
}

#[derive(Insertable)]
#[diesel(table_name = records)]
pub struct NewRecord {
    pub user_id: i32,
    pub run_id: i32,
    pub distance: i32,
    pub duration: i32,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct Record {
    pub id: i32,
    pub user_id: i32,
    pub run_id: i32,
    pub distance: i32,
    pub duration: i32,
}
