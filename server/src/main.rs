use actix_web::{web, App, HttpServer, middleware::Logger};
use listenfd::ListenFd;
use actix_session::CookieSession;

#[doc(inline)]
pub use std;

mod app;
mod json_api;
mod server;
mod user;
mod error;
mod common;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let state = web::Data::new(server::Server::new());
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(app::get_service())
            .service(json_api::get_service())
            .wrap(Logger::default())
            .wrap({
                CookieSession::signed(&[0; 32])
                    .secure(false)
            })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8000")?
    };

    println!("Start server...");
    server.run().await
}
