use actix_web::{get, HttpResponse, Responder, web};
use crate::models::{db::UserData, posts::Post};
use bson::doc;
use super::posts::PostBody;
use serde::Deserialize;
#[get("/u/{username}")]
pub async fn get_profile(username: web::Path<String>, db: web::Data<mongodb::Database>) -> impl Responder {
    let users_collection = db.collection("users");
    // Fetch user data from the database
    let user = users_collection
        .find_one(doc! { "username": username.to_string() }, None)
        .await
        .unwrap();
    if let Some(user_doc) = user {
        let user_data: UserData = bson::from_document(user_doc).unwrap();
        // Format the user's profile page as HTML
        let profile_page = format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>{username}'s Profile</title>
                </head>
                <body>
                    <h1>{username}</h1>
                  
                    <h2>Posts</h2>
                    <ul>
                        {posts}
                    </ul>
                    <h2>Wallets</h2>
                    <ul>
                        
                    </ul>
                </body>
                </html>
            "#,
            username = user_data.username,
            posts = user_data.posts.into_iter().map(|post_doc| {
               let post: Post = bson::from_document(post_doc).unwrap(); // Removed comment
         //  let post = bson::from_document::<Post>(post_doc).unwrap();
     
                let body = match &post.body {
                    PostBody::File(file) => format!("File: {}", file),
                    PostBody::Folder(folder) => format!("Folder: {}", folder),
                    PostBody::Text(text) => text.clone(),
                };
                  //  format!("<li>Title: {}<br>Body: {}<br>Offer: {} - {}<br>Is Escrowed: {}<br>Escrowed By: {:?}</li>", post.title, body, post.offer.as_ref().map(|x| &x.0).unwrap_or(&"".to_string()), post.offer.map(|x| x.1).unwrap_or(0), post.is_escrowed.unwrap_or(false), post.escrowed_by)
                  format!(
                    "<li>Title: {}<br>Body: {}<br>Offer: {} - {}<br>Is Escrowed: {}<br>Escrowed By: {:?}</li>",
                    post.title,
                    body,
                    post.offer.as_ref().map(|x| x.0.clone()).unwrap_or_else(|| "".to_string()), // Clone the value
                    post.offer.as_ref().map(|x| x.1).unwrap_or(0), // Use as_ref() instead of a move
                    post.is_escrowed.unwrap_or(false),
                    post.escrowed_by
                )
                
                }).collect::<String>(),
        );

        HttpResponse::Ok().content_type("text/html").body(profile_page)
    } else {
        HttpResponse::NotFound().body("User not found.")
    }
}