use crate::schema::posts;
use crate::{DBPool, DBPC};
use actix_web::web::{block, Data, Json};
use actix_web::{post, HttpResponse};
use chrono::{Local, NaiveDateTime};
use diesel::result::Error;
use diesel::{Insertable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const APPLICATION_JSON: &str = "application/json";

#[table_name = "posts"]
#[derive(Debug, Deserialize, Serialize, Insertable)]
pub struct Post {
    pub id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
}

impl Post {
    pub fn new(body: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            body: body,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub body: String,
}

impl CreatePostRequest {
    pub fn to_post(&self) -> Post {
        Post::new(self.body.to_string())
    }
}

fn create_post(post: Post, conn: &DBPC) -> Result<Post, Error> {
    use crate::schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values(&post)
        .execute(conn)
        .and(Ok(post))
}

#[post("/post")]
pub async fn create(post_req: Json<CreatePostRequest>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect("Failed to get db connection");

    let post = block(move || create_post(post_req.to_post(), &conn)).await;

    match post {
        Ok(post) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(post),
        _ => HttpResponse::InternalServerError().await.unwrap(),
    }
}
