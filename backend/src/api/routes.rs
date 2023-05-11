use crate::controllers::changesettings::get_user;
use crate::models::db::register_user;
use crate::models::db::UserData;
use crate::models::login::login_user;
use crate::utils::ipfssync::web3_login;
use actix::{Actor, ActorContext, StreamHandler};
use actix_web::middleware::Condition;
use actix_web::{get, web, web::Payload, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").route("/{username}", web::get().to(get_user)), // GET /users/{username}
                                                                            //  .route("/login", web::post().to(login_user))
                                                                            // .route("/register", web::post().to(register_user)) // POST /users/register
                                                                            //  .route("/web3_login", web::post().to(web3_login))
    );
}
pub struct MyWebSocket {
    user_data: UserData,
}

impl MyWebSocket {
    pub fn new(user_data: UserData) -> Self {
        MyWebSocket { user_data }
    }
    fn handle_text_message(&self, text: &str) -> String {
        match text {
            "username" => {
                // Fetch and return the username
                format!("username:{}", self.user_data.username)
            }
            // Add more cases for other data types
            _ => {
                // Return an error response for invalid data types
                "error:Invalid data type".to_string()
            }
        }
    }
}
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let response = self.handle_text_message(&text);
                ctx.text(response);
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
#[get("/ws/")]
pub async fn ws_index(req: HttpRequest, stream: Payload) -> HttpResponse {
    let user_data = UserData::new(
        "sample_username".to_string(),
        "sample_email@example.com".to_string(),
        "sample_password".to_string(),
    );
    let res = ws::start(MyWebSocket::new(user_data), &req, stream);
    match res {
        Ok(response) => response,
        Err(e) => {
            log::error!("WebSocket start error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}