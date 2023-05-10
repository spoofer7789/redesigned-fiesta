use actix_web::{get, HttpResponse, Responder, web, post, HttpRequest};
use crate::models::db::UserData;
use bson::doc;
use actix_web::http::header::AUTHORIZATION;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize}; // Added Serialize import
use super::login::Claims;
#[derive(Clone, Debug, Deserialize, Serialize)] // Added Serialize to the derive list
pub enum PostBody {
    File(String),
    Folder(String),
    Text(String),
}

#[derive(Debug,Clone,Deserialize, Serialize)] // Added Serialize to the derive list
pub struct Post {
    pub user: String, // Added missing field `user`
    pub title: String,
    pub body: PostBody,
    pub offer: Option<(String, i64)>,
    pub is_escrowed: Option<bool>,
    pub escrowed_by: Option<Vec<String>>,
    pub comments: Option<Vec<Comment>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)] // Added Serialize to the derive list
pub struct Comment {
    pub user: String,
    pub text: String,
}

fn validate_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret("secret_key".as_ref());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}


#[post("/u/{username}/{post}")]
pub async fn make_post(
    req: HttpRequest,
    username: web::Path<String>,
    post_data: web::Json<Post>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    let auth_header = req.headers().get(AUTHORIZATION);
    if let Some(header_value) = auth_header {
        if let Ok(token) = header_value.to_str() {
            if let Ok(claims) = validate_jwt_token(token.trim_start_matches("Bearer ")) {
                // Check if the JWT token belongs to the same user as the one making the post
                if claims.sub == username.to_string() {
                    // Continue with the post creation process
                } else {
                    return HttpResponse::Unauthorized().json("Invalid token");
                }
            } else {
                return HttpResponse::Unauthorized().json("Invalid token");
            }
        } else {
            return HttpResponse::Unauthorized().json("Invalid token");
        }
    } else {
        return HttpResponse::Unauthorized().json("Authorization header missing");
    }
    let posts_collection = db.collection("posts");

    // Check if a post with the same title already exists for the user
    let existing_post = posts_collection
        .find_one(
            doc! { "user": username.to_string(), "title": &post_data.title },

            None,
        )
        .await
        .unwrap();

    if existing_post.is_some() {
        return HttpResponse::Conflict().json("A post with this title already exists.");
    }
    // Create the new post from the input data
    let new_post = Post {
        user: username.to_string(),
        title: post_data.title.clone(),
        body: post_data.body.clone(),
        offer: post_data.offer.clone(),
        is_escrowed: post_data.is_escrowed,
        escrowed_by: post_data.escrowed_by.clone(),
        comments: post_data.comments.clone(), // Add this line
    };
    // Insert the new post into the database
    let result = posts_collection.insert_one(bson::to_document(&new_post).unwrap(), None).await;
    match result {
        Ok(_) => {
            HttpResponse::Created().json(format!("Post created with title: {}", post_data.title))
        }
        Err(_) => HttpResponse::InternalServerError().json("Post creation failed."),
    }
}

#[get("/u/{username}/{post_title}")]
pub async fn get_post(
    username: web::Path<String>,
    post_title: web::Path<String>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    let posts_collection = db.collection("posts");

    // Find the post in the database using the post_title
    let post = posts_collection
        .find_one(doc! { "user": username.to_string(), "title": post_title.to_string() },
        None)
        .await
        .unwrap();

    // If the post is found, return it as a JSON object; otherwise, return an error message
    match post {
        Some(post_doc) => {
            let post_data: Post = bson::from_document(post_doc).unwrap();
            HttpResponse::Ok().json(post_data)
        }
        None => HttpResponse::NotFound().json("Post not found."),
    }
}
