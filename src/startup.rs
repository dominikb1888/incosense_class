use std::collections::HashMap;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder, Result};
use askama::Template;
use sqlx::PgPool;

use crate::routes::{health_check, stream_heartrate, stream_voltage, subscribe};

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index(_query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = Index.render().expect("template should be valid");
    Ok(web::Html::new(html))
}

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/ecg", web::get().to(stream_voltage))
            .route("/heartrate", web::get().to(stream_heartrate))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
