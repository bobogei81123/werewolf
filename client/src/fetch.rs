use anyhow::Error;
use http::{
    self,
    request::{Request, Builder},
    response::Response,
};
use std::{cell::RefCell, rc::Rc};
use yew::{
    format::{Json, Text},
    html::{Component, ComponentLink},
    services::FetchService,
};

pub trait RequestExt: Sized {
    fn json<T>(self, body: T) -> http::Result<Request<Json<T>>>;
}

impl RequestExt for Builder {
    fn json<T>(self, body: T) -> http::Result<Request<Json<T>>> {
        self.header(http::header::CONTENT_TYPE, "application/json")
            .body(Json(body))
    }
}

pub fn fetch<IN, OUT, COMP, F, M>(
    request: Request<IN>,
    link: &ComponentLink<COMP>,
    callback: F,
) -> Result<(), Error>
where
    IN: Into<Text>,
    OUT: From<Text> + 'static,
    COMP: Component,
    F: FnOnce(Response<OUT>) -> M + 'static,
    M: Into<COMP::Message>,
{
    let task_holder = Rc::new(RefCell::new(None));
    let callback = {
        let task_holder = task_holder.clone();
        move |resp| {
            let _task_holder = task_holder;
            callback(resp)
        }
    };
    let task = FetchService::fetch(request, link.callback_once(callback))?;
    *task_holder.borrow_mut() = Some(task);
    Ok(())
}
