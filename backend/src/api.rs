use actix_web::{HttpResponse, Responder, get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MsgResponse {
    msg: String,
}

impl MsgResponse {
    fn new(msg_text: &str) -> MsgResponse {
        MsgResponse {
            msg: msg_text.to_string(),
        }
    }
}

#[get("/api/health")]
pub async fn health() -> impl Responder {
    let mensaje: MsgResponse = MsgResponse::new("{'Server':'True'}");
    HttpResponse::Ok().body(mensaje.msg)
}
