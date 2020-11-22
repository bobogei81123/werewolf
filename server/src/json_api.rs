use crate::server::Server;
use actix_session::UserSession;
use actix_web::{
    dev::{HttpServiceFactory, Service, ServiceRequest},
    get, post, web, HttpResponse, Responder
};
use std::future::Future;
use crate::common::*;
use crate::user::{UID, User};

#[get("/rooms")]
async fn rooms(data: web::Data<Server>) -> impl Responder {
    web::Json(data.rooms())
}

#[post("/add_room")]
async fn add_room(
    data: web::Data<Server>,
    _req: web::Json<api::AddRoom>,
    user: User,
) -> impl Responder {
    let _room_id = data.create_room(user);
    HttpResponse::Created().json(data.rooms())
}

#[post("/join_room")]
async fn join_room(
    data: web::Data<Server>,
    req: web::Json<api::JoinRoom>,
    user: User,
) -> HttpResult<impl Responder> {
    data.join_room(req.id as usize, user)?;
    Ok(HttpResponse::Ok())
}

fn check_session_id<S>(
    request: ServiceRequest,
    service: &mut S,
) -> impl Future<Output = Result<S::Response, S::Error>>
where
    S: Service<Request = ServiceRequest>,
{
    let session = request.get_session();
    if let Some(uuid) = session.get::<UID>("id").unwrap() {
        println!("Uuid = {}", uuid);
    } else {
        let uuid = UID::new_v4();
        session.set("id", uuid).unwrap();
        println!("New uuid = {}", uuid);
    }
    service.call(request)
}

#[post("/set_username")]
async fn set_username(
    data: web::Data<Server>,
    req: web::Json<api::SetUsername>,
    user: User,
) -> HttpResult<impl Responder> {
    data.join_room(req.id as usize, user)?;
    Ok(HttpResponse::Ok())
}

pub fn get_service() -> impl HttpServiceFactory + 'static {
    web::scope("/json")
        .service(rooms)
        .service(add_room)
        .service(join_room)
        .wrap_fn(check_session_id)
}
