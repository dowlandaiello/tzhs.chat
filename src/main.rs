use std::io;
use actix_web::{HttpServer, App};
use libtz::cmd::CmdKind;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
