use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub author_id: i32,
    pub created: NaiveDateTime,
    pub title: String,
    pub body: String
}

#[derive(Queryable)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String
}
