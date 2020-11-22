use crate::common::*;
use actix_session::UserSession;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future;
use uuid::Uuid;
use std::{rc::Rc, cell::RefCell};

pub type Uid = Uuid;
#[derive(PartialEq, Eq, Hash)]
pub struct Identity(pub Uid);
//{
    //uid: Uid,
    ////user: Option<Rc<RefCell<User>>>,
//}

impl FromRequest for Identity {
    type Error = HttpError;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let result = session
            .get::<Uid>("id")
            .unwrap()
            .map(Identity)
            .context("ID not set in session.")
            .map_err(|x| x.into());

        future::ready(result)
    }
}

#[derive(PartialEq, Eq)]
pub struct User {
    name: String,
    identity: Identity,
}

impl From<&User> for api::User {
    fn from(user: &User) -> Self {
        api::User { name: user.name }
    }
}
