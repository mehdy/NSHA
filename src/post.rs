use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Local>,
}

impl Post {
    pub fn new(body: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            body: body,
            created_at: Local::now(),
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

#[post("/post")]
pub async fn create(post_req: Json<CreatePostRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(post_req.to_post())
}
